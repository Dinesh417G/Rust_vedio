use super::{map_chapter, ReadCtx, CHAPTER_COLS};
use crate::error::{CoreError, Result};
use crate::ids::new_id;
use crate::models::*;
use crate::repo::block::Anchor;

/// id, kind, heading, caption, body, code, lang, asset, meta, sort_index —
/// the shape `duplicate` copies verbatim.
type BlockCopyRow = (
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    String,
    f64,
);
use crate::repo::{block, course, search};
use crate::sortindex;
use crate::time::now_ms;
use rusqlite::{params, Connection, OptionalExtension, Transaction};

/// A part or section that is empty or only whitespace is no grouping at all.
/// Trimming also keeps "Module 2" and "Module 2 " from becoming two groups.
pub(crate) fn clean_group(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
}

pub fn list_rows(conn: &Connection, include_deleted: bool) -> Result<Vec<Chapter>> {
    let where_clause = if include_deleted {
        ""
    } else {
        "WHERE deleted_at IS NULL"
    };
    let mut stmt = conn.prepare(&format!(
        "SELECT {CHAPTER_COLS} FROM chapter {where_clause} ORDER BY deleted_at IS NOT NULL, sort_index"
    ))?;
    let rows = stmt
        .query_map([], map_chapter)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn list(
    conn: &Connection,
    ctx: &ReadCtx<'_>,
    include_deleted: bool,
) -> Result<Vec<ChapterSummary>> {
    let rows = list_rows(conn, include_deleted)?;
    rows.into_iter()
        .map(|c| {
            let (block_count, ready, empty, missing, completed) =
                block::state_counts(conn, ctx, &c.id)?;
            Ok(ChapterSummary {
                chapter: c,
                block_count,
                ready_count: ready,
                empty_count: empty,
                missing_count: missing,
                completed_count: completed,
            })
        })
        .collect()
}

pub fn get_row(conn: &Connection, id: &str) -> Result<Chapter> {
    conn.query_row(
        &format!("SELECT {CHAPTER_COLS} FROM chapter WHERE id = ?1"),
        [id],
        map_chapter,
    )
    .optional()?
    .ok_or_else(|| CoreError::not_found("chapter", id))
}

pub fn get(conn: &Connection, ctx: &ReadCtx<'_>, id: &str) -> Result<ChapterView> {
    let chapter = get_row(conn, id)?;
    let blocks = block::list(conn, ctx, id)?;
    Ok(ChapterView { chapter, blocks })
}

fn key_after(tx: &Transaction<'_>, anchor: Anchor<'_>, moving_id: Option<&str>) -> Result<f64> {
    let mut stmt = tx.prepare(
        "SELECT id, sort_index FROM chapter WHERE deleted_at IS NULL ORDER BY sort_index",
    )?;
    let mut items: Vec<(String, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);
    if let Some(m) = moving_id {
        items.retain(|(id, _)| id != m);
    }
    let pos = match anchor {
        Anchor::Start => 0,
        Anchor::End => items.len(),
        Anchor::After(a) => items
            .iter()
            .position(|(id, _)| id == a)
            .map(|i| i + 1)
            .ok_or_else(|| CoreError::not_found("chapter", a))?,
    };
    let before = pos
        .checked_sub(1)
        .and_then(|i| items.get(i))
        .map(|(_, k)| *k);
    let after = items.get(pos).map(|(_, k)| *k);
    let key = sortindex::between(before, after);

    let mut projected: Vec<f64> = items.iter().map(|(_, k)| *k).collect();
    projected.insert(pos, key);
    if sortindex::needs_rebalance(&projected) {
        rebalance(tx, moving_id)?;
        let fresh = sortindex::rebalanced(items.len());
        let before = pos.checked_sub(1).and_then(|i| fresh.get(i)).copied();
        let after = fresh.get(pos).copied();
        return Ok(sortindex::between(before, after));
    }
    Ok(key)
}

pub fn rebalance(tx: &Transaction<'_>, skip: Option<&str>) -> Result<()> {
    let mut stmt =
        tx.prepare("SELECT id FROM chapter WHERE deleted_at IS NULL ORDER BY sort_index")?;
    let ids: Vec<String> = stmt
        .query_map([], |r| r.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);
    let mut n = 0usize;
    for id in ids {
        if Some(id.as_str()) == skip {
            continue;
        }
        n += 1;
        tx.execute(
            "UPDATE chapter SET sort_index = ?2 WHERE id = ?1",
            params![id, n as f64 * sortindex::STEP],
        )?;
    }
    Ok(())
}

pub fn create(conn: &mut Connection, ctx: &ReadCtx<'_>, req: NewChapter) -> Result<ChapterView> {
    if req.title.trim().is_empty() {
        return Err(CoreError::invalid("A chapter needs a title."));
    }
    let course = course::get_or_seed(conn)?;
    let id = {
        let tx = conn.transaction()?;
        let id = insert_in_tx(&tx, &course.id, &req)?;
        for (kind, seed) in req.template.unwrap_or_default().blocks() {
            block::insert_in_tx(&tx, &id, kind, None, Some(seed))?;
        }
        tx.commit()?;
        id
    };
    get(conn, ctx, &id)
}

pub fn insert_in_tx(tx: &Transaction<'_>, course_id: &str, req: &NewChapter) -> Result<String> {
    let key = key_after(tx, Anchor::after_or_end(req.after_id.as_deref()), None)?;
    let id = new_id();
    let now = now_ms();
    tx.execute(
        "INSERT INTO chapter (id, course_id, display_number, title, summary_md, objectives_md,
                              estimated_minutes, status, sort_index, created_at, updated_at,
                              part, section)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'draft', ?8, ?9, ?9, ?10, ?11)",
        params![
            &id,
            course_id,
            req.display_number,
            req.title.trim(),
            req.summary_md,
            req.objectives_md,
            req.estimated_minutes,
            key,
            now,
            clean_group(req.part.as_deref()),
            clean_group(req.section.as_deref())
        ],
    )?;
    search::index_chapter_in_tx(tx, &id)?;
    Ok(id)
}

