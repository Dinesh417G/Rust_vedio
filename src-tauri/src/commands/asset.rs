use crate::error::{AppError, CmdResult};
use crate::state::AppState;
use rustclass_core::assets::{self, CancelFlag, ImportProgress, ImportRequest, ImportTicket};
use rustclass_core::ids::new_id;
use rustclass_core::models::*;
use rustclass_core::repo::{asset as asset_repo, block as block_repo, ReadCtx};
use rustclass_core::{gc, paths};
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportDone {
    ticket_id: String,
    block_id: Option<String>,
    asset_id: String,
    deduped: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportFailed {
    ticket_id: String,
    block_id: Option<String>,
    code: String,
    message: String,
}

/// Starts an import and returns immediately. Everything after this point is
/// reported over `import://progress`, `import://done` and `import://failed`.
///
/// The database lock is taken only for the dedupe probe and the final row
/// insert — never across the copy, so the UI keeps responding while a 2 GB
/// file streams in.
#[tauri::command]
pub async fn asset_import(
    app: AppHandle,
    state: State<'_, AppState>,
    req: ImportRequest,
) -> CmdResult<ImportTicket> {
    let src = std::path::PathBuf::from(&req.path);
    if !src.is_file() {
        return Err(AppError::new(
            "invalid",
            format!("`{}` is not a readable file.", req.path),
        ));
    }
    let bytes_total = src.metadata().map(|m| m.len() as i64).unwrap_or(0);
    let original_name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "file".to_string());

    if let Some(block_id) = &req.block_id {
        let conn = state.db.lock().await;
        let block = block_repo::get_row(&conn, block_id)?;
        if !block.kind.is_asset_backed() {
            return Err(AppError::new(
                "invalid",
                format!("A {} block does not hold a file.", block.kind.as_str()),
            ));
        }
    }

    let ticket_id = new_id();
    let cancel = state.register_import(ticket_id.clone(), req.block_id.clone());
    let ticket = ImportTicket {
        ticket_id: ticket_id.clone(),
        block_id: req.block_id.clone(),
        original_name,
        bytes_total,
        deduped_asset_id: None,
    };

    // The block's state is computed from the in-flight import registry, so the
    // frontend has to re-read it now to see `Importing`. Without this the
    // placeholder sits there looking untouched until the copy finishes.
    super::notify(&app, "block", req.block_id.clone());

    let app2 = app.clone();
    tauri::async_runtime::spawn(async move {
        let result = run_import(&app2, req.clone(), ticket_id.clone(), cancel).await;
        let state = app2.state::<AppState>();
        state.finish_import(&ticket_id);
        match result {
            Ok((asset_id, deduped)) => {
                let _ = app2.emit(
                    super::EV_IMPORT_DONE,
                    ImportDone {
                        ticket_id,
                        block_id: req.block_id.clone(),
                        asset_id,
                        deduped,
                    },
                );
                super::notify(&app2, "asset", req.block_id.clone());
                super::notify(&app2, "block", None);
            }
            Err(e) => {
                tracing::warn!(code = %e.code, error = %e.message, "import failed");
                let _ = app2.emit(
                    super::EV_IMPORT_FAILED,
                    ImportFailed {
                        ticket_id,
                        block_id: req.block_id.clone(),
                        code: e.code,
                        message: e.message,
                    },
                );
                super::notify(&app2, "block", None);
            }
        }
    });

    Ok(ticket)
}

