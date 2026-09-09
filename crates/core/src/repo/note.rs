use crate::error::{CoreError, Result};
use crate::ids::new_id;
use crate::models::Note;
use crate::time::now_ms;
use rusqlite::{params, Connection};

const COLS: &str = "id, chapter_id, body_md, created_at, updated_at";

fn map(r: &rusqlite::Row<'_>) -> rusqlite::Result<Note> {
    Ok(Note {
        id: r.get(0)?,
        chapter_id: r.get(1)?,
        body_md: r.get(2)?,
        created_at: r.get(3)?,
        updated_at: r.get(4)?,
    })
}

pub fn list(conn: &Connection, chapter_id: &str) -> Result<Vec<Note>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {COLS} FROM note WHERE chapter_id = ?1 ORDER BY created_at"
    ))?;
    let rows = stmt
        .query_map([chapter_id], map)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn create(conn: &Connection, chapter_id: &str, body_md: &str) -> Result<Note> {
    let id = new_id();
    let now = now_ms();
    conn.execute(
        "INSERT INTO note (id, chapter_id, body_md, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?4)",
        params![&id, chapter_id, body_md, now],
    )?;
    Ok(Note {
        id,
        chapter_id: chapter_id.to_string(),
        body_md: body_md.to_string(),
        created_at: now,
        updated_at: now,
    })
}

pub fn update(conn: &Connection, id: &str, body_md: &str) -> Result<Note> {
    let n = conn.execute(
        "UPDATE note SET body_md = ?2, updated_at = ?3 WHERE id = ?1",
        params![id, body_md, now_ms()],
    )?;
    if n == 0 {
        return Err(CoreError::not_found("note", id));
    }
    Ok(conn.query_row(&format!("SELECT {COLS} FROM note WHERE id = ?1"), [id], map)?)
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    let n = conn.execute("DELETE FROM note WHERE id = ?1", [id])?;
    if n == 0 {
        return Err(CoreError::not_found("note", id));
    }
    Ok(())
}
