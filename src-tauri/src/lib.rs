mod commands;
mod error;
mod state;

use rustclass_core::paths::Paths;
use state::AppState;
use tauri::Manager;
use tracing_subscriber::prelude::*;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(all(desktop, not(any(target_os = "android", target_os = "ios"))))]
    {
        // A second launch focuses the window we already have rather than
        // opening a second one onto the same database.
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }));
    }

    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .setup(|app| {
            let app_data = app.path().app_data_dir()?;
            let paths = Paths::new(app_data);
            paths.ensure()?;
            init_logging(&paths);

            tracing::info!(version = APP_VERSION, dir = ?paths.app_data, "RustClass starting");

            // A crash mid-import leaves `.part` files; clear them before the
            // integrity check runs so they are not reported as real problems.
            if let Err(e) = rustclass_core::gc::sweep_partials(&paths) {
                tracing::warn!(error = %e, "could not sweep partial imports");
            }

            let conn = rustclass_core::db::open(&paths.db_file())?;
            // First run seeds the course row so the UI always has something.
            rustclass_core::repo::course::get_or_seed(&conn)?;

            app.manage(AppState::new(conn, paths, APP_VERSION.to_string()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::course::course_get,
            commands::course::course_update,
            commands::chapter::chapter_list,
            commands::chapter::chapter_get,
            commands::chapter::chapter_create,
            commands::chapter::chapter_update,
            commands::chapter::chapter_reorder,
            commands::chapter::chapter_duplicate,
            commands::chapter::chapter_delete,
            commands::chapter::chapter_restore,
            commands::chapter::chapter_purge,
            commands::block::block_list,
            commands::block::block_get,
            commands::block::block_create,
            commands::block::block_update,
            commands::block::block_reorder,
            commands::block::block_convert,
            commands::block::block_convert_preview,
            commands::block::block_delete,
            commands::block::block_restore,
            commands::block::block_purge,
            commands::block::trash_list,
            commands::block::marker_create,
            commands::block::marker_delete,
            commands::block::note_list,
            commands::block::note_create,
            commands::block::note_update,
            commands::block::note_delete,
            commands::asset::asset_import,
            commands::asset::asset_import_cancel,
            commands::asset::asset_attach,
            commands::asset::asset_detach,
            commands::asset::asset_src,
            commands::asset::asset_get,
            commands::asset::asset_relink,
            commands::asset::asset_reveal,
            commands::asset::asset_open_external,
            commands::asset::asset_set_media_meta,
            commands::asset::asset_set_thumbnail,
            commands::asset::asset_gc,
            commands::progress::progress_set,
            commands::progress::progress_set_chapter,
            commands::progress::progress_course,
            commands::outline::outline_preview,
            commands::outline::outline_import_markdown,
            commands::outline::search_query,
            commands::bundle::bundle_export,
            commands::bundle::bundle_import,
            commands::system::system_info,
            commands::system::integrity_check,
            commands::system::trash_sweep,
            commands::system::open_path,
            commands::system::reveal_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running RustClass");
}

/// Rolling daily log file next to the database, plus stderr in a dev build.
fn init_logging(paths: &Paths) {
    let appender = tracing_appender::rolling::daily(paths.logs_root(), "rustclass.log");
    let filter = tracing_subscriber::EnvFilter::try_from_env("RUSTCLASS_LOG")
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(appender)
        .with_ansi(false);
    let registry = tracing_subscriber::registry().with(filter).with(file_layer);
    #[cfg(debug_assertions)]
    let registry = registry.with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr));
    if registry.try_init().is_err() {
        // A second init only happens under test harnesses; not worth failing over.
        eprintln!("logging was already initialised");
    }
}
