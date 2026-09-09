use super::{asset_cols_prefixed, map_asset_at, map_block, ReadCtx, BLOCK_COLS};
use crate::error::{CoreError, Result};
use crate::ids::new_id;
use crate::models::*;
use crate::repo::{marker, progress, search};
use crate::sortindex;
use crate::state::{self, AssetStatus};
use crate::time::now_ms;
use rusqlite::{params, Connection, OptionalExtension, Transaction};

fn joined_select() -> String {
    let b = BLOCK_COLS
        .split(", ")
        .map(|c| format!("b.{}", c.trim()))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "SELECT {b}, {} FROM block b LEFT JOIN asset a ON a.id = b.asset_id",
        asset_cols_prefixed("a")
    )
}

const BLOCK_COL_COUNT: usize = 14;

fn row_to_view(
    r: &rusqlite::Row<'_>,
    ctx: &ReadCtx<'_>,
) -> rusqlite::Result<(Block, Option<AssetView>, AssetStatus)> {
    let block = map_block(r)?;
    let asset_id: Option<String> = r.get(BLOCK_COL_COUNT)?;
    match asset_id {
        None => Ok((block, None, AssetStatus::None)),
        Some(_) => {
            let row = map_asset_at(r, BLOCK_COL_COUNT)?;
            let view = row.into_view(ctx.paths);
            let status = if view.file_exists {
                AssetStatus::Present
            } else {
                AssetStatus::FileMissing
            };
            Ok((block, Some(view), status))
        }
    }
}

fn assemble(
    conn: &Connection,
    ctx: &ReadCtx<'_>,
    block: Block,
    asset: Option<AssetView>,
    status: AssetStatus,
) -> Result<BlockView> {
    let importing = ctx.importing_blocks.contains(&block.id);
    let state = state::compute(&block, status, importing);
    let progress = progress::get(conn, &block.id)?;
    let markers = if block.kind == BlockKind::Video {
        marker::list(conn, &block.id)?
    } else {
        Vec::new()
    };
    Ok(BlockView {
        block,
        state,
        asset,
        progress,
        markers,
    })
}

pub fn list(conn: &Connection, ctx: &ReadCtx<'_>, chapter_id: &str) -> Result<Vec<BlockView>> {
    let sql = format!(
        "{} WHERE b.chapter_id = ?1 AND b.deleted_at IS NULL ORDER BY b.sort_index",
        joined_select()
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map([chapter_id], |r| row_to_view(r, ctx))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    rows.into_iter()
        .map(|(b, a, s)| assemble(conn, ctx, b, a, s))
        .collect()
}

pub fn get(conn: &Connection, ctx: &ReadCtx<'_>, id: &str) -> Result<BlockView> {
    let sql = format!("{} WHERE b.id = ?1", joined_select());
    let found = conn
        .query_row(&sql, [id], |r| row_to_view(r, ctx))
        .optional()?;
    let (b, a, s) = found.ok_or_else(|| CoreError::not_found("block", id))?;
    assemble(conn, ctx, b, a, s)
}

pub fn get_row(conn: &Connection, id: &str) -> Result<Block> {
    conn.query_row(
        &format!("SELECT {BLOCK_COLS} FROM block WHERE id = ?1"),
        [id],
        map_block,
    )
    .optional()?
    .ok_or_else(|| CoreError::not_found("block", id))
}

/// Where an item is being placed.
///
/// Creation and movement want different defaults for "no anchor given":
/// a new block belongs at the end of the chapter, while dragging a block to
/// the top of the list means there is nothing for it to go after. Making that
/// explicit here keeps the two callers from quietly disagreeing.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Anchor<'a> {
    Start,
    End,
    After(&'a str),
}

impl<'a> Anchor<'a> {
    pub(crate) fn after_or_end(id: Option<&'a str>) -> Self {
        match id {
            Some(i) => Anchor::After(i),
            None => Anchor::End,
        }
    }
    pub(crate) fn after_or_start(id: Option<&'a str>) -> Self {
        match id {
            Some(i) => Anchor::After(i),
            None => Anchor::Start,
        }
    }
}

/// Sort key for a new or moved block, plus a rebalance when the gaps collapse.
fn key_after(
    tx: &Transaction<'_>,
    chapter_id: &str,
    anchor: Anchor<'_>,
    moving_id: Option<&str>,
) -> Result<f64> {
    let mut stmt = tx.prepare(
        "SELECT id, sort_index FROM block
         WHERE chapter_id = ?1 AND deleted_at IS NULL ORDER BY sort_index",
    )?;
    let mut items: Vec<(String, f64)> = stmt
        .query_map([chapter_id], |r| Ok((r.get(0)?, r.get(1)?)))?
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
            .ok_or_else(|| {
                CoreError::invalid("The block to insert after is not in this chapter.")
            })?,
    };
    let before = pos
        .checked_sub(1)
        .and_then(|i| items.get(i))
        .map(|(_, k)| *k);
    let after = items.get(pos).map(|(_, k)| *k);
    let key = sortindex::between(before, after);

    // If placing here would exhaust the float gap, spread the chapter out first
    // and recompute against the fresh keys.
    let mut projected: Vec<f64> = items.iter().map(|(_, k)| *k).collect();
    projected.insert(pos, key);
    if sortindex::needs_rebalance(&projected) {
        rebalance(tx, chapter_id, moving_id)?;
        let fresh = sortindex::rebalanced(items.len());
        let before = pos.checked_sub(1).and_then(|i| fresh.get(i)).copied();
        let after = fresh.get(pos).copied();
        return Ok(sortindex::between(before, after));
    }
    Ok(key)
}

