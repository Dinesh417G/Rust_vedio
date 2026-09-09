use crate::error::{AppError, CmdResult};
use crate::state::AppState;
use rustclass_core::models::IntegrityIssue;
use rustclass_core::{gc, integrity, SystemInfo};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn system_info(state: State<'_, AppState>) -> CmdResult<SystemInfo> {
    let conn = state.db.lock().await;
    Ok(rustclass_core::system_info(
        &conn,
        &state.paths,
        &state.app_version,
    )?)
}

#[tauri::command]
pub async fn integrity_check(state: State<'_, AppState>) -> CmdResult<Vec<IntegrityIssue>> {
    let conn = state.db.lock().await;
    Ok(integrity::check(&conn, &state.paths)?)
}

/// Empties the Trash of anything past the 30-day window. Called at startup.
#[tauri::command]
pub async fn trash_sweep(state: State<'_, AppState>) -> CmdResult<u32> {
    use rustclass_core::repo::{block, chapter};
    let conn = state.db.lock().await;
    let (chapters, blocks) = gc::expired_trash(&conn, gc::TRASH_RETENTION_MS)?;
    let n = (chapters.len() + blocks.len()) as u32;
    for id in chapters {
        chapter::purge(&conn, &id)?;
    }
    for id in blocks {
        block::purge(&conn, &id)?;
    }
    Ok(n)
}

#[tauri::command]
pub async fn open_path(app: AppHandle, path: String) -> CmdResult<()> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| AppError::new("io", e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn reveal_path(app: AppHandle, path: String) -> CmdResult<()> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .reveal_item_in_dir(std::path::PathBuf::from(path))
        .map_err(|e| AppError::new("io", e.to_string()))?;
    Ok(())
}
