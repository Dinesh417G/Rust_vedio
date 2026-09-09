//! `.rcbundle` — a ZIP holding `manifest.json` (schema version + a full JSON
//! dump of every table) plus the `media/` tree. This is the backup format and
//! the hand-off format.

use crate::error::{CoreError, Result};
use crate::ids::new_id;
use crate::paths::{self, Paths};
use crate::repo::search;
use crate::time::now_ms;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use std::path::Path;

pub const SCHEMA_VERSION: u32 = 1;
pub const MANIFEST_NAME: &str = "manifest.json";

/// Every table dumped, in an order that satisfies the foreign keys on import.
const TABLES: &[&str] = &[
    "asset", "course", "chapter", "block", "progress", "marker", "note",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BundleMode {
    /// Bring the bundle in alongside what is already here, with fresh IDs.
    Merge,
    /// Wipe the current course first.
    Replace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub schema_version: u32,
    pub app_version: String,
    pub exported_at: i64,
    pub includes_media: bool,
    pub tables: HashMap<String, Vec<Map<String, Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExportReport {
    pub path: String,
    pub bytes: i64,
    pub chapters: u32,
    pub blocks: u32,
    pub assets: u32,
    pub media_files: u32,
    pub media_skipped: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BundleImportReport {
    pub chapters: u32,
    pub blocks: u32,
    pub assets: u32,
    pub media_files: u32,
    pub replaced: bool,
    pub warnings: Vec<String>,
}

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

fn dump_table(conn: &Connection, table: &str) -> Result<Vec<Map<String, Value>>> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM {table}"))?;
    let names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();
    let rows = stmt.query_map([], |r| {
        let mut m = Map::new();
        for (i, name) in names.iter().enumerate() {
            let v = match r.get_ref(i)? {
                rusqlite::types::ValueRef::Null => Value::Null,
                rusqlite::types::ValueRef::Integer(i) => Value::from(i),
                rusqlite::types::ValueRef::Real(f) => Value::from(f),
                rusqlite::types::ValueRef::Text(t) => {
                    Value::from(String::from_utf8_lossy(t).to_string())
                }
                rusqlite::types::ValueRef::Blob(b) => {
                    Value::from(b.iter().map(|x| *x as u64).collect::<Vec<_>>())
                }
            };
            m.insert(name.clone(), v);
        }
        Ok(m)
    })?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

pub fn export(
    conn: &Connection,
    p: &Paths,
    dest: &Path,
    include_media: bool,
    app_version: &str,
) -> Result<ExportReport> {
    let mut tables = HashMap::new();
    for t in TABLES {
        tables.insert((*t).to_string(), dump_table(conn, t)?);
    }
    let manifest = Manifest {
        schema_version: SCHEMA_VERSION,
        app_version: app_version.to_string(),
        exported_at: now_ms(),
        includes_media: include_media,
        tables,
    };

    let mut report = ExportReport {
        path: paths::display_path(dest),
        chapters: manifest.tables.get("chapter").map(|v| v.len()).unwrap_or(0) as u32,
        blocks: manifest.tables.get("block").map(|v| v.len()).unwrap_or(0) as u32,
        assets: manifest.tables.get("asset").map(|v| v.len()).unwrap_or(0) as u32,
        ..Default::default()
    };

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(paths::long_path(parent))?;
    }
    let file = std::fs::File::create(paths::long_path(dest))?;
    let mut zip = zip::ZipWriter::new(file);
    let opts: zip::write::FileOptions<'_, ()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    // Media is already compressed; re-deflating a 2 GB MP4 wastes minutes.
    let stored: zip::write::FileOptions<'_, ()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file(MANIFEST_NAME, opts)?;
    zip.write_all(&serde_json::to_vec_pretty(&manifest)?)?;

    if include_media {
        for row in crate::repo::asset::list_rows(conn)? {
            let abs = row.absolute(p);
            if !abs.is_file() {
                report.media_skipped.push(row.original_name.clone());
                continue;
            }
            let name = format!("media/{}", row.rel_path.replace('\\', "/"));
            zip.start_file(&name, stored)?;
            stream_into(&abs, &mut zip)?;
            report.media_files += 1;

            if let Some(t) = &row.thumb_rel_path {
                let tp = p.resolve_rel(t);
                if tp.is_file() {
                    zip.start_file(format!("media/{}", t.replace('\\', "/")), stored)?;
                    stream_into(&tp, &mut zip)?;
                }
            }
        }
    }

    zip.finish()?;
    report.bytes = std::fs::metadata(paths::long_path(dest))
        .map(|m| m.len() as i64)
        .unwrap_or(0);
    Ok(report)
}

fn stream_into<W: Write + Seek>(src: &Path, zip: &mut zip::ZipWriter<W>) -> Result<()> {
    let mut f = std::fs::File::open(paths::long_path(src))?;
    let mut buf = vec![0u8; 1024 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        zip.write_all(&buf[..n])?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Import
// ---------------------------------------------------------------------------

fn as_str(m: &Map<String, Value>, k: &str) -> Option<String> {
    m.get(k).and_then(|v| v.as_str()).map(String::from)
}

pub fn import(
    conn: &mut Connection,
    p: &Paths,
    src: &Path,
    mode: BundleMode,
) -> Result<BundleImportReport> {
    let file = std::fs::File::open(paths::long_path(src))?;
    let mut zip = zip::ZipArchive::new(file)?;
    let manifest: Manifest = {
        let mut f = zip
            .by_name(MANIFEST_NAME)
            .map_err(|_| CoreError::invalid("That file is not a RustClass bundle."))?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        serde_json::from_str(&s)
            .map_err(|e| CoreError::invalid(format!("The bundle manifest is unreadable: {e}")))?
    };
    if manifest.schema_version > SCHEMA_VERSION {
        return Err(CoreError::invalid(format!(
            "This bundle was written by a newer RustClass (bundle schema v{}, this build reads v{SCHEMA_VERSION}).",
            manifest.schema_version
        )));
    }

    let mut report = BundleImportReport {
        replaced: mode == BundleMode::Replace,
        ..Default::default()
    };

    // Fresh IDs everywhere, so a merge can never collide with existing rows and
    // a replace still produces a self-consistent tree.
    let mut id_map: HashMap<String, String> = HashMap::new();

    let empty = Vec::new();
    let assets = manifest.tables.get("asset").unwrap_or(&empty);
    let chapters = manifest.tables.get("chapter").unwrap_or(&empty);
    let blocks = manifest.tables.get("block").unwrap_or(&empty);
    let progress = manifest.tables.get("progress").unwrap_or(&empty);
    let markers = manifest.tables.get("marker").unwrap_or(&empty);
    let notes = manifest.tables.get("note").unwrap_or(&empty);
    let courses = manifest.tables.get("course").unwrap_or(&empty);

    // Media first: a failed extraction should not leave half a course behind.
    if manifest.includes_media {
        for a in assets {
            let Some(old_id) = as_str(a, "id") else {
                continue;
            };
            let new_asset_id = id_map.entry(old_id).or_insert_with(new_id).clone();
            let Some(old_rel) = as_str(a, "rel_path") else {
                continue;
            };
            let ext = paths::extension_of(&old_rel);
            let new_rel = match &ext {
                Some(e) => format!(
                    "{}/{new_asset_id}.{e}",
                    old_rel.split('/').next().unwrap_or("imported")
                ),
                None => format!(
                    "{}/{new_asset_id}",
                    old_rel.split('/').next().unwrap_or("imported")
                ),
            };
            let zip_name = format!("media/{old_rel}");
            match zip.by_name(&zip_name) {
                Ok(mut entry) => {
                    let dest = p.resolve_rel(&new_rel);
                    p.guard_inside(&dest)?;
                    if let Some(parent) = dest.parent() {
                        std::fs::create_dir_all(paths::long_path(parent))?;
                    }
                    let mut out = std::fs::File::create(paths::long_path(&dest))?;
                    std::io::copy(&mut entry, &mut out)?;
                    report.media_files += 1;
                    id_map.insert(format!("rel::{old_rel}"), new_rel);
                }
                Err(_) => {
                    report.warnings.push(format!(
                        "`{}` was listed in the bundle but its file was not inside it.",
                        as_str(a, "original_name").unwrap_or_else(|| old_rel.clone())
                    ));
                    id_map.insert(format!("rel::{old_rel}"), new_rel);
                }
            }
        }
    } else {
        for a in assets {
            let Some(old_id) = as_str(a, "id") else {
                continue;
            };
            id_map.entry(old_id).or_insert_with(new_id);
            if let Some(old_rel) = as_str(a, "rel_path") {
                id_map.insert(format!("rel::{old_rel}"), old_rel);
            }
        }
        report
            .warnings
            .push("This bundle carries no media, so video and image blocks will show as missing until you relink them.".into());
    }

    let tx = conn.transaction()?;

    if mode == BundleMode::Replace {
        // Assets are not deleted here — files on disk are only ever removed by
        // `asset_gc`, and only after the user confirms the list.
        tx.execute("DELETE FROM note", [])?;
        tx.execute("DELETE FROM marker", [])?;
        tx.execute("DELETE FROM progress", [])?;
        tx.execute("DELETE FROM block", [])?;
        tx.execute("DELETE FROM chapter", [])?;
    }

    let course_id: String = tx.query_row(
        "SELECT id FROM course ORDER BY created_at LIMIT 1",
        [],
        |r| r.get(0),
    )?;
    if mode == BundleMode::Replace {
        if let Some(c) = courses.first() {
            tx.execute(
                "UPDATE course SET title = ?2, subtitle = ?3, author = ?4, description_md = ?5,
                        updated_at = ?6 WHERE id = ?1",
                params![
                    &course_id,
                    as_str(c, "title").unwrap_or_else(|| "Imported course".into()),
                    as_str(c, "subtitle"),
                    as_str(c, "author"),
                    as_str(c, "description_md"),
                    now_ms()
                ],
            )?;
        }
    }

    for a in assets {
        let Some(old_id) = as_str(a, "id") else {
            continue;
        };
        let new_id_ = id_map.get(&old_id).cloned().unwrap_or_else(new_id);
        let old_rel = as_str(a, "rel_path").unwrap_or_default();
        let new_rel = id_map
            .get(&format!("rel::{old_rel}"))
            .cloned()
            .unwrap_or(old_rel);
        tx.execute(
            "INSERT INTO asset (id, kind, original_name, rel_path, abs_path, import_mode, mime,
                                bytes, sha256, duration_secs, width, height, thumb_rel_path,
                                imported_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                &new_id_,
                as_str(a, "kind").unwrap_or_else(|| "file".into()),
                as_str(a, "original_name").unwrap_or_else(|| "file".into()),
                new_rel,
                // A reference import only makes sense on the machine that made
                // it; on import it becomes a managed copy or a missing file.
                Option::<String>::None,
                "copy",
                as_str(a, "mime"),
                a.get("bytes").and_then(|v| v.as_i64()).unwrap_or(0),
                as_str(a, "sha256").unwrap_or_default(),
                a.get("duration_secs").and_then(|v| v.as_f64()),
                a.get("width").and_then(|v| v.as_i64()),
                a.get("height").and_then(|v| v.as_i64()),
                as_str(a, "thumb_rel_path"),
                a.get("imported_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
            ],
        )?;
        report.assets += 1;
    }

    // Appended chapters go after everything already present.
    let base: f64 = tx.query_row(
        "SELECT COALESCE(max(sort_index), 0) FROM chapter",
        [],
        |r| r.get(0),
    )?;

    for (i, c) in chapters.iter().enumerate() {
        let Some(old_id) = as_str(c, "id") else {
            continue;
        };
        let new_id_ = id_map.entry(old_id).or_insert_with(new_id).clone();
        tx.execute(
            "INSERT INTO chapter (id, course_id, display_number, title, summary_md, objectives_md,
                                  estimated_minutes, status, sort_index, created_at, updated_at,
                                  deleted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                &new_id_,
                &course_id,
                as_str(c, "display_number"),
                as_str(c, "title").unwrap_or_else(|| "Untitled chapter".into()),
                as_str(c, "summary_md"),
                as_str(c, "objectives_md"),
                c.get("estimated_minutes").and_then(|v| v.as_i64()),
                as_str(c, "status").unwrap_or_else(|| "draft".into()),
                base + (i as f64 + 1.0) * crate::sortindex::STEP,
                c.get("created_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
                c.get("updated_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
                c.get("deleted_at").and_then(|v| v.as_i64()),
            ],
        )?;
        report.chapters += 1;
    }

    for b in blocks {
        let Some(old_id) = as_str(b, "id") else {
            continue;
        };
        let Some(old_chapter) = as_str(b, "chapter_id") else {
            continue;
        };
        let Some(new_chapter) = id_map.get(&old_chapter).cloned() else {
            report.warnings.push(format!(
                "A block referenced chapter `{old_chapter}`, which the bundle does not contain."
            ));
            continue;
        };
        let new_id_ = id_map.entry(old_id).or_insert_with(new_id).clone();
        let new_asset = as_str(b, "asset_id").and_then(|a| id_map.get(&a).cloned());
        tx.execute(
            "INSERT INTO block (id, chapter_id, kind, heading, caption_md, body_md, code_text,
                                code_lang, asset_id, meta_json, sort_index, created_at, updated_at,
                                deleted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                &new_id_,
                new_chapter,
                as_str(b, "kind").unwrap_or_else(|| "text".into()),
                as_str(b, "heading"),
                as_str(b, "caption_md"),
                as_str(b, "body_md"),
                as_str(b, "code_text"),
                as_str(b, "code_lang"),
                new_asset,
                as_str(b, "meta_json").unwrap_or_else(|| "{}".into()),
                b.get("sort_index")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1024.0),
                b.get("created_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
                b.get("updated_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
                b.get("deleted_at").and_then(|v| v.as_i64()),
            ],
        )?;
        report.blocks += 1;
    }

    for pr in progress {
        let Some(new_block) = as_str(pr, "block_id").and_then(|b| id_map.get(&b).cloned()) else {
            continue;
        };
        tx.execute(
            "INSERT OR REPLACE INTO progress (block_id, completed, last_position_secs,
                                              watched_secs, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                new_block,
                pr.get("completed").and_then(|v| v.as_i64()).unwrap_or(0),
                pr.get("last_position_secs")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                pr.get("watched_secs")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                pr.get("updated_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
            ],
        )?;
    }

    for m in markers {
        let Some(new_block) = as_str(m, "block_id").and_then(|b| id_map.get(&b).cloned()) else {
            continue;
        };
        tx.execute(
            "INSERT INTO marker (id, block_id, at_secs, label, note_md) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                new_id(),
                new_block,
                m.get("at_secs").and_then(|v| v.as_f64()).unwrap_or(0.0),
                as_str(m, "label").unwrap_or_else(|| "Marker".into()),
                as_str(m, "note_md"),
            ],
        )?;
    }

    for n in notes {
        let Some(new_chapter) = as_str(n, "chapter_id").and_then(|c| id_map.get(&c).cloned())
        else {
            continue;
        };
        tx.execute(
            "INSERT INTO note (id, chapter_id, body_md, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                new_id(),
                new_chapter,
                as_str(n, "body_md").unwrap_or_default(),
                n.get("created_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
                n.get("updated_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(now_ms()),
            ],
        )?;
    }

    tx.commit()?;
    search::reindex_all(conn)?;
    Ok(report)
}
