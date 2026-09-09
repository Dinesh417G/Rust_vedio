use crate::error::CmdResult;
use crate::state::AppState;
use rustclass_core::models::{CourseProgress, ProgressPatch, ProgressView};
use rustclass_core::repo::{chapter, progress};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn progress_set(
    app: AppHandle,
    state: State<'_, AppState>,
    block_id: String,
    patch: ProgressPatch,
) -> CmdResult<ProgressView> {
    let conn = state.db.lock().await;
    let out = progress::set(&conn, &block_id, patch)?;
    super::notify(&app, "progress", Some(block_id));
    Ok(out)
}

#[tauri::command]
pub async fn progress_set_chapter(
    app: AppHandle,
    state: State<'_, AppState>,
    chapter_id: String,
    completed: bool,
) -> CmdResult<u32> {
    let conn = state.db.lock().await;
    let n = progress::set_chapter(&conn, &chapter_id, completed)?;
    super::notify(&app, "progress", Some(chapter_id));
    Ok(n)
}

#[tauri::command]
pub async fn progress_course(state: State<'_, AppState>) -> CmdResult<CourseProgress> {
    let conn = state.db.lock().await;
    Ok(chapter::progress(&conn)?)
}
