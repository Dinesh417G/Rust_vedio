use crate::error::{AppError, CmdResult};
use crate::state::AppState;
use rustclass_core::bundle::{self, BundleImportReport, BundleMode, ExportReport};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn bundle_export(
    state: State<'_, AppState>,
    dest_path: String,
    include_media: bool,
) -> CmdResult<ExportReport> {
    let conn = state.db.lock().await;
    let dest = std::path::PathBuf::from(&dest_path);
    if dest.as_os_str().is_empty() {
        return Err(AppError::new("invalid", "Choose where to save the bundle."));
    }
    Ok(bundle::export(
        &conn,
        &state.paths,
        &dest,
        include_media,
        &state.app_version,
    )?)
}

#[tauri::command]
pub async fn bundle_import(
    app: AppHandle,
    state: State<'_, AppState>,
    src_path: String,
    mode: BundleMode,
) -> CmdResult<BundleImportReport> {
    let mut conn = state.db.lock().await;
    let report = bundle::import(
        &mut conn,
        &state.paths,
        std::path::Path::new(&src_path),
        mode,
    )?;
    super::notify(&app, "course", None);
    super::notify(&app, "chapter", None);
    super::notify(&app, "block", None);
    Ok(report)
}
