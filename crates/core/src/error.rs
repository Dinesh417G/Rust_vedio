use std::fmt;

/// The single error type crossing the IPC boundary.
///
/// Serialises to `{ code, message, detail }` so the frontend can branch on
/// `code` and still show something useful to a human.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Invalid(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Cancelled(String),
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("archive error: {0}")]
    Zip(String),
    #[error("{0}")]
    Internal(String),
}

impl CoreError {
    pub fn code(&self) -> &'static str {
        match self {
            CoreError::NotFound(_) => "not_found",
            CoreError::Invalid(_) => "invalid",
            CoreError::Conflict(_) => "conflict",
            CoreError::Cancelled(_) => "cancelled",
            CoreError::Db(_) => "db",
            CoreError::Io(_) => "io",
            CoreError::Serde(_) => "serde",
            CoreError::Zip(_) => "archive",
            CoreError::Internal(_) => "internal",
        }
    }

    pub fn not_found(what: impl fmt::Display, id: impl fmt::Display) -> Self {
        CoreError::NotFound(format!("{what} `{id}` was not found"))
    }

    pub fn invalid(msg: impl Into<String>) -> Self {
        CoreError::Invalid(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        CoreError::Internal(msg.into())
    }
}

impl From<zip::result::ZipError> for CoreError {
    fn from(e: zip::result::ZipError) -> Self {
        CoreError::Zip(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, CoreError>;
