//! FTS5 index maintenance and querying.
//!
//! The index is a derived view of `chapter` and `block`; every mutation path
//! calls one of the `index_*` helpers so it can never go stale.

use crate::error::Result;
use crate::models::SearchHit;
use rusqlite::{params, Connection, OptionalExtension, Transaction};

/// title, summary, objectives, deleted_at
type ChapterIndexRow = (String, Option<String>, Option<String>, Option<i64>);
/// kind, heading, caption, body, code, deleted_at
type BlockIndexRow = (
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i64>,
);
/// entity_id, entity_type, title, snippet
type RawHit = (String, String, String, String);
/// chapter id, chapter title, chapter display number
type Located = (String, String, Option<String>);

fn upsert(
    conn: &Connection,
    entity_id: &str,
    entity_type: &str,
    title: &str,
    body: &str,
) -> Result<()> {
    conn.execute("DELETE FROM search_idx WHERE entity_id = ?1", [entity_id])?;
    conn.execute(
        "INSERT INTO search_idx (entity_id, entity_type, title, body) VALUES (?1, ?2, ?3, ?4)",
        params![entity_id, entity_type, title, body],
    )?;
    Ok(())
}

pub fn remove(conn: &Connection, entity_id: &str) -> Result<()> {
    conn.execute("DELETE FROM search_idx WHERE entity_id = ?1", [entity_id])?;
    Ok(())
}

pub fn index_chapter(conn: &Connection, id: &str) -> Result<()> {
    let row: Option<ChapterIndexRow> = conn
        .query_row(
            "SELECT title, summary_md, objectives_md, deleted_at FROM chapter WHERE id = ?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .optional()?;
    let Some((title, summary, objectives, deleted)) = row else {
        return remove(conn, id);
    };
    if deleted.is_some() {
        return remove(conn, id);
    }
    let body = [summary.unwrap_or_default(), objectives.unwrap_or_default()].join("\n");
    upsert(conn, id, "chapter", &title, &body)
}

pub fn index_block(conn: &Connection, id: &str) -> Result<()> {
    let row: Option<BlockIndexRow> = conn
        .query_row(
            "SELECT kind, heading, caption_md, body_md, code_text, deleted_at
             FROM block WHERE id = ?1",
            [id],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            },
        )
        .optional()?;
    let Some((kind, heading, caption, body, code, deleted)) = row else {
        return remove(conn, id);
    };
    if deleted.is_some() {
        return remove(conn, id);
    }
    let title = heading.unwrap_or_else(|| format!("Untitled {kind} block"));
    let body = [
        caption.unwrap_or_default(),
        body.unwrap_or_default(),
        code.unwrap_or_default(),
    ]
    .join("\n");
    upsert(conn, id, "block", &title, &body)
}

pub fn index_chapter_in_tx(tx: &Transaction<'_>, id: &str) -> Result<()> {
    index_chapter(tx, id)
}

pub fn index_block_in_tx(tx: &Transaction<'_>, id: &str) -> Result<()> {
    index_block(tx, id)
}

/// Rebuilds the whole index — used after a bundle import.
pub fn reindex_all(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM search_idx", [])?;
    let chapters: Vec<String> = {
        let mut s = conn.prepare("SELECT id FROM chapter WHERE deleted_at IS NULL")?;
        let out = s
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        out
    };
    for id in chapters {
        index_chapter(conn, &id)?;
    }
    let blocks: Vec<String> = {
        let mut s = conn.prepare("SELECT id FROM block WHERE deleted_at IS NULL")?;
        let out = s
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        out
    };
    for id in blocks {
        index_block(conn, &id)?;
    }
    Ok(())
}

/// Escapes user input so FTS5 treats it as a phrase prefix rather than syntax.
/// Without this a stray `"` or `*` turns a search into a parse error.
pub fn to_match_expr(q: &str) -> Option<String> {
    let tokens: Vec<String> = q
        .split_whitespace()
        .map(|t| {
            t.chars()
                .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '\'')
                .collect::<String>()
        })
        .filter(|t| !t.is_empty())
        .map(|t| format!("\"{t}\"*"))
        .collect();
    (!tokens.is_empty()).then(|| tokens.join(" AND "))
}

pub fn query(conn: &Connection, q: &str, limit: u32) -> Result<Vec<SearchHit>> {
    let Some(expr) = to_match_expr(q) else {
        return Ok(Vec::new());
    };
    let sql = "
        SELECT s.entity_id, s.entity_type, s.title,
               snippet(search_idx, 3, '[', ']', '…', 12) AS snip
        FROM search_idx s
        WHERE search_idx MATCH ?1
        ORDER BY bm25(search_idx, 0.0, 0.0, 4.0, 1.0)
        LIMIT ?2";
    let mut stmt = conn.prepare(sql)?;
    let raw: Vec<RawHit> = stmt
        .query_map(params![expr, limit], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    let mut hits = Vec::with_capacity(raw.len());
    for (entity_id, entity_type, title, snippet) in raw {
        let located: Option<Located> = if entity_type == "chapter" {
            conn.query_row(
                "SELECT id, title, display_number FROM chapter WHERE id = ?1 AND deleted_at IS NULL",
                [&entity_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?
        } else {
            conn.query_row(
                "SELECT c.id, c.title, c.display_number FROM block b
                 JOIN chapter c ON c.id = b.chapter_id
                 WHERE b.id = ?1 AND b.deleted_at IS NULL AND c.deleted_at IS NULL",
                [&entity_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?
        };
        // A hit whose chapter has since been deleted is silently dropped.
        let Some((chapter_id, chapter_title, chapter_number)) = located else {
            continue;
        };
        hits.push(SearchHit {
            entity_id,
            entity_type,
            chapter_id,
            chapter_title,
            chapter_number,
            title,
            snippet,
        });
    }
    Ok(hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_syntax_cannot_be_injected() {
        assert_eq!(to_match_expr("ownership"), Some("\"ownership\"*".into()));
        assert_eq!(
            to_match_expr("borrow checker"),
            Some("\"borrow\"* AND \"checker\"*".into())
        );
        // Quotes and operators are stripped rather than passed through.
        assert_eq!(
            to_match_expr(r#"a" OR b"#),
            Some("\"a\"* AND \"OR\"* AND \"b\"*".into())
        );
        assert_eq!(to_match_expr("   "), None);
        assert_eq!(to_match_expr("***"), None);
    }
}