pub fn update(
    conn: &Connection,
    ctx: &ReadCtx<'_>,
    id: &str,
    patch: ChapterPatch,
) -> Result<ChapterView> {
    let current = get_row(conn, id)?;
    if let Some(t) = &patch.title {
        if t.trim().is_empty() {
            return Err(CoreError::invalid("A chapter needs a title."));
        }
    }
    let title = patch
        .title
        .map(|t| t.trim().to_string())
        .unwrap_or(current.title);
    let number = patch.display_number.unwrap_or(current.display_number);
    let summary = patch.summary_md.unwrap_or(current.summary_md);
    let objectives = patch.objectives_md.unwrap_or(current.objectives_md);
    let minutes = patch.estimated_minutes.unwrap_or(current.estimated_minutes);
    let status = patch.status.unwrap_or(current.status);
    let part = patch
        .part
        .map(|p| clean_group(p.as_deref()))
        .unwrap_or(current.part);
    let section = patch
        .section
        .map(|s| clean_group(s.as_deref()))
        .unwrap_or(current.section);
    conn.execute(
        "UPDATE chapter SET title = ?2, display_number = ?3, summary_md = ?4, objectives_md = ?5,
                estimated_minutes = ?6, status = ?7, part = ?8, section = ?9, updated_at = ?10
         WHERE id = ?1",
        params![
            id,
            title,
            number,
            summary,
            objectives,
            minutes,
            status.as_str(),
            part,
            section,
            now_ms()
        ],
    )?;
    search::index_chapter(conn, id)?;
    get(conn, ctx, id)
}

pub fn reorder(conn: &mut Connection, id: &str, after_id: Option<&str>) -> Result<()> {
    if Some(id) == after_id {
        return Err(CoreError::invalid(
            "A chapter cannot be placed after itself.",
        ));
    }
    let tx = conn.transaction()?;
    let exists: bool = tx
        .query_row("SELECT 1 FROM chapter WHERE id = ?1", [id], |_| Ok(true))
        .optional()?
        .unwrap_or(false);
    if !exists {
        return Err(CoreError::not_found("chapter", id));
    }
    let key = key_after(&tx, Anchor::after_or_start(after_id), Some(id))?;
    tx.execute(
        "UPDATE chapter SET sort_index = ?2, updated_at = ?3 WHERE id = ?1",
        params![id, key, now_ms()],
    )?;
    tx.commit()?;
    Ok(())
}

