use crate::error::Result;
use crate::models::{ProgressPatch, ProgressView};
use crate::time::now_ms;
use rusqlite::{params, Connection, OptionalExtension};

pub fn get(conn: &Connection, block_id: &str) -> Result<Option<ProgressView>> {
    Ok(conn
        .query_row(
            "SELECT block_id, completed, last_position_secs, watched_secs, updated_at
             FROM progress WHERE block_id = ?1",
            [block_id],
            |r| {
                Ok(ProgressView {
                    block_id: r.get(0)?,
                    completed: r.get::<_, i64>(1)? != 0,
                    last_position_secs: r.get::<_, Option<f64>>(2)?.unwrap_or(0.0),
                    watched_secs: r.get::<_, Option<f64>>(3)?.unwrap_or(0.0),
                    updated_at: r.get(4)?,
                })
            },
        )
        .optional()?)
}

pub fn set(conn: &Connection, block_id: &str, patch: ProgressPatch) -> Result<ProgressView> {
    let current = get(conn, block_id)?;
    let completed = patch
        .completed
        .unwrap_or_else(|| current.as_ref().is_some_and(|c| c.completed));
    let position = patch
        .last_position_secs
        .or_else(|| current.as_ref().map(|c| c.last_position_secs))
        .unwrap_or(0.0)
        .max(0.0);
    // Watch time only ever grows; scrubbing backwards must not erase it.
    let watched = patch
        .watched_secs
        .unwrap_or(0.0)
        .max(current.as_ref().map(|c| c.watched_secs).unwrap_or(0.0));
    conn.execute(
        "INSERT INTO progress (block_id, completed, last_position_secs, watched_secs, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(block_id) DO UPDATE SET
           completed = excluded.completed,
           last_position_secs = excluded.last_position_secs,
           watched_secs = excluded.watched_secs,
           updated_at = excluded.updated_at",
        params![block_id, completed as i64, position, watched, now_ms()],
    )?;
    Ok(get(conn, block_id)?.expect("row was just written"))
}

/// Marks every non-divider block in a chapter complete (or not).
pub fn set_chapter(conn: &Connection, chapter_id: &str, completed: bool) -> Result<u32> {
    let ids: Vec<String> = {
        let mut s = conn.prepare(
            "SELECT id FROM block WHERE chapter_id = ?1 AND deleted_at IS NULL AND kind != 'divider'",
        )?;
        let out = s
            .query_map([chapter_id], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        out
    };
    let n = ids.len() as u32;
    for id in ids {
        set(
            conn,
            &id,
            ProgressPatch {
                completed: Some(completed),
                ..Default::default()
            },
        )?;
    }
    Ok(n)
}