/// Rewrites a chapter's block keys to `1024, 2048, 3072, ...` in one statement
/// pass. `skip` is the block currently being moved, which is placed afterwards.
pub fn rebalance(tx: &Transaction<'_>, chapter_id: &str, skip: Option<&str>) -> Result<()> {
    let mut stmt = tx.prepare(
        "SELECT id FROM block WHERE chapter_id = ?1 AND deleted_at IS NULL ORDER BY sort_index",
    )?;
    let ids: Vec<String> = stmt
        .query_map([chapter_id], |r| r.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);
    let mut n = 0usize;
    for id in ids {
        if Some(id.as_str()) == skip {
            continue;
        }
        n += 1;
        tx.execute(
            "UPDATE block SET sort_index = ?2 WHERE id = ?1",
            params![id, n as f64 * sortindex::STEP],
        )?;
    }
    tracing::debug!(chapter_id, blocks = n, "rebalanced block order");
    Ok(())
}

pub fn create(
    conn: &mut Connection,
    ctx: &ReadCtx<'_>,
    chapter_id: &str,
    kind: BlockKind,
    after_block_id: Option<&str>,
    seed: Option<BlockSeed>,
) -> Result<BlockView> {
    let id = {
        let tx = conn.transaction()?;
        let exists: bool = tx
            .query_row(
                "SELECT 1 FROM chapter WHERE id = ?1 AND deleted_at IS NULL",
                [chapter_id],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);
        if !exists {
            return Err(CoreError::not_found("chapter", chapter_id));
        }
        let id = insert_in_tx(&tx, chapter_id, kind, after_block_id, seed)?;
        tx.commit()?;
        id
    };
    get(conn, ctx, &id)
}

/// Insert used by `create`, chapter templates and outline import alike.
pub fn insert_in_tx(
    tx: &Transaction<'_>,
    chapter_id: &str,
    kind: BlockKind,
    after_block_id: Option<&str>,
    seed: Option<BlockSeed>,
) -> Result<String> {
    let seed = seed.unwrap_or_default();
    let key = key_after(tx, chapter_id, Anchor::after_or_end(after_block_id), None)?;
    let id = new_id();
    let now = now_ms();
    let meta = seed.meta_json.unwrap_or_else(|| "{}".to_string());
    serde_json::from_str::<serde_json::Value>(&meta)
        .map_err(|_| CoreError::invalid("Block metadata must be valid JSON."))?;
    tx.execute(
        "INSERT INTO block (id, chapter_id, kind, heading, caption_md, body_md, code_text,
                            code_lang, asset_id, meta_json, sort_index, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10, ?11, ?11)",
        params![
            &id,
            chapter_id,
            kind.as_str(),
            seed.heading,
            seed.caption_md,
            seed.body_md,
            seed.code_text,
            seed.code_lang
                .or_else(|| { (kind == BlockKind::Code).then(|| "rust".to_string()) }),
            meta,
            key,
            now
        ],
    )?;
    search::index_block_in_tx(tx, &id)?;
    Ok(id)
}

