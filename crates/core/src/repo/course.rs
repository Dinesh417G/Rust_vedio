use crate::error::{CoreError, Result};
use crate::ids::new_id;
use crate::models::{CoursePatch, CourseView};
use crate::time::now_ms;
use rusqlite::{params, Connection, OptionalExtension};

const COLS: &str =
    "id, title, subtitle, author, description_md, cover_asset_id, created_at, updated_at";

fn map(r: &rusqlite::Row<'_>) -> rusqlite::Result<CourseView> {
    Ok(CourseView {
        id: r.get(0)?,
        title: r.get(1)?,
        subtitle: r.get(2)?,
        author: r.get(3)?,
        description_md: r.get(4)?,
        cover_asset_id: r.get(5)?,
        created_at: r.get(6)?,
        updated_at: r.get(7)?,
    })
}

/// There is exactly one course per database. First run seeds it.
pub fn get_or_seed(conn: &Connection) -> Result<CourseView> {
    let existing = conn
        .query_row(
            &format!("SELECT {COLS} FROM course ORDER BY created_at LIMIT 1"),
            [],
            map,
        )
        .optional()?;
    if let Some(c) = existing {
        return Ok(c);
    }
    let now = now_ms();
    let id = new_id();
    conn.execute(
        "INSERT INTO course (id, title, subtitle, author, description_md, created_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, NULL, ?4, ?4)",
        params![&id, "My Rust Course", "A course I am building", now],
    )?;
    get(conn, &id)
}

pub fn get(conn: &Connection, id: &str) -> Result<CourseView> {
    conn.query_row(
        &format!("SELECT {COLS} FROM course WHERE id = ?1"),
        [id],
        map,
    )
    .optional()?
    .ok_or_else(|| CoreError::not_found("course", id))
}

pub fn update(conn: &Connection, patch: CoursePatch) -> Result<CourseView> {
    let current = get_or_seed(conn)?;
    if let Some(t) = &patch.title {
        if t.trim().is_empty() {
            return Err(CoreError::invalid("A course needs a title."));
        }
    }
    let title = patch.title.unwrap_or(current.title);
    let subtitle = patch.subtitle.unwrap_or(current.subtitle);
    let author = patch.author.unwrap_or(current.author);
    let description_md = patch.description_md.unwrap_or(current.description_md);
    let cover_asset_id = patch.cover_asset_id.unwrap_or(current.cover_asset_id);
    conn.execute(
        "UPDATE course SET title = ?2, subtitle = ?3, author = ?4, description_md = ?5,
                cover_asset_id = ?6, updated_at = ?7 WHERE id = ?1",
        params![
            &current.id,
            title,
            subtitle,
            author,
            description_md,
            cover_asset_id,
            now_ms()
        ],
    )?;
    get(conn, &current.id)
}