async fn run_import(
    app: &AppHandle,
    req: ImportRequest,
    ticket_id: String,
    cancel: CancelFlag,
) -> Result<(String, bool), AppError> {
    // --- phase 1: stat only. Instant, no lock, no read. ---------------------
    let probe = assets::probe(&req)?;
    emit_progress(app, &ticket_id, &req.block_id, None, 0, probe.bytes);

    let paths = {
        let state = app.state::<AppState>();
        state.paths.clone()
    };

    // --- phase 2: dedupe, but only when it is worth the read ----------------
    // A pre-hash costs a full pass over the file. Byte length is free and
    // rules out a duplicate outright, so the full hash only runs when some
    // existing asset is exactly this size.
    let same_size = {
        let state = app.state::<AppState>();
        let conn = state.db.lock().await;
        asset_repo::any_with_size(&conn, probe.bytes)?
    };

    if same_size {
        let sha = {
            let app = app.clone();
            let ticket = ticket_id.clone();
            let block_id = req.block_id.clone();
            let src = probe.src.clone();
            let cancel = cancel.clone();
            tauri::async_runtime::spawn_blocking(move || {
                assets::hash_file(&src, &cancel, |done, total| {
                    emit_progress(&app, &ticket, &block_id, None, done, total);
                })
            })
            .await
            .map_err(|e| AppError::new("internal", e.to_string()))??
        };
        let state = app.state::<AppState>();
        let conn = state.db.lock().await;
        if let Some(existing) = asset_repo::find_by_hash(&conn, &sha.0, sha.1)? {
            emit_progress(
                app,
                &ticket_id,
                &req.block_id,
                Some(&existing.id),
                probe.bytes,
                probe.bytes,
            );
            attach(&conn, &state, &req, &existing.id)?;
            return Ok((existing.id, true));
        }
    }

    // --- phase 3: copy and hash in one pass, with no lock held --------------
    let placement = {
        let app = app.clone();
        let ticket = ticket_id.clone();
        let block_id = req.block_id.clone();
        let probe = probe.clone();
        let mode = req.mode;
        let cancel = cancel.clone();
        let paths = paths.clone();
        tauri::async_runtime::spawn_blocking(move || {
            assets::place(&paths, &probe, mode, &cancel, |done, total| {
                emit_progress(&app, &ticket, &block_id, None, done, total);
            })
        })
        .await
        .map_err(|e| AppError::new("internal", e.to_string()))??
    };

    // --- phase 4: record the row and attach it ------------------------------
    let state = app.state::<AppState>();
    let conn = state.db.lock().await;

    // A same-size asset can appear between the check above and here, and a
    // reference import never took the pre-hash path at all.
    if let Some(existing) = asset_repo::find_by_hash(&conn, &placement.sha256, placement.bytes)? {
        assets::discard(&paths, &placement, req.mode)?;
        attach(&conn, &state, &req, &existing.id)?;
        return Ok((existing.id, true));
    }

    assets::record(&conn, &probe, &placement, req.mode)?;
    attach(&conn, &state, &req, &placement.asset_id)?;
    Ok((placement.asset_id, false))
}

/// Points the requesting block at the asset the import landed on.
fn attach(
    conn: &rusqlite::Connection,
    state: &tauri::State<'_, AppState>,
    req: &ImportRequest,
    asset_id: &str,
) -> Result<(), AppError> {
    let Some(block_id) = &req.block_id else {
        return Ok(());
    };
    let importing = std::collections::HashSet::new();
    let ctx = ReadCtx::new(&state.paths, &importing);
    block_repo::set_asset(conn, &ctx, block_id, Some(asset_id))?;
    Ok(())
}

fn emit_progress(
    app: &AppHandle,
    ticket_id: &str,
    block_id: &Option<String>,
    asset_id: Option<&str>,
    done: i64,
    total: i64,
) {
    let pct = if total > 0 {
        (done as f64 * 100.0 / total as f64).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let _ = app.emit(
        super::EV_IMPORT_PROGRESS,
        ImportProgress {
            ticket_id: ticket_id.to_string(),
            block_id: block_id.clone(),
            asset_id: asset_id.map(String::from),
            bytes_done: done,
            bytes_total: total,
            pct,
        },
    );
}

#[tauri::command]
pub async fn asset_import_cancel(state: State<'_, AppState>, ticket_id: String) -> CmdResult<()> {
    if !state.cancel_import(&ticket_id) {
        // Already finished — nothing to cancel, and nothing worth erroring over.
        tracing::debug!(ticket_id, "cancel for an import that already ended");
    }
    Ok(())
}

#[tauri::command]
pub async fn asset_attach(
    app: AppHandle,
    state: State<'_, AppState>,
    block_id: String,
    asset_id: String,
) -> CmdResult<BlockView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    let out = block_repo::set_asset(&conn, &ctx, &block_id, Some(&asset_id))?;
    super::notify(&app, "block", Some(out.block.chapter_id.clone()));
    Ok(out)
}