pub fn update(
    conn: &Connection,
    ctx: &ReadCtx<'_>,
    id: &str,
    patch: BlockPatch,
) -> Result<BlockView> {
    let current = get_row(conn, id)?;
    if let Some(meta) = &patch.meta_json {
        serde_json::from_str::<serde_json::Value>(meta)
            .map_err(|_| CoreError::invalid("Block metadata must be valid JSON."))?;
    }
    let heading = patch.heading.unwrap_or(current.heading);
    let caption = patch.caption_md.unwrap_or(current.caption_md);
    let body = patch.body_md.unwrap_or(current.body_md);
    let code = patch.code_text.unwrap_or(current.code_text);
    let lang = patch.code_lang.unwrap_or(current.code_lang);
    let meta = patch.meta_json.unwrap_or(current.meta_json);
    conn.execute(
        "UPDATE block SET heading = ?2, caption_md = ?3, body_md = ?4, code_text = ?5,
                code_lang = ?6, meta_json = ?7, updated_at = ?8 WHERE id = ?1",
        params![id, heading, caption, body, code, lang, meta, now_ms()],
    )?;
    search::index_block(conn, id)?;
    get(conn, ctx, id)
}

pub fn reorder(
    conn: &mut Connection,
    id: &str,
    target_chapter_id: &str,
    after_block_id: Option<&str>,
) -> Result<()> {
    let tx = conn.transaction()?;
    let current: Block = tx
        .query_row(
            &format!("SELECT {BLOCK_COLS} FROM block WHERE id = ?1"),
            [id],
            map_block,
        )
        .optional()?
        .ok_or_else(|| CoreError::not_found("block", id))?;
    let target_exists: bool = tx
        .query_row(
            "SELECT 1 FROM chapter WHERE id = ?1 AND deleted_at IS NULL",
            [target_chapter_id],
            |_| Ok(true),
        )
        .optional()?
        .unwrap_or(false);
    if !target_exists {
        return Err(CoreError::not_found("chapter", target_chapter_id));
    }
    if let Some(a) = after_block_id {
        if a == id {
            return Err(CoreError::invalid("A block cannot be placed after itself."));
        }
    }
    let key = key_after(
        &tx,
        target_chapter_id,
        Anchor::after_or_start(after_block_id),
        Some(id),
    )?;
    tx.execute(
        "UPDATE block SET chapter_id = ?2, sort_index = ?3, updated_at = ?4 WHERE id = ?1",
        params![id, target_chapter_id, key, now_ms()],
    )?;
    if current.chapter_id != target_chapter_id {
        search::index_block_in_tx(&tx, id)?;
    }
    tx.commit()?;
    Ok(())
}

/// What a conversion would discard, so the UI can warn before it happens.
pub fn convert_losses(block: &Block, new_kind: BlockKind) -> Vec<&'static str> {
    let mut lost = Vec::new();
    let keeps_body = matches!(new_kind, BlockKind::Text | BlockKind::Callout);
    let keeps_code = new_kind == BlockKind::Code;
    let keeps_asset =
        new_kind.asset_kind().is_some() && block.kind.asset_kind() == new_kind.asset_kind();
    if block
        .body_md
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty())
        && !keeps_body
    {
        lost.push("the written body");
    }
    if block
        .code_text
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty())
        && !keeps_code
    {
        lost.push("the code sample");
    }
    if block.asset_id.is_some() && !keeps_asset {
        lost.push("the attached file");
    }
    if state::quiz_has_questions(&block.meta_json) && new_kind != BlockKind::Quiz {
        lost.push("the quiz questions");
    }
    lost
}

/// Heading and caption always survive a conversion — they are the planning
/// layer and must outlive any payload.
pub fn convert(
    conn: &Connection,
    ctx: &ReadCtx<'_>,
    id: &str,
    new_kind: BlockKind,
) -> Result<BlockView> {
    let current = get_row(conn, id)?;
    if current.kind == new_kind {
        return get(conn, ctx, id);
    }
    let keeps_body = matches!(new_kind, BlockKind::Text | BlockKind::Callout);
    let keeps_code = new_kind == BlockKind::Code;
    let keeps_asset =
        new_kind.asset_kind().is_some() && current.kind.asset_kind() == new_kind.asset_kind();
    let keeps_meta = new_kind == BlockKind::Quiz;

    conn.execute(
        "UPDATE block SET kind = ?2, body_md = ?3, code_text = ?4, code_lang = ?5,
                asset_id = ?6, meta_json = ?7, updated_at = ?8 WHERE id = ?1",
        params![
            id,
            new_kind.as_str(),
            keeps_body.then_some(current.body_md).flatten(),
            keeps_code.then_some(current.code_text).flatten(),
            if keeps_code {
                current.code_lang.or_else(|| Some("rust".into()))
            } else {
                current.code_lang
            },
            keeps_asset.then_some(current.asset_id).flatten(),
            if keeps_meta {
                current.meta_json
            } else {
                "{}".to_string()
            },
            now_ms()
        ],
    )?;
    search::index_block(conn, id)?;
    get(conn, ctx, id)
}

