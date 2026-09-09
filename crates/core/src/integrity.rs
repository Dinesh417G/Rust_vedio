//! The report behind the "3 files are missing" banner and the `/health` page.

use crate::error::Result;
use crate::models::IntegrityIssue;
use crate::paths::{self, Paths};
use crate::repo::asset as asset_repo;
use rusqlite::Connection;

pub fn check(conn: &Connection, p: &Paths) -> Result<Vec<IntegrityIssue>> {
    let mut issues = Vec::new();

    for row in asset_repo::list_rows(conn)? {
        let abs = row.absolute(p);
        if !abs.is_file() {
            let blocks = asset_repo::referencing_blocks(conn, &row.id)?;
            issues.push(IntegrityIssue {
                kind: "missing_file".into(),
                severity: if blocks.is_empty() {
                    "warning"
                } else {
                    "error"
                }
                .into(),
                entity_id: row.id.clone(),
                entity_type: "asset".into(),
                message: format!(
                    "`{}` is not where RustClass expects it{}.",
                    row.original_name,
                    if blocks.is_empty() {
                        " (no block uses it)"
                    } else {
                        ""
                    }
                ),
                path: Some(paths::display_path(&abs)),
            });
            continue;
        }
        // A reference import whose original has been swapped for a different
        // file is still readable, but it is no longer what was imported.
        if let Ok(meta) = abs.metadata() {
            if meta.len() as i64 != row.bytes {
                issues.push(IntegrityIssue {
                    kind: "size_mismatch".into(),
                    severity: "warning".into(),
                    entity_id: row.id.clone(),
                    entity_type: "asset".into(),
                    message: format!(
                        "`{}` is {} bytes on disk but {} bytes were imported.",
                        row.original_name,
                        meta.len(),
                        row.bytes
                    ),
                    path: Some(paths::display_path(&abs)),
                });
            }
        }
    }

    // Blocks pointing at an asset row that no longer exists.
    let mut stmt = conn.prepare(
        "SELECT b.id, b.asset_id, b.heading, b.kind FROM block b
         LEFT JOIN asset a ON a.id = b.asset_id
         WHERE b.asset_id IS NOT NULL AND a.id IS NULL AND b.deleted_at IS NULL",
    )?;
    let dangling: Vec<(String, String, Option<String>, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);
    for (block_id, asset_id, heading, kind) in dangling {
        issues.push(IntegrityIssue {
            kind: "dangling_asset_ref".into(),
            severity: "error".into(),
            entity_id: block_id,
            entity_type: "block".into(),
            message: format!(
                "The {kind} block `{}` points at asset `{asset_id}`, which is not in the library.",
                heading.unwrap_or_else(|| "(untitled)".into())
            ),
            path: None,
        });
    }

    // Leftovers from an import that was killed mid-copy.
    for part in scan_partials(p)? {
        let bytes = part.metadata().map(|m| m.len() as i64).unwrap_or(0);
        issues.push(IntegrityIssue {
            kind: "partial_file".into(),
            severity: "warning".into(),
            entity_id: paths::display_path(&part),
            entity_type: "file".into(),
            message: format!(
                "An unfinished import left {} behind. Clean it up from Settings → Storage.",
                human_bytes(bytes)
            ),
            path: Some(paths::display_path(&part)),
        });
    }

    Ok(issues)
}

/// Every `*.part` file under the media root.
pub fn scan_partials(p: &Paths) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    walk(&p.media_root(), &mut |f| {
        if f.to_string_lossy().ends_with(crate::assets::PART_SUFFIX) {
            out.push(f.to_path_buf());
        }
    })?;
    Ok(out)
}

/// Every regular file under the media root.
pub fn walk_media(p: &Paths) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    walk(&p.media_root(), &mut |f| out.push(f.to_path_buf()))?;
    Ok(out)
}

fn walk(dir: &std::path::Path, f: &mut impl FnMut(&std::path::Path)) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(paths::long_path(dir))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk(&path, f)?;
        } else if path.is_file() {
            f(&path);
        }
    }
    Ok(())
}

pub fn human_bytes(n: i64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut v = n as f64;
    let mut u = 0;
    while v >= 1024.0 && u < UNITS.len() - 1 {
        v /= 1024.0;
        u += 1;
    }
    if u == 0 {
        format!("{n} B")
    } else {
        format!("{v:.1} {}", UNITS[u])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_formatting_reads_naturally() {
        assert_eq!(human_bytes(0), "0 B");
        assert_eq!(human_bytes(1023), "1023 B");
        assert_eq!(human_bytes(1024), "1.0 KB");
        assert_eq!(human_bytes(1_288_490_188), "1.2 GB");
    }
}
