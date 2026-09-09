pub mod asset;
pub mod block;
pub mod bundle;
pub mod chapter;
pub mod course;
pub mod outline;
pub mod progress;
pub mod system;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

/// Emitted after every mutation so the frontend can invalidate exactly the
/// TanStack Query keys that changed instead of refetching the world.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataChanged {
    pub entity: &'static str,
    pub id: Option<String>,
}

pub const EV_DATA_CHANGED: &str = "data://changed";
pub const EV_IMPORT_PROGRESS: &str = "import://progress";
pub const EV_IMPORT_DONE: &str = "import://done";
pub const EV_IMPORT_FAILED: &str = "import://failed";

pub fn notify(app: &AppHandle, entity: &'static str, id: Option<String>) {
    if let Err(e) = app.emit(EV_DATA_CHANGED, DataChanged { entity, id }) {
        tracing::warn!(error = %e, entity, "could not emit data://changed");
    }
}
