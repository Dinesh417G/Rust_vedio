pub mod migrations;

use crate::error::Result;
use rusqlite::Connection;
use std::path::Path;

/// Opens (creating if needed) the course database and brings it to the current
/// schema version. WAL keeps reads non-blocking while an import writes.
pub fn open(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut conn = Connection::open(path)?;
    configure(&conn)?;
    migrations::migrate(&mut conn)?;
    Ok(conn)
}

/// In-memory database with the full schema — used by the test suite.
pub fn open_memory() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    migrations::migrate(&mut conn)?;
    Ok(conn)
}

fn configure(conn: &Connection) -> Result<()> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "busy_timeout", 5000)?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    Ok(())
}
