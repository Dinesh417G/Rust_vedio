use rustclass_core::assets::CancelFlag;
use rustclass_core::paths::Paths;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex;

/// One in-flight import.
pub struct ImportJob {
    pub block_id: Option<String>,
    pub cancel: CancelFlag,
}

/// Everything the command layer shares.
///
/// The single `Connection` lives behind an async mutex so a long chapter read
/// cannot be interleaved with a write; the import registry uses a plain mutex
/// because it is only ever touched for a few microseconds at a time.
pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub paths: Paths,
    pub imports: StdMutex<HashMap<String, ImportJob>>,
    pub app_version: String,
}

impl AppState {
    pub fn new(conn: rusqlite::Connection, paths: Paths, app_version: String) -> Self {
        Self {
            db: Mutex::new(conn),
            paths,
            imports: StdMutex::new(HashMap::new()),
            app_version,
        }
    }

    /// Blocks with an import in flight — feeds `BlockState::Importing`.
    pub fn importing_blocks(&self) -> HashSet<String> {
        self.imports
            .lock()
            .map(|m| m.values().filter_map(|j| j.block_id.clone()).collect())
            .unwrap_or_default()
    }

    pub fn register_import(&self, ticket: String, block_id: Option<String>) -> CancelFlag {
        let cancel = CancelFlag::new();
        if let Ok(mut m) = self.imports.lock() {
            m.insert(
                ticket,
                ImportJob {
                    block_id,
                    cancel: cancel.clone(),
                },
            );
        }
        cancel
    }

    pub fn finish_import(&self, ticket: &str) {
        if let Ok(mut m) = self.imports.lock() {
            m.remove(ticket);
        }
    }

    pub fn cancel_import(&self, ticket: &str) -> bool {
        self.imports
            .lock()
            .ok()
            .and_then(|m| m.get(ticket).map(|j| j.cancel.cancel()))
            .is_some()
    }
}
