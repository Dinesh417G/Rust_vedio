use super::{map_asset, AssetRow, ASSET_COLS};
use crate::error::{CoreError, Result};
use crate::models::{AssetKind, AssetView, ImportMode};
use crate::paths::Paths;
use rusqlite::{params, Connection, OptionalExtension};

pub fn get_row(conn: &Connection, id: &str) -> Result<AssetRow> {
    conn.query_row(
        &format!("SELECT {ASSET_COLS} FROM asset WHERE id = ?1"),
        [id],
        map_asset,
    )
    .optional()?
    .ok_or_else(|| CoreError::not_found("asset", id))
}

pub fn get(conn: &Connection, paths: &Paths, id: &str) -> Result<AssetView> {
    Ok(get_row(conn, id)?.into_view(paths))
}

pub fn list_rows(conn: &Connection) -> Result<Vec<AssetRow>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {ASSET_COLS} FROM asset ORDER BY imported_at DESC"
    ))?;
    let rows = stmt
        .query_map([], map_asset)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

/// Dedupe key: identical content *and* identical length. Hash alone would be
/// enough in practice; the length check makes a collision impossible to act on.
pub fn find_by_hash(conn: &Connection, sha256: &str, bytes: i64) -> Result<Option<AssetRow>> {
    Ok(conn
        .query_row(
            &format!("SELECT {ASSET_COLS} FROM asset WHERE sha256 = ?1 AND bytes = ?2 LIMIT 1"),
            params![sha256, bytes],
            map_asset,
        )
        .optional()?)
}

/// Cheap dedupe pre-filter: is any asset exactly this many bytes?
///
/// Hashing a multi-gigabyte file costs a full read. Byte length costs an index
/// lookup, and a mismatch rules out a duplicate outright.
pub fn any_with_size(conn: &Connection, bytes: i64) -> Result<bool> {
    Ok(conn
        .query_row(
            "SELECT 1 FROM asset WHERE bytes = ?1 LIMIT 1",
            [bytes],
            |_| Ok(true),
        )
        .optional()?
        .unwrap_or(false))
}

#[allow(clippy::too_many_arguments)]
pub fn insert(
    conn: &Connection,
    id: &str,
    kind: AssetKind,
    original_name: &str,
    rel_path: &str,
    abs_path: Option<&str>,
    import_mode: ImportMode,
    mime: Option<&str>,
    bytes: i64,
    sha256: &str,
    imported_at: i64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO asset (id, kind, original_name, rel_path, abs_path, import_mode, mime,
                            bytes, sha256, imported_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            id,
            kind.as_str(),
            original_name,
            rel_path,
            abs_path,
            import_mode.as_str(),
            mime,
            bytes,
            sha256,
            imported_at
        ],
    )?;
    Ok(())
}

/// Media dimensions and duration, measured by the webview after import.
pub fn set_media_meta(
    conn: &Connection,
    id: &str,
    duration_secs: Option<f64>,
    width: Option<i64>,
    height: Option<i64>,
) -> Result<()> {
    let duration = duration_secs.filter(|d| d.is_finite() && *d > 0.0);
    let n = conn.execute(
        "UPDATE asset SET duration_secs = COALESCE(?2, duration_secs),
                          width = COALESCE(?3, width),
                          height = COALESCE(?4, height)
         WHERE id = ?1",
        params![
            id,
            duration,
            width.filter(|w| *w > 0),
            height.filter(|h| *h > 0)
        ],
    )?;
    if n == 0 {
        return Err(CoreError::not_found("asset", id));
    }
    Ok(())
}

pub fn set_thumb(conn: &Connection, id: &str, thumb_rel_path: &str) -> Result<()> {
    let n = conn.execute(
        "UPDATE asset SET thumb_rel_path = ?2 WHERE id = ?1",
        params![id, thumb_rel_path],
    )?;
    if n == 0 {
        return Err(CoreError::not_found("asset", id));
    }
    Ok(())
}

/// Points a `reference` asset at a file that has moved, or converts a `copy`
/// asset whose managed file vanished into a reference to the recovered file.
pub fn relink(conn: &Connection, paths: &Paths, id: &str, new_path: &str) -> Result<AssetView> {
    let p = std::path::Path::new(new_path);
    if !p.is_file() {
        return Err(CoreError::invalid(format!(
            "`{new_path}` is not a readable file."
        )));
    }
    let row = get_row(conn, id)?;
    let len = p.metadata()?.len() as i64;
    if len != row.bytes {
        tracing::warn!(
            asset = id,
            expected = row.bytes,
            found = len,
            "relinked file has a different size"
        );
    }
    conn.execute(
        "UPDATE asset SET abs_path = ?2, import_mode = 'reference', bytes = ?3 WHERE id = ?1",
        params![id, new_path, len],
    )?;
    get(conn, paths, id)
}

/// Which blocks currently point at this asset.
pub fn referencing_blocks(conn: &Connection, asset_id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT id FROM block WHERE asset_id = ?1")?;
    let rows = stmt
        .query_map([asset_id], |r| r.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn delete_row(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM asset WHERE id = ?1", [id])?;
    Ok(())
}

/// Total bytes tracked by the database (not what is on disk — see `gc`).
pub fn total_bytes(conn: &Connection) -> Result<i64> {
    Ok(conn.query_row(
        "SELECT COALESCE(sum(bytes), 0) FROM asset WHERE import_mode = 'copy'",
        [],
        |r| r.get(0),
    )?)
}
