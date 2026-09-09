use crate::error::CmdResult;
use crate::state::AppState;
use rustclass_core::models::*;
use rustclass_core::repo::{chapter, ReadCtx};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn chapter_list(
    state: State<'_, AppState>,
    include_deleted: bool,
) -> CmdResult<Vec<ChapterSummary>> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    Ok(chapter::list(&conn, &ctx, include_deleted)?)
}

#[tauri::command]
pub async fn chapter_get(state: State<'_, AppState>, id: String) -> CmdResult<ChapterView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    Ok(chapter::get(&conn, &ctx, &id)?)
}

#[tauri::command]
pub async fn chapter_create(
    app: AppHandle,
    state: State<'_, AppState>,
    req: NewChapter,
) -> CmdResult<ChapterView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let mut conn = state.db.lock().await;
    let out = chapter::create(&mut conn, &ctx, req)?;
    super::notify(&app, "chapter", Some(out.chapter.id.clone()));
    Ok(out)
}

#[tauri::command]
pub async fn chapter_update(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    patch: ChapterPatch,
) -> CmdResult<ChapterView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let conn = state.db.lock().await;
    let out = chapter::update(&conn, &ctx, &id, patch)?;
    super::notify(&app, "chapter", Some(id));
    Ok(out)
}

#[tauri::command]
pub async fn chapter_reorder(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    after_id: Option<String>,
) -> CmdResult<()> {
    let mut conn = state.db.lock().await;
    chapter::reorder(&mut conn, &id, after_id.as_deref())?;
    super::notify(&app, "chapter", Some(id));
    Ok(())
}

#[tauri::command]
pub async fn chapter_duplicate(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<ChapterView> {
    let importing = state.importing_blocks();
    let ctx = ReadCtx::new(&state.paths, &importing);
    let mut conn = state.db.lock().await;
    let out = chapter::duplicate(&mut conn, &ctx, &id)?;
    super::notify(&app, "chapter", Some(out.chapter.id.clone()));
    Ok(out)
}

#[tauri::command]
pub async fn chapter_delete(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<()> {
    let conn = state.db.lock().await;
    chapter::soft_delete(&conn, &id)?;
    super::notify(&app, "chapter", Some(id));
    Ok(())
}

#[tauri::command]
pub async fn chapter_restore(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<()> {
    let conn = state.db.lock().await;
    chapter::restore(&conn, &id)?;
    super::notify(&app, "chapter", Some(id));
    Ok(())
}

/// Permanent. Blocks cascade; asset files are untouched — only `asset_gc`
/// deletes bytes.
#[tauri::command]
pub async fn chapter_purge(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<()> {
    let conn = state.db.lock().await;
    chapter::purge(&conn, &id)?;
    super::notify(&app, "chapter", Some(id));
    Ok(())
}
