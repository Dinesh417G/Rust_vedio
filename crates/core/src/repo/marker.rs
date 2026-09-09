use crate::error::{CoreError, Result};
use crate::ids::new_id;
use crate::models::Marker;
use rusqlite::{params, Connection};

pub fn list(conn: &Connection, block_id: &str) -> Result<Vec<Marker>> {
    let mut stmt = conn.prepare(
        "SELECT id, block_id, at_secs, label, note_md FROM marker
         WHERE block_id = ?1 ORDER BY at_secs",
    )?;
    let rows = stmt
        .query_map([block_id], |r| {
            Ok(Marker {
                id: r.get(0)?,
                block_id: r.get(1)?,
                at_secs: r.get(2)?,
                label: r.get(3)?,
                note_md: r.get(4)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn create(
    conn: &Connection,
    block_id: &str,
    at_secs: f64,
    label: &str,
    note_md: Option<&str>,
) -> Result<Marker> {
    if label.trim().is_empty() {
        return Err(CoreError::invalid("A marker needs a label."));
    }
    if !at_secs.is_finite() || at_secs < 0.0 {
        return Err(CoreError::invalid("A marker needs a valid timestamp."));
    }
    let id = new_id();
    conn.execute(
        "INSERT INTO marker (id, block_id, at_secs, label, note_md) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&id, block_id, at_secs, label.trim(), note_md],
    )?;
    Ok(Marker {
        id,
        block_id: block_id.to_string(),
        at_secs,
        label: label.trim().to_string(),
        note_md: note_md.map(|s| s.to_string()),
    })
}

pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    let n = conn.execute("DELETE FROM marker WHERE id = ?1", [id])?;
    if n == 0 {
        return Err(CoreError::not_found("marker", id));
    }
    Ok(())
}
