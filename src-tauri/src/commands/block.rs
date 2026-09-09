use crate::error::CmdResult;
use crate::state::AppState;
use rustclass_core::models::*;
use rustclass_core::repo::{block, marker, note, ReadCtx};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn block_list(
    state: State<'_, AppState>,
    chapter_id: String,
) -> CmdResult<Vec<BlockView>> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    Ok(block::list(&conn, &ctx, &chapter_id)?)
}

#[tauri::command]
pub async fn block_get(state: State<'_, AppState>, id: String) -> CmdResult<BlockView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    Ok(block::get(&conn, &ctx, &id)?)
}

#[tauri::command]
pub async fn block_create(
    app: AppHandle,
    state: State<'_, AppState>,
    chapter_id: String,
    kind: BlockKind,
    after_block_id: Option<String>,
    seed: Option<BlockSeed>,
) -> CmdResult<BlockView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let mut conn = state.db.lock().await;
    let out = block::create(
        &mut conn,
        &ctx,
        &chapter_id,
        kind,
        after_block_id.as_deref(),
        seed,
    )?;
    super::notify(&app, "block", Some(chapter_id));
    Ok(out)
}

#[tauri::command]
pub async fn block_update(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    patch: BlockPatch,
) -> CmdResult<BlockView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    let out = block::update(&conn, &ctx, &id, patch)?;
    super::notify(&app, "block", Some(out.block.chapter_id.clone()));
    Ok(out)
}

/// Supports moving a block to a different chapter, which is what makes
/// cross-chapter drag work.
#[tauri::command]
pub async fn block_reorder(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    chapter_id: String,
    after_block_id: Option<String>,
) -> CmdResult<()> {
    let mut conn = state.db.lock().await;
    block::reorder(&mut conn, &id, &chapter_id, after_block_id.as_deref())?;
    super::notify(&app, "block", Some(chapter_id));
    Ok(())
}

/// What a conversion would throw away, so the UI can warn first.
#[tauri::command]
pub async fn block_convert_preview(
    state: State<'_, AppState>,
    id: String,
    new_kind: BlockKind,
) -> CmdResult<Vec<String>> {
    let conn = state.db.lock().await;
    let b = block::get_row(&conn, &id)?;
    Ok(block::convert_losses(&b, new_kind)
        .into_iter()
        .map(String::from)
        .collect())
}

#[tauri::command]
pub async fn block_convert(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    new_kind: BlockKind,
) -> CmdResult<BlockView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    let out = block::convert(&conn, &ctx, &id, new_kind)?;
    super::notify(&app, "block", Some(out.block.chapter_id.clone()));
    Ok(out)
}

#[tauri::command]
pub async fn block_delete(app: AppHandle, state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().await;
    let chapter_id = block::get_row(&conn, &id)?.chapter_id;
    block::soft_delete(&conn, &id)?;
    super::notify(&app, "block", Some(chapter_id));
    Ok(())
}

#[tauri::command]
pub async fn block_restore(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<()> {
    let conn = state.db.lock().await;
    block::restore(&conn, &id)?;
    let chapter_id = block::get_row(&conn, &id)?.chapter_id;
    super::notify(&app, "block", Some(chapter_id));
    Ok(())
}

#[tauri::command]
pub async fn block_purge(app: AppHandle, state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().await;
    let chapter_id = block::get_row(&conn, &id).map(|b| b.chapter_id).ok();
    block::purge(&conn, &id)?;
    super::notify(&app, "block", chapter_id);
    Ok(())
}

/// Everything currently in the Trash, with the age the UI shows.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrashView {
    pub chapters: Vec<ChapterSummary>,
    pub blocks: Vec<TrashedBlock>,
    pub retention_days: i64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrashedBlock {
    #[serde(flatten)]
    pub block: Block,
    pub chapter_title: String,
}

#[tauri::command]
pub async fn trash_list(state: State<'_, AppState>) -> CmdResult<TrashView> {
    use rustclass_core::repo::chapter as chapter_repo;
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    let all = chapter_repo::list(&conn, &ctx, true)?;
    let chapters = all
        .into_iter()
        .filter(|c| c.chapter.deleted_at.is_some())
        .collect();
    let mut blocks = Vec::new();
    for b in block::list_deleted(&conn)? {
        let chapter_title = chapter_repo::get_row(&conn, &b.chapter_id)
            .map(|c| c.title)
            .unwrap_or_else(|_| "(deleted chapter)".to_string());
        blocks.push(TrashedBlock {
            block: b,
            chapter_title,
        });
    }
    Ok(TrashView {
        chapters,
        blocks,
        retention_days: rustclass_core::gc::TRASH_RETENTION_MS / (24 * 60 * 60 * 1000),
    })
}

// --- markers and notes ------------------------------------------------------

#[tauri::command]
pub async fn marker_create(
    app: AppHandle,
    state: State<'_, AppState>,
    block_id: String,
    at_secs: f64,
    label: String,
    note_md: Option<String>,
) -> CmdResult<Marker> {
    let conn = state.db.lock().await;
    let out = marker::create(&conn, &block_id, at_secs, &label, note_md.as_deref())?;
    super::notify(&app, "marker", Some(block_id));
    Ok(out)
}

#[tauri::command]
pub async fn marker_delete(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<()> {
    let conn = state.db.lock().await;
    marker::delete(&conn, &id)?;
    super::notify(&app, "marker", None);
    Ok(())
}

#[tauri::command]
pub async fn note_list(state: State<'_, AppState>, chapter_id: String) -> CmdResult<Vec<Note>> {
    let conn = state.db.lock().await;
    Ok(note::list(&conn, &chapter_id)?)
}

#[tauri::command]
pub async fn note_create(
    app: AppHandle,
    state: State<'_, AppState>,
    chapter_id: String,
    body_md: String,
) -> CmdResult<Note> {
    let conn = state.db.lock().await;
    let out = note::create(&conn, &chapter_id, &body_md)?;
    super::notify(&app, "note", Some(chapter_id));
    Ok(out)
}

#[tauri::command]
pub async fn note_update(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    body_md: String,
) -> CmdResult<Note> {
    let conn = state.db.lock().await;
    let out = note::update(&conn, &id, &body_md)?;
    super::notify(&app, "note", Some(out.chapter_id.clone()));
    Ok(out)
}

#[tauri::command]
pub async fn note_delete(app: AppHandle, state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().await;
    note::delete(&conn, &id)?;
    super::notify(&app, "note", None);
    Ok(())
}
