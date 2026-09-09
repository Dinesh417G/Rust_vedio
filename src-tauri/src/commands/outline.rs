use crate::error::CmdResult;
use crate::state::AppState;
use rustclass_core::models::SearchHit;
use rustclass_core::outline::{self, ImportReport, OutlineMode, ParsedOutline};
use rustclass_core::repo::search;
use tauri::{AppHandle, State};

/// Parses without writing anything — this is what the preview diff renders.
#[tauri::command]
pub async fn outline_preview(md: String) -> CmdResult<ParsedOutline> {
    Ok(outline::parse(&md))
}

#[tauri::command]
pub async fn outline_import_markdown(
    app: AppHandle,
    state: State<'_, AppState>,
    md: String,
    mode: OutlineMode,
) -> CmdResult<ImportReport> {
    let parsed = outline::parse(&md);
    let mut conn = state.db.lock().await;
    let report = rustclass_core::apply_outline(&mut conn, &parsed, mode)?;
    super::notify(&app, "chapter", None);
    super::notify(&app, "course", None);
    Ok(report)
}

#[tauri::command]
pub async fn search_query(
    state: State<'_, AppState>,
    q: String,
    limit: u32,
) -> CmdResult<Vec<SearchHit>> {
    let conn = state.db.lock().await;
    Ok(search::query(&conn, &q, limit.clamp(1, 200))?)
}
