use crate::error::CmdResult;
use crate::state::AppState;
use rustclass_library::{LoadMode, LoadReport};
use tauri::{AppHandle, State};

/// Loads the teaching library that is baked into the app (see
/// `crates/library/build.rs`) into the course. `Replace` moves every live
/// chapter to the trash first; `Append` adds the library after them.
#[tauri::command]
pub async fn library_load_builtin(
    app: AppHandle,
    state: State<'_, AppState>,
    mode: LoadMode,
) -> CmdResult<LoadReport> {
    let lib = rustclass_library::parse_files(&rustclass_library::builtin())
        .map_err(|errors| rustclass_library::errors_to_core(&errors))?;
    let mut conn = state.db.lock().await;
    let report = rustclass_library::load(&mut conn, &lib, mode)?;
    super::notify(&app, "chapter", None);
    super::notify(&app, "course", None);
    Ok(report)
}
