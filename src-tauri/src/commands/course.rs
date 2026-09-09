use crate::error::CmdResult;
use crate::state::AppState;
use rustclass_core::models::{CoursePatch, CourseView};
use rustclass_core::repo::course;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn course_get(state: State<'_, AppState>) -> CmdResult<CourseView> {
    let conn = state.db.lock().await;
    Ok(course::get_or_seed(&conn)?)
}

#[tauri::command]
pub async fn course_update(
    app: AppHandle,
    state: State<'_, AppState>,
    patch: CoursePatch,
) -> CmdResult<CourseView> {
    let conn = state.db.lock().await;
    let out = course::update(&conn, patch)?;
    super::notify(&app, "course", Some(out.id.clone()));
    Ok(out)
}