pub fn set_asset(
    conn: &Connection,
    ctx: &ReadCtx<'_>,
    block_id: &str,
    asset_id: Option<&str>,
) -> Result<BlockView> {
    let block = get_row(conn, block_id)?;
    if !block.kind.is_asset_backed() {
        return Err(CoreError::invalid(format!(
            "A {} block does not hold a file.",
            block.kind.as_str()
        )));
    }
    if let Some(aid) = asset_id {
        let kind: Option<String> = conn
            .query_row("SELECT kind FROM asset WHERE id = ?1", [aid], |r| r.get(0))
            .optional()?;
        let kind = kind.ok_or_else(|| CoreError::not_found("asset", aid))?;
        let want = block
            .kind
            .asset_kind()
            .map(|k| k.as_str())
            .unwrap_or("file");
        // An attachment block accepts anything; video/image blocks are strict.
        if want != "file" && kind != want {
            return Err(CoreError::invalid(format!(
                "That file is a {kind}, but this block expects a {want}."
            )));
        }
    }
    conn.execute(
        "UPDATE block SET asset_id = ?2, updated_at = ?3 WHERE id = ?1",
        params![block_id, asset_id, now_ms()],
    )?;
    get(conn, ctx, block_id)
}

pub fn soft_delete(conn: &Connection, id: &str) -> Result<()> {
    let n = conn.execute(
        "UPDATE block SET deleted_at = ?2, updated_at = ?2 WHERE id = ?1 AND deleted_at IS NULL",
        params![id, now_ms()],
    )?;
    if n == 0 {
        return Err(CoreError::not_found("block", id));
    }
    search::remove(conn, id)?;
    Ok(())
}

pub fn restore(conn: &Connection, id: &str) -> Result<()> {
    let n = conn.execute(
        "UPDATE block SET deleted_at = NULL, updated_at = ?2 WHERE id = ?1",
        params![id, now_ms()],
    )?;
    if n == 0 {
        return Err(CoreError::not_found("block", id));
    }
    search::index_block(conn, id)?;
    Ok(())
}

/// Permanent removal. Asset *files* are never touched here — only `asset_gc`
/// deletes bytes, and only after an explicit confirmation.
pub fn purge(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM block WHERE id = ?1", [id])?;
    search::remove(conn, id)?;
    Ok(())
}

pub fn list_deleted(conn: &Connection) -> Result<Vec<Block>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {BLOCK_COLS} FROM block WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC"
    ))?;
    let v = stmt
        .query_map([], map_block)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(v)
}

/// State histogram for one chapter, used by the sidebar without loading bodies.
pub fn state_counts(
    conn: &Connection,
    ctx: &ReadCtx<'_>,
    chapter_id: &str,
) -> Result<(u32, u32, u32, u32, u32)> {
    let sql = format!(
        "{} WHERE b.chapter_id = ?1 AND b.deleted_at IS NULL",
        joined_select()
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map([chapter_id], |r| row_to_view(r, ctx))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let (mut total, mut ready, mut empty, mut missing) = (0u32, 0u32, 0u32, 0u32);
    for (b, _, status) in &rows {
        total += 1;
        match state::compute(b, *status, ctx.importing_blocks.contains(&b.id)) {
            BlockState::Ready => ready += 1,
            BlockState::Empty => empty += 1,
            BlockState::Missing => missing += 1,
            BlockState::Importing => {}
        }
    }
    let completed: u32 = conn.query_row(
        "SELECT count(*) FROM progress p JOIN block b ON b.id = p.block_id
         WHERE b.chapter_id = ?1 AND b.deleted_at IS NULL AND p.completed = 1",
        [chapter_id],
        |r| r.get::<_, i64>(0),
    )? as u32;
    Ok((total, ready, empty, missing, completed))
}
