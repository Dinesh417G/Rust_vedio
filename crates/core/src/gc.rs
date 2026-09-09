//! Garbage collection: the *only* code path in the app that deletes bytes, and
//! it never runs without an explicit confirmation of exactly what will go.

use crate::error::Result;
use crate::integrity;
use crate::models::{GcEntry, GcReport};
use crate::paths::{self, Paths};
use crate::repo::asset as asset_repo;
use rusqlite::Connection;
use std::collections::HashSet;

pub fn run(conn: &Connection, p: &Paths, dry_run: bool) -> Result<GcReport> {
    let mut report = GcReport {
        dry_run,
        ..Default::default()
    };

    let rows = asset_repo::list_rows(conn)?;
    let mut known: HashSet<String> = HashSet::new();
    for row in &rows {
        known.insert(paths::display_path(&row.absolute(p)));
        if let Some(t) = &row.thumb_rel_path {
            known.insert(paths::display_path(&p.resolve_rel(t)));
        }
    }

    // Asset rows nothing points at any more.
    for row in &rows {
        let refs = asset_repo::referencing_blocks(conn, &row.id)?;
        let is_cover: bool = conn.query_row(
            "SELECT count(*) FROM course WHERE cover_asset_id = ?1",
            [&row.id],
            |r| r.get::<_, i64>(0),
        )? > 0;
        if refs.is_empty() && !is_cover {
            report.unreferenced_assets.push(GcEntry {
                id: row.id.clone(),
                label: row.original_name.clone(),
                path: paths::display_path(&row.absolute(p)),
                bytes: row.bytes,
            });
        }
    }

    // Files under the media root that no asset row claims.
    for file in integrity::walk_media(p)? {
        let display = paths::display_path(&file);
        if display.ends_with(crate::assets::PART_SUFFIX) {
            report.partial_files.push(GcEntry {
                id: display.clone(),
                label: file
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default(),
                path: display,
                bytes: file.metadata().map(|m| m.len() as i64).unwrap_or(0),
            });
            continue;
        }
        if !known.contains(&display) {
            report.orphan_files.push(GcEntry {
                id: display.clone(),
                label: file
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default(),
                path: display,
                bytes: file.metadata().map(|m| m.len() as i64).unwrap_or(0),
            });
        }
    }

    report.bytes_reclaimable = report
        .orphan_files
        .iter()
        .chain(&report.partial_files)
        .chain(&report.unreferenced_assets)
        .map(|e| e.bytes)
        .sum();

    if dry_run {
        return Ok(report);
    }

    for entry in report.orphan_files.iter().chain(&report.partial_files) {
        let path = std::path::PathBuf::from(&entry.path);
        // Belt and braces: never unlink anything outside our own folder.
        p.guard_inside(&path)?;
        if let Err(e) = std::fs::remove_file(paths::long_path(&path)) {
            tracing::warn!(path = %entry.path, error = %e, "could not remove orphan file");
        }
    }
    for entry in &report.unreferenced_assets {
        if let Ok(row) = asset_repo::get_row(conn, &entry.id) {
            // A `reference` import points at the user's own file. Drop the row,
            // never the file.
            if row.import_mode == crate::models::ImportMode::Copy {
                let abs = row.absolute(p);
                if p.guard_inside(&abs).is_ok() {
                    let _ = std::fs::remove_file(paths::long_path(&abs));
                }
            }
            if let Some(t) = &row.thumb_rel_path {
                let tp = p.resolve_rel(t);
                if p.guard_inside(&tp).is_ok() {
                    let _ = std::fs::remove_file(paths::long_path(&tp));
                }
            }
        }
        asset_repo::delete_row(conn, &entry.id)?;
    }
    Ok(report)
}

/// Deletes `*.part` leftovers at startup. Silent — the user never asked for
/// these files to exist in the first place.
pub fn sweep_partials(p: &Paths) -> Result<u32> {
    let mut n = 0;
    for part in integrity::scan_partials(p)? {
        if p.guard_inside(&part).is_ok() && std::fs::remove_file(paths::long_path(&part)).is_ok() {
            n += 1;
        }
    }
    if n > 0 {
        tracing::info!(count = n, "cleaned up partial imports from a previous run");
    }
    Ok(n)
}

/// Chapters and blocks soft-deleted longer ago than the retention window.
pub fn expired_trash(conn: &Connection, older_than_ms: i64) -> Result<(Vec<String>, Vec<String>)> {
    let cutoff = crate::time::now_ms() - older_than_ms;
    let chapters: Vec<String> = {
        let mut s = conn
            .prepare("SELECT id FROM chapter WHERE deleted_at IS NOT NULL AND deleted_at < ?1")?;
        let out = s
            .query_map([cutoff], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        out
    };
    let blocks: Vec<String> = {
        let mut s =
            conn.prepare("SELECT id FROM block WHERE deleted_at IS NOT NULL AND deleted_at < ?1")?;
        let out = s
            .query_map([cutoff], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        out
    };
    Ok((chapters, blocks))
}

/// 30 days, per the Trash spec.
pub const TRASH_RETENTION_MS: i64 = 30 * 24 * 60 * 60 * 1000;