/// Copies structure only. Assets are shared, never re-copied on disk.
pub fn duplicate(conn: &mut Connection, ctx: &ReadCtx<'_>, id: &str) -> Result<ChapterView> {
    let src = get_row(conn, id)?;
    let new_chapter_id = {
        let tx = conn.transaction()?;
        let key = key_after(&tx, Anchor::After(id), None)?;
        let new_id_ = new_id();
        let now = now_ms();
        tx.execute(
            "INSERT INTO chapter (id, course_id, display_number, title, summary_md, objectives_md,
                                  estimated_minutes, status, sort_index, created_at, updated_at,
                                  part, section)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'draft', ?8, ?9, ?9, ?10, ?11)",
            params![
                &new_id_,
                &src.course_id,
                &src.display_number,
                format!("{} (copy)", src.title),
                &src.summary_md,
                &src.objectives_md,
                src.estimated_minutes,
                key,
                now,
                &src.part,
                &src.section
            ],
        )?;
        {
            let mut stmt = tx.prepare(
                "SELECT id, kind, heading, caption_md, body_md, code_text, code_lang, asset_id,
                        meta_json, sort_index
                 FROM block WHERE chapter_id = ?1 AND deleted_at IS NULL ORDER BY sort_index",
            )?;
            let rows: Vec<BlockCopyRow> = stmt
                .query_map([id], |r| {
                    Ok((
                        r.get(0)?,
                        r.get(1)?,
                        r.get(2)?,
                        r.get(3)?,
                        r.get(4)?,
                        r.get(5)?,
                        r.get(6)?,
                        r.get(7)?,
                        r.get(8)?,
                        r.get(9)?,
                    ))
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            drop(stmt);
            for (_src_id, kind, heading, caption, body, code, lang, asset, meta, sort) in rows {
                let bid = new_id();
                tx.execute(
                    "INSERT INTO block (id, chapter_id, kind, heading, caption_md, body_md,
                                        code_text, code_lang, asset_id, meta_json, sort_index,
                                        created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12)",
                    params![
                        &bid, &new_id_, kind, heading, caption, body, code, lang, asset, meta,
                        sort, now
                    ],
                )?;
                search::index_block_in_tx(&tx, &bid)?;
            }
        }
        search::index_chapter_in_tx(&tx, &new_id_)?;
        tx.commit()?;
        new_id_
    };
    get(conn, ctx, &new_chapter_id)
}

pub fn soft_delete(conn: &Connection, id: &str) -> Result<()> {
    let n = conn.execute(
        "UPDATE chapter SET deleted_at = ?2, updated_at = ?2 WHERE id = ?1 AND deleted_at IS NULL",
        params![id, now_ms()],
    )?;
    if n == 0 {
        return Err(CoreError::not_found("chapter", id));
    }
    search::remove(conn, id)?;
    // Blocks stay untouched so a restore brings back the whole page intact.
    Ok(())
}

pub fn restore(conn: &Connection, id: &str) -> Result<()> {
    let n = conn.execute(
        "UPDATE chapter SET deleted_at = NULL, updated_at = ?2 WHERE id = ?1",
        params![id, now_ms()],
    )?;
    if n == 0 {
        return Err(CoreError::not_found("chapter", id));
    }
    search::index_chapter(conn, id)?;
    Ok(())
}

/// Permanent removal. Blocks cascade; asset files are never deleted here.
pub fn purge(conn: &Connection, id: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id FROM block WHERE chapter_id = ?1")?;
    let block_ids: Vec<String> = stmt
        .query_map([id], |r| r.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);
    conn.execute("DELETE FROM chapter WHERE id = ?1", [id])?;
    for b in block_ids {
        search::remove(conn, &b)?;
    }
    search::remove(conn, id)?;
    Ok(())
}

pub fn progress(conn: &Connection) -> Result<CourseProgress> {
    let chapters = list_rows(conn, false)?;
    let mut out = Vec::new();
    let (mut total, mut done) = (0u32, 0u32);
    for c in &chapters {
        let t: i64 = conn.query_row(
            "SELECT count(*) FROM block WHERE chapter_id = ?1 AND deleted_at IS NULL
             AND kind != 'divider'",
            [&c.id],
            |r| r.get(0),
        )?;
        let d: i64 = conn.query_row(
            "SELECT count(*) FROM progress p JOIN block b ON b.id = p.block_id
             WHERE b.chapter_id = ?1 AND b.deleted_at IS NULL AND b.kind != 'divider'
             AND p.completed = 1",
            [&c.id],
            |r| r.get(0),
        )?;
        total += t as u32;
        done += d as u32;
        out.push(ChapterProgress {
            chapter_id: c.id.clone(),
            title: c.title.clone(),
            display_number: c.display_number.clone(),
            total_blocks: t as u32,
            completed_blocks: d as u32,
            percent: if t == 0 {
                0.0
            } else {
                d as f64 * 100.0 / t as f64
            },
        });
    }
    let last: Option<(String, String)> = conn
        .query_row(
            "SELECT b.id, b.chapter_id FROM progress p JOIN block b ON b.id = p.block_id
             JOIN chapter c ON c.id = b.chapter_id
             WHERE b.deleted_at IS NULL AND c.deleted_at IS NULL
             ORDER BY p.updated_at DESC LIMIT 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    Ok(CourseProgress {
        chapters: out,
        total_blocks: total,
        completed_blocks: done,
        percent: if total == 0 {
            0.0
        } else {
            done as f64 * 100.0 / total as f64
        },
        last_block_id: last.as_ref().map(|(b, _)| b.clone()),
        last_chapter_id: last.map(|(_, c)| c),
    })
}
