use serde::Serialize;

/// The wire shape of every command failure: `{ code, message, detail }`.
///
/// Command handlers never panic — they return this instead, so a bad path or a
/// yanked USB drive shows a sentence in the UI rather than killing the webview.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,
    pub detail: Option<String>,
}

impl AppError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

impl From<rustclass_core::CoreError> for AppError {
    fn from(e: rustclass_core::CoreError) -> Self {
        use rustclass_core::CoreError as C;
        let code = e.code().to_string();
        // A storage or IO failure is never the user's fault and never
        // self-explanatory, so the raw cause rides along for the /health page
        // and the log file. Everything else already reads as a sentence.
        let detail = match &e {
            C::Db(_) | C::Io(_) | C::Serde(_) | C::Zip(_) | C::Internal(_) => {
                Some(format!("{e:?}"))
            }
            _ => None,
        };
        let message = e.to_string();
        tracing::warn!(code = %code, error = %message, "command failed");
        AppError {
            code,
            message,
            detail,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::new("io", e.to_string()).with_detail(format!("{:?}", e.kind()))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::new("serde", e.to_string())
    }
}

impl From<tauri::Error> for AppError {
    fn from(e: tauri::Error) -> Self {
        AppError::new("internal", e.to_string())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

pub type CmdResult<T> = std::result::Result<T, AppError>;