/// The block returns to `Empty`; the asset row and its file are kept.
#[tauri::command]
pub async fn asset_detach(
    app: AppHandle,
    state: State<'_, AppState>,
    block_id: String,
) -> CmdResult<BlockView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    let out = block_repo::set_asset(&conn, &ctx, &block_id, None)?;
    super::notify(&app, "block", Some(out.block.chapter_id.clone()));
    Ok(out)
}

/// Absolute path for `convertFileSrc()`. Media is never base64'd into the DOM;
/// the asset protocol serves it with Range support so seeking works.
#[tauri::command]
pub async fn asset_src(state: State<'_, AppState>, asset_id: String) -> CmdResult<String> {
    let conn = state.db.lock().await;
    let row = asset_repo::get_row(&conn, &asset_id)?;
    Ok(paths::display_path(&row.absolute(&state.paths)))
}

#[tauri::command]
pub async fn asset_get(state: State<'_, AppState>, asset_id: String) -> CmdResult<AssetView> {
    let conn = state.db.lock().await;
    Ok(asset_repo::get(&conn, &state.paths, &asset_id)?)
}

#[tauri::command]
pub async fn asset_relink(
    app: AppHandle,
    state: State<'_, AppState>,
    asset_id: String,
    new_path: String,
) -> CmdResult<AssetView> {
    let conn = state.db.lock().await;
    let out = asset_repo::relink(&conn, &state.paths, &asset_id, &new_path)?;
    super::notify(&app, "asset", Some(asset_id));
    super::notify(&app, "block", None);
    Ok(out)
}

#[tauri::command]
pub async fn asset_reveal(
    app: AppHandle,
    state: State<'_, AppState>,
    asset_id: String,
) -> CmdResult<()> {
    use tauri_plugin_opener::OpenerExt;
    let path = {
        let conn = state.db.lock().await;
        let row = asset_repo::get_row(&conn, &asset_id)?;
        row.absolute(&state.paths)
    };
    app.opener()
        .reveal_item_in_dir(&path)
        .map_err(|e| AppError::new("io", e.to_string()))?;
    Ok(())
}

/// Opens the file in whatever the OS considers its default player — the escape
/// hatch when WebView2 cannot decode the codec.
#[tauri::command]
pub async fn asset_open_external(
    app: AppHandle,
    state: State<'_, AppState>,
    asset_id: String,
) -> CmdResult<()> {
    use tauri_plugin_opener::OpenerExt;
    let path = {
        let conn = state.db.lock().await;
        let row = asset_repo::get_row(&conn, &asset_id)?;
        row.absolute(&state.paths)
    };
    app.opener()
        .open_path(path.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| AppError::new("io", e.to_string()))?;
    Ok(())
}

/// Duration and dimensions measured by the webview — there is no ffmpeg here
/// and there is not going to be one.
#[tauri::command]
pub async fn asset_set_media_meta(
    app: AppHandle,
    state: State<'_, AppState>,
    asset_id: String,
    duration_secs: Option<f64>,
    width: Option<i64>,
    height: Option<i64>,
) -> CmdResult<()> {
    let conn = state.db.lock().await;
    asset_repo::set_media_meta(&conn, &asset_id, duration_secs, width, height)?;
    super::notify(&app, "asset", Some(asset_id));
    Ok(())
}

/// A frame grabbed to a `<canvas>` and handed back as base64 JPEG.
#[tauri::command]
pub async fn asset_set_thumbnail(
    app: AppHandle,
    state: State<'_, AppState>,
    asset_id: String,
    png_base64: String,
) -> CmdResult<String> {
    let conn = state.db.lock().await;
    let rel = assets::write_thumbnail(&conn, &state.paths, &asset_id, &png_base64)?;
    super::notify(&app, "asset", Some(asset_id));
    Ok(rel)
}

#[tauri::command]
pub async fn asset_gc(state: State<'_, AppState>, dry_run: bool) -> CmdResult<GcReport> {
    let conn = state.db.lock().await;
    Ok(gc::run(&conn, &state.paths, dry_run)?)
}
