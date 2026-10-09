//! RustClass domain core.
//!
//! Everything that touches the database or the filesystem lives here, with no
//! dependency on Tauri, so the whole product can be exercised by `cargo test`
//! on any platform. `src-tauri` is a thin adapter over this crate.

pub mod assets;
pub mod bundle;
pub mod db;
pub mod error;
pub mod gc;
pub mod ids;
pub mod integrity;
pub mod models;
pub mod outline;
pub mod paths;
pub mod repo;
pub mod sortindex;
pub mod state;
pub mod templates;
pub mod time;

pub use error::{CoreError, Result};

use models::*;
use rusqlite::Connection;

/// Applies a parsed outline to the database in one transaction.
pub fn apply_outline(
    conn: &mut Connection,
    parsed: &outline::ParsedOutline,
    mode: outline::OutlineMode,
) -> Result<outline::ImportReport> {
    let course = repo::course::get_or_seed(conn)?;
    let mut report = outline::ImportReport {
        chapters_created: 0,
        blocks_created: 0,
        chapters_removed: 0,
        course_title_changed: false,
        warnings: parsed.warnings.clone(),
    };

    let tx = conn.transaction()?;

    if mode == outline::OutlineMode::Replace {
        let now = time::now_ms();
        report.chapters_removed = tx.execute(
            "UPDATE chapter SET deleted_at = ?1, updated_at = ?1 WHERE deleted_at IS NULL",
            [now],
        )? as u32;
        tx.execute("DELETE FROM search_idx WHERE entity_type = 'chapter'", [])?;
    }

    if let Some(title) = &parsed.course_title {
        if !title.trim().is_empty() && title.trim() != course.title {
            tx.execute(
                "UPDATE course SET title = ?2, updated_at = ?3 WHERE id = ?1",
                rusqlite::params![&course.id, title.trim(), time::now_ms()],
            )?;
            report.course_title_changed = true;
        }
    }

    let mut after: Option<String> = None;
    for pc in &parsed.chapters {
        let req = NewChapter {
            title: pc.title.clone(),
            display_number: pc.display_number.clone(),
            summary_md: pc.summary_md.clone(),
            objectives_md: None,
            estimated_minutes: None,
            template: None,
            after_id: after.clone(),
            part: None,
            section: None,
        };
        let chapter_id = repo::chapter::insert_in_tx(&tx, &course.id, &req)?;
        report.chapters_created += 1;
        let mut prev_block: Option<String> = None;
        for pb in &pc.blocks {
            let bid = repo::block::insert_in_tx(
                &tx,
                &chapter_id,
                pb.kind,
                prev_block.as_deref(),
                Some(outline::seed_for(pb)),
            )?;
            report.blocks_created += 1;
            prev_block = Some(bid);
        }
        after = Some(chapter_id);
    }

    tx.commit()?;
    Ok(report)
}

/// Numbers and sizes for the settings and health screens.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub app_version: String,
    pub schema_version: i64,
    pub bundle_schema_version: u32,
    pub app_data_dir: String,
    pub db_path: String,
    pub media_dir: String,
    pub log_dir: String,
    pub db_bytes: i64,
    pub media_bytes: i64,
    pub asset_bytes_tracked: i64,
    pub chapter_count: u32,
    pub block_count: u32,
    pub asset_count: u32,
    pub trashed_chapters: u32,
    pub trashed_blocks: u32,
}

/// Total bytes the database occupies, sidecars included.
///
/// In WAL mode most recent writes live in `-wal` until a checkpoint, so the
/// main file alone can read as a few kilobytes for a course with ten chapters
/// in it. Reporting that would be actively misleading.
fn db_bytes_on_disk(p: &paths::Paths) -> i64 {
    let main = p.db_file();
    ["", "-wal", "-shm"]
        .iter()
        .map(|suffix| {
            let mut name = main.as_os_str().to_os_string();
            name.push(suffix);
            std::fs::metadata(std::path::PathBuf::from(name))
                .map(|m| m.len() as i64)
                .unwrap_or(0)
        })
        .sum()
}

pub fn system_info(conn: &Connection, p: &paths::Paths, app_version: &str) -> Result<SystemInfo> {
    let count =
        |sql: &str| -> Result<u32> { Ok(conn.query_row(sql, [], |r| r.get::<_, i64>(0))? as u32) };
    let media_bytes: i64 = integrity::walk_media(p)?
        .iter()
        .filter_map(|f| f.metadata().ok())
        .map(|m| m.len() as i64)
        .sum();
    Ok(SystemInfo {
        app_version: app_version.to_string(),
        schema_version: db::migrations::current_version(conn)?,
        bundle_schema_version: bundle::SCHEMA_VERSION,
        app_data_dir: paths::display_path(&p.app_data),
        db_path: paths::display_path(&p.db_file()),
        media_dir: paths::display_path(&p.media_root()),
        log_dir: paths::display_path(&p.logs_root()),
        db_bytes: db_bytes_on_disk(p),
        media_bytes,
        asset_bytes_tracked: repo::asset::total_bytes(conn)?,
        chapter_count: count("SELECT count(*) FROM chapter WHERE deleted_at IS NULL")?,
        block_count: count("SELECT count(*) FROM block WHERE deleted_at IS NULL")?,
        asset_count: count("SELECT count(*) FROM asset")?,
        trashed_chapters: count("SELECT count(*) FROM chapter WHERE deleted_at IS NOT NULL")?,
        trashed_blocks: count("SELECT count(*) FROM block WHERE deleted_at IS NOT NULL")?,
    })
}
