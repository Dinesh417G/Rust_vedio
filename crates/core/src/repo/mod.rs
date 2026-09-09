pub mod asset;
pub mod block;
pub mod chapter;
pub mod course;
pub mod marker;
pub mod note;
pub mod progress;
pub mod search;

use crate::models::*;
use crate::paths::{self, Paths};
use rusqlite::Row;
use std::collections::HashSet;

/// Everything a read needs beyond the connection: where files live and which
/// blocks currently have an import in flight.
pub struct ReadCtx<'a> {
    pub paths: &'a Paths,
    pub importing_blocks: &'a HashSet<String>,
}

impl<'a> ReadCtx<'a> {
    pub fn new(paths: &'a Paths, importing_blocks: &'a HashSet<String>) -> Self {
        Self {
            paths,
            importing_blocks,
        }
    }
}

pub(crate) const CHAPTER_COLS: &str = "id, course_id, display_number, title, summary_md, \
     objectives_md, estimated_minutes, status, sort_index, created_at, updated_at, deleted_at";

pub(crate) const BLOCK_COLS: &str = "id, chapter_id, kind, heading, caption_md, body_md, \
     code_text, code_lang, asset_id, meta_json, sort_index, created_at, updated_at, deleted_at";

pub(crate) const ASSET_COLS: &str = "id, kind, original_name, rel_path, abs_path, import_mode, \
     mime, bytes, sha256, duration_secs, width, height, thumb_rel_path, imported_at";

pub(crate) fn map_chapter(r: &Row<'_>) -> rusqlite::Result<Chapter> {
    Ok(Chapter {
        id: r.get(0)?,
        course_id: r.get(1)?,
        display_number: r.get(2)?,
        title: r.get(3)?,
        summary_md: r.get(4)?,
        objectives_md: r.get(5)?,
        estimated_minutes: r.get(6)?,
        status: ChapterStatus::parse(&r.get::<_, String>(7)?),
        sort_index: r.get(8)?,
        created_at: r.get(9)?,
        updated_at: r.get(10)?,
        deleted_at: r.get(11)?,
    })
}

pub(crate) fn map_block(r: &Row<'_>) -> rusqlite::Result<Block> {
    let kind_str: String = r.get(2)?;
    Ok(Block {
        id: r.get(0)?,
        chapter_id: r.get(1)?,
        // An unknown kind from a hand-edited database degrades to text rather
        // than failing the whole chapter load.
        kind: BlockKind::parse(&kind_str).unwrap_or(BlockKind::Text),
        heading: r.get(3)?,
        caption_md: r.get(4)?,
        body_md: r.get(5)?,
        code_text: r.get(6)?,
        code_lang: r.get(7)?,
        asset_id: r.get(8)?,
        meta_json: r.get(9)?,
        sort_index: r.get(10)?,
        created_at: r.get(11)?,
        updated_at: r.get(12)?,
        deleted_at: r.get(13)?,
    })
}

/// A raw `asset` row before it is resolved against the filesystem.
pub struct AssetRow {
    pub id: String,
    pub kind: AssetKind,
    pub original_name: String,
    pub rel_path: String,
    pub abs_path: Option<String>,
    pub import_mode: ImportMode,
    pub mime: Option<String>,
    pub bytes: i64,
    pub sha256: String,
    pub duration_secs: Option<f64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumb_rel_path: Option<String>,
    pub imported_at: i64,
}

pub(crate) fn map_asset(r: &Row<'_>) -> rusqlite::Result<AssetRow> {
    map_asset_at(r, 0)
}

/// Same mapping, but starting at column `o` — lets a block query LEFT JOIN the
/// asset table and build both rows from one scan.
pub(crate) fn map_asset_at(r: &Row<'_>, o: usize) -> rusqlite::Result<AssetRow> {
    Ok(AssetRow {
        id: r.get(o)?,
        kind: AssetKind::parse(&r.get::<_, String>(o + 1)?).unwrap_or(AssetKind::File),
        original_name: r.get(o + 2)?,
        rel_path: r.get(o + 3)?,
        abs_path: r.get(o + 4)?,
        import_mode: ImportMode::parse(&r.get::<_, String>(o + 5)?).unwrap_or(ImportMode::Copy),
        mime: r.get(o + 6)?,
        bytes: r.get(o + 7)?,
        sha256: r.get(o + 8)?,
        duration_secs: r.get(o + 9)?,
        width: r.get(o + 10)?,
        height: r.get(o + 11)?,
        thumb_rel_path: r.get(o + 12)?,
        imported_at: r.get(o + 13)?,
    })
}

/// Asset columns prefixed for use inside a join.
pub(crate) fn asset_cols_prefixed(alias: &str) -> String {
    ASSET_COLS
        .split(", ")
        .map(|c| format!("{alias}.{}", c.trim()))
        .collect::<Vec<_>>()
        .join(", ")
}

impl AssetRow {
    /// Where the bytes actually live right now.
    pub fn absolute(&self, p: &Paths) -> std::path::PathBuf {
        match (self.import_mode, self.abs_path.as_deref()) {
            (ImportMode::Reference, Some(a)) => std::path::PathBuf::from(a),
            _ => p.resolve_rel(&self.rel_path),
        }
    }

    pub fn into_view(self, p: &Paths) -> AssetView {
        let abs = self.absolute(p);
        let file_exists = abs.is_file();
        let thumb_path = self
            .thumb_rel_path
            .as_deref()
            .map(|t| p.resolve_rel(t))
            .filter(|t| t.is_file())
            .map(|t| paths::display_path(&t));
        AssetView {
            id: self.id,
            kind: self.kind,
            original_name: self.original_name,
            rel_path: self.rel_path,
            abs_path: self.abs_path,
            import_mode: self.import_mode,
            mime: self.mime,
            bytes: self.bytes,
            sha256: self.sha256,
            duration_secs: self.duration_secs,
            width: self.width,
            height: self.height,
            thumb_rel_path: self.thumb_rel_path,
            imported_at: self.imported_at,
            file_exists,
            resolved_path: paths::display_path(&abs),
            thumb_path,
        }
    }
}
