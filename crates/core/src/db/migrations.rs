use crate::error::Result;
use rusqlite::Connection;

/// Forward-only migration ladder driven by `PRAGMA user_version`.
///
/// Each entry is applied inside a transaction; adding a migration means
/// appending to this slice and never editing an existing one.
const MIGRATIONS: &[&str] = &[
    // ---- v1: initial schema -------------------------------------------------
    r#"
CREATE TABLE course (
  id             TEXT PRIMARY KEY,
  title          TEXT NOT NULL,
  subtitle       TEXT,
  author         TEXT,
  description_md TEXT,
  cover_asset_id TEXT REFERENCES asset(id) ON DELETE SET NULL,
  created_at     INTEGER NOT NULL,
  updated_at     INTEGER NOT NULL
);

CREATE TABLE chapter (
  id                     TEXT PRIMARY KEY,
  course_id              TEXT NOT NULL REFERENCES course(id) ON DELETE CASCADE,
  display_number         TEXT,
  title                  TEXT NOT NULL,
  summary_md             TEXT,
  objectives_md          TEXT,
  estimated_minutes      INTEGER,
  status                 TEXT NOT NULL DEFAULT 'draft',
  sort_index             REAL NOT NULL,
  created_at             INTEGER NOT NULL,
  updated_at             INTEGER NOT NULL,
  deleted_at             INTEGER
);
CREATE INDEX idx_chapter_course ON chapter(course_id, sort_index);

CREATE TABLE block (
  id          TEXT PRIMARY KEY,
  chapter_id  TEXT NOT NULL REFERENCES chapter(id) ON DELETE CASCADE,
  kind        TEXT NOT NULL,
  heading     TEXT,
  caption_md  TEXT,
  body_md     TEXT,
  code_text   TEXT,
  code_lang   TEXT DEFAULT 'rust',
  asset_id    TEXT REFERENCES asset(id) ON DELETE SET NULL,
  meta_json   TEXT NOT NULL DEFAULT '{}',
  sort_index  REAL NOT NULL,
  created_at  INTEGER NOT NULL,
  updated_at  INTEGER NOT NULL,
  deleted_at  INTEGER
);
CREATE INDEX idx_block_chapter ON block(chapter_id, sort_index);
CREATE INDEX idx_block_asset ON block(asset_id);

CREATE TABLE asset (
  id             TEXT PRIMARY KEY,
  kind           TEXT NOT NULL,
  original_name  TEXT NOT NULL,
  rel_path       TEXT NOT NULL,
  abs_path       TEXT,
  import_mode    TEXT NOT NULL DEFAULT 'copy',
  mime           TEXT,
  bytes          INTEGER NOT NULL,
  sha256         TEXT NOT NULL,
  duration_secs  REAL,
  width          INTEGER,
  height         INTEGER,
  thumb_rel_path TEXT,
  imported_at    INTEGER NOT NULL
);
CREATE INDEX idx_asset_sha ON asset(sha256);

CREATE TABLE progress (
  block_id           TEXT PRIMARY KEY REFERENCES block(id) ON DELETE CASCADE,
  completed          INTEGER NOT NULL DEFAULT 0,
  last_position_secs REAL DEFAULT 0,
  watched_secs       REAL DEFAULT 0,
  updated_at         INTEGER NOT NULL
);

CREATE TABLE marker (
  id       TEXT PRIMARY KEY,
  block_id TEXT NOT NULL REFERENCES block(id) ON DELETE CASCADE,
  at_secs  REAL NOT NULL,
  label    TEXT NOT NULL,
  note_md  TEXT
);
CREATE INDEX idx_marker_block ON marker(block_id, at_secs);

CREATE TABLE note (
  id         TEXT PRIMARY KEY,
  chapter_id TEXT NOT NULL REFERENCES chapter(id) ON DELETE CASCADE,
  body_md    TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
CREATE INDEX idx_note_chapter ON note(chapter_id);

CREATE VIRTUAL TABLE search_idx USING fts5(
  entity_id UNINDEXED, entity_type UNINDEXED, title, body, tokenize = 'porter'
);
"#,
];

pub fn current_version(conn: &Connection) -> Result<i64> {
    Ok(conn.query_row("PRAGMA user_version", [], |r| r.get(0))?)
}

pub fn target_version() -> i64 {
    MIGRATIONS.len() as i64
}

/// Applies every migration above the connection's current `user_version`.
pub fn migrate(conn: &mut Connection) -> Result<i64> {
    let mut version = current_version(conn)?;
    if version > target_version() {
        return Err(crate::error::CoreError::internal(format!(
            "database is at schema v{version} but this build only understands v{}; \
             upgrade RustClass or restore an older backup",
            target_version()
        )));
    }
    while version < target_version() {
        let sql = MIGRATIONS[version as usize];
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        version += 1;
        tx.pragma_update(None, "user_version", version)?;
        tx.commit()?;
        tracing::info!(version, "applied migration");
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_up_from_empty() {
        let mut conn = Connection::open_in_memory().expect("open memory db");
        assert_eq!(current_version(&conn).unwrap(), 0);
        let v = migrate(&mut conn).unwrap();
        assert_eq!(v, target_version());
        // Every table the data model promises must exist.
        for t in [
            "course",
            "chapter",
            "block",
            "asset",
            "progress",
            "marker",
            "note",
            "search_idx",
        ] {
            let n: i64 = conn
                .query_row(
                    "SELECT count(*) FROM sqlite_master WHERE name = ?1",
                    [t],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(n, 1, "table {t} missing");
        }
    }

    #[test]
    fn migration_is_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        let again = migrate(&mut conn).unwrap();
        assert_eq!(again, target_version());
    }

    #[test]
    fn refuses_a_newer_database() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "user_version", target_version() + 5)
            .unwrap();
        assert!(migrate(&mut conn).is_err());
    }
}
