//! The import pipeline: stream, hash, dedupe, and never block the UI thread.

use crate::error::{CoreError, Result};
use crate::ids::new_id;
use crate::models::{AssetKind, ImportMode};
use crate::paths::{self, Paths};
use crate::repo::asset as asset_repo;
use crate::time::{now_ms, year_month};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// 1 MiB — big enough to keep the syscall count low on a multi-GB file, small
/// enough that a cancel is felt immediately.
const CHUNK: usize = 1024 * 1024;
/// Progress events are throttled to this interval so the IPC channel and the
/// React render loop are not flooded during a fast local copy.
pub const PROGRESS_INTERVAL_MS: i64 = 100;
/// Partial copies carry this suffix so a crash leaves something identifiable.
pub const PART_SUFFIX: &str = ".part";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportRequest {
    pub path: String,
    /// Attach to this block once the import lands.
    #[serde(default)]
    pub block_id: Option<String>,
    #[serde(default)]
    pub kind: Option<AssetKind>,
    #[serde(default)]
    pub mode: ImportMode,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportTicket {
    pub ticket_id: String,
    pub block_id: Option<String>,
    pub original_name: String,
    pub bytes_total: i64,
    /// Set when the file was already in the library and nothing had to be copied.
    pub deduped_asset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportProgress {
    pub ticket_id: String,
    pub block_id: Option<String>,
    pub asset_id: Option<String>,
    pub bytes_done: i64,
    pub bytes_total: i64,
    pub pct: f64,
}

/// Cooperative cancellation shared between the command handler and the
/// blocking copy running on the thread pool.
#[derive(Debug, Clone, Default)]
pub struct CancelFlag(Arc<AtomicBool>);

impl CancelFlag {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

/// Best-effort kind inference from the extension, used when the caller does
/// not know (a drop onto an attachment block, say).
pub fn infer_kind(name: &str) -> AssetKind {
    match paths::extension_of(name).as_deref() {
        Some("mp4" | "m4v" | "webm" | "mkv" | "mov" | "avi" | "ogv" | "wmv" | "flv") => {
            AssetKind::Video
        }
        Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg" | "avif" | "tif" | "tiff") => {
            AssetKind::Image
        }
        _ => AssetKind::File,
    }
}

/// Hashes a file without copying it, reporting progress as it goes.
///
/// Used for `reference` imports and for the dedupe check when a same-sized
/// asset already exists. A multi-gigabyte read is not instant, so it reports
/// like any other phase rather than leaving the bar sitting at zero.
pub fn hash_file<F>(src: &Path, cancel: &CancelFlag, mut on_progress: F) -> Result<(String, i64)>
where
    F: FnMut(i64, i64),
{
    let expected = src.metadata().map(|m| m.len() as i64).unwrap_or(0);
    let mut f = std::fs::File::open(paths::long_path(src))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; CHUNK];
    let mut total: i64 = 0;
    let mut last_tick = now_ms();
    on_progress(0, expected);
    loop {
        if cancel.is_cancelled() {
            return Err(CoreError::Cancelled("Import cancelled.".into()));
        }
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        total += n as i64;
        let now = now_ms();
        if now - last_tick >= PROGRESS_INTERVAL_MS {
            last_tick = now;
            on_progress(total, expected);
        }
    }
    on_progress(total, expected);
    Ok((hex(&hasher.finalize()), total))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Where a copied asset lands: `media/<yyyy-mm>/<uuid>.<ext>`.
pub fn dest_rel_path(asset_id: &str, original_name: &str, at_ms: i64) -> String {
    match paths::extension_of(original_name) {
        Some(ext) => format!("{}/{asset_id}.{ext}", year_month(at_ms)),
        None => format!("{}/{asset_id}", year_month(at_ms)),
    }
}

/// Outcome of a streamed copy.
#[derive(Debug)]
pub struct CopyOutcome {
    pub sha256: String,
    pub bytes: i64,
}

/// Streams `src` to `dest` in 1 MiB chunks, hashing as it goes and reporting
/// progress at most every `PROGRESS_INTERVAL_MS`.
///
/// On cancel or any error the partial file is removed before returning, so a
/// failed import never leaves bytes behind for the user to find.
pub fn copy_streaming<F>(
    src: &Path,
    dest: &Path,
    total: i64,
    cancel: &CancelFlag,
    mut on_progress: F,
) -> Result<CopyOutcome>
where
    F: FnMut(i64, i64),
{
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(paths::long_path(parent))?;
    }
    let part = with_part_suffix(dest);
    let result = copy_inner(src, &part, total, cancel, &mut on_progress);
    match result {
        Ok(outcome) => {
            std::fs::rename(paths::long_path(&part), paths::long_path(dest))?;
            on_progress(outcome.bytes, total);
            Ok(outcome)
        }
        Err(e) => {
            let _ = std::fs::remove_file(paths::long_path(&part));
            Err(e)
        }
    }
}

pub fn with_part_suffix(dest: &Path) -> PathBuf {
    let mut s = dest.as_os_str().to_os_string();
    s.push(PART_SUFFIX);
    PathBuf::from(s)
}

fn copy_inner<F>(
    src: &Path,
    part: &Path,
    total: i64,
    cancel: &CancelFlag,
    on_progress: &mut F,
) -> Result<CopyOutcome>
where
    F: FnMut(i64, i64),
{
    let mut input = std::fs::File::open(paths::long_path(src))?;
    let mut output = std::fs::File::create(paths::long_path(part))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; CHUNK];
    let mut done: i64 = 0;
    let mut last_tick = now_ms();
    on_progress(0, total);
    loop {
        if cancel.is_cancelled() {
            return Err(CoreError::Cancelled("Import cancelled.".into()));
        }
        let n = input.read(&mut buf)?;
        if n == 0 {
            break;
        }
        output.write_all(&buf[..n])?;
        hasher.update(&buf[..n]);
        done += n as i64;
        let now = now_ms();
        if now - last_tick >= PROGRESS_INTERVAL_MS {
            last_tick = now;
            on_progress(done, total);
        }
    }
    output.flush()?;
    output.sync_all()?;
    Ok(CopyOutcome {
        sha256: hex(&hasher.finalize()),
        bytes: done,
    })
}

/// What a stat call alone can tell us about a source file. Cheap: no read.
#[derive(Debug, Clone)]
pub struct Probe {
    pub src: PathBuf,
    pub original_name: String,
    pub kind: AssetKind,
    pub mime: Option<&'static str>,
    pub bytes: i64,
    pub at_ms: i64,
}

/// Phase 1: stat the file. Instant, and enough to decide what happens next.
pub fn probe(req: &ImportRequest) -> Result<Probe> {
    let src = PathBuf::from(&req.path);
    if !src.is_file() {
        return Err(CoreError::invalid(format!(
            "`{}` is not a readable file.",
            src.display()
        )));
    }
    let bytes = src.metadata()?.len() as i64;
    let original_name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "file".to_string());
    Ok(Probe {
        kind: req.kind.unwrap_or_else(|| infer_kind(&original_name)),
        mime: paths::guess_mime(&original_name),
        original_name,
        src,
        bytes,
        at_ms: now_ms(),
    })
}

/// Where a probed file ended up, and what it hashed to.
pub struct Placement {
    pub asset_id: String,
    pub rel_path: String,
    pub abs_path: Option<String>,
    pub bytes: i64,
    pub sha256: String,
}

/// Phase 2: put the bytes where they belong, hashing in the same pass.
///
/// In `copy` mode this streams into the media root and the SHA-256 falls out
/// of the copy for free — the file is read exactly once. In `reference` mode
/// nothing is copied, so the hash costs one read.
pub fn place<F>(
    p: &Paths,
    probe: &Probe,
    mode: ImportMode,
    cancel: &CancelFlag,
    mut on_progress: F,
) -> Result<Placement>
where
    F: FnMut(i64, i64),
{
    let asset_id = new_id();
    let rel = dest_rel_path(&asset_id, &probe.original_name, probe.at_ms);
    match mode {
        ImportMode::Reference => {
            let (sha256, bytes) = hash_file(&probe.src, cancel, &mut on_progress)?;
            Ok(Placement {
                asset_id,
                rel_path: rel,
                abs_path: Some(paths::display_path(&probe.src)),
                bytes,
                sha256,
            })
        }
        ImportMode::Copy => {
            let dest = p.resolve_rel(&rel);
            p.guard_inside(&dest)?;
            let outcome = copy_streaming(&probe.src, &dest, probe.bytes, cancel, &mut on_progress)?;
            Ok(Placement {
                asset_id,
                rel_path: rel,
                abs_path: None,
                bytes: outcome.bytes,
                sha256: outcome.sha256,
            })
        }
    }
}

/// Undoes a `place` — used when a copy turns out to duplicate an asset that
/// was already in the library.
pub fn discard(p: &Paths, placement: &Placement, mode: ImportMode) -> Result<()> {
    if mode == ImportMode::Copy {
        let dest = p.resolve_rel(&placement.rel_path);
        p.guard_inside(&dest)?;
        let _ = std::fs::remove_file(paths::long_path(&dest));
    }
    Ok(())
}

/// Phase 3: write the row. Fast, so the database lock is held only briefly.
pub fn record(
    conn: &rusqlite::Connection,
    probe: &Probe,
    placement: &Placement,
    mode: ImportMode,
) -> Result<()> {
    asset_repo::insert(
        conn,
        &placement.asset_id,
        probe.kind,
        &probe.original_name,
        &placement.rel_path,
        placement.abs_path.as_deref(),
        mode,
        probe.mime,
        placement.bytes,
        &placement.sha256,
        probe.at_ms,
    )
}

/// What `import` produced.
pub struct ImportOutcome {
    pub asset_id: String,
    pub deduped: bool,
}

/// Runs the phases back to back.
///
/// The Tauri layer drives them itself so it can release the database lock
/// around the copy; this convenience form is what the test suite exercises and
/// it encodes the same dedupe strategy:
///
/// A pre-hash would let us skip the copy for a file already in the library,
/// but it costs a full read of every import — doubling the time for the common
/// case, a file that is *not* a duplicate. Byte length is a free pre-filter:
/// only when some existing asset is exactly the same size is a full hash worth
/// paying for. Otherwise the copy hashes as it goes and a duplicate is caught
/// at the end, where the wasted copy is simply deleted.
pub fn import<F>(
    conn: &rusqlite::Connection,
    p: &Paths,
    req: &ImportRequest,
    cancel: &CancelFlag,
    mut on_progress: F,
) -> Result<ImportOutcome>
where
    F: FnMut(i64, i64),
{
    let probe = probe(req)?;

    if asset_repo::any_with_size(conn, probe.bytes)? {
        let (sha, bytes) = hash_file(&probe.src, cancel, &mut on_progress)?;
        if let Some(existing) = asset_repo::find_by_hash(conn, &sha, bytes)? {
            tracing::info!(asset = %existing.id, "import deduped before copying");
            return Ok(ImportOutcome {
                asset_id: existing.id,
                deduped: true,
            });
        }
    }

    let placement = place(p, &probe, req.mode, cancel, &mut on_progress)?;

    // A same-size asset can appear between the check above and here, and a
    // reference import never took the pre-hash path at all.
    if let Some(existing) = asset_repo::find_by_hash(conn, &placement.sha256, placement.bytes)? {
        discard(p, &placement, req.mode)?;
        tracing::info!(asset = %existing.id, "import deduped after copying");
        return Ok(ImportOutcome {
            asset_id: existing.id,
            deduped: true,
        });
    }

    record(conn, &probe, &placement, req.mode)?;
    Ok(ImportOutcome {
        asset_id: placement.asset_id,
        deduped: false,
    })
}

/// Decodes a base64 JPEG grabbed from a `<canvas>` and stores it as this
/// asset's thumbnail.
pub fn write_thumbnail(
    conn: &rusqlite::Connection,
    p: &Paths,
    asset_id: &str,
    data_url_or_b64: &str,
) -> Result<String> {
    let bytes = base64_decode(data_url_or_b64)?;
    if bytes.is_empty() {
        return Err(CoreError::invalid("The thumbnail image was empty."));
    }
    let rel = format!("thumbs/{asset_id}.jpg");
    let dest = p.resolve_rel(&rel);
    p.guard_inside(&dest)?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(paths::long_path(parent))?;
    }
    std::fs::write(paths::long_path(&dest), &bytes)?;
    asset_repo::set_thumb(conn, asset_id, &rel)?;
    Ok(rel)
}

/// Minimal standard-alphabet base64 decoder — the only thing we need from a
/// base64 crate, and small enough to test directly.
///
/// Accepts either a bare payload or a whole `data:...;base64,...` URL, because
/// `canvas.toDataURL()` hands us the latter.
pub fn base64_decode(input: &str) -> Result<Vec<u8>> {
    let s = input
        .split_once(";base64,")
        .map(|(_, b)| b)
        .unwrap_or(input);
    fn val(c: u8) -> Option<u32> {
        Some(match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a') as u32 + 26,
            b'0'..=b'9' => (c - b'0') as u32 + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        })
    }
    let mut out = Vec::with_capacity(s.len() / 4 * 3);
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    for c in s.bytes() {
        if c.is_ascii_whitespace() || c == b'=' {
            continue;
        }
        let v = val(c).ok_or_else(|| CoreError::invalid("The thumbnail was not valid base64."))?;
        acc = (acc << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_file(dir: &Path, name: &str, bytes: &[u8]) -> PathBuf {
        let p = dir.join(name);
        let mut f = std::fs::File::create(&p).unwrap();
        f.write_all(bytes).unwrap();
        p
    }

    #[test]
    fn destination_layout_matches_the_spec() {
        // 2026-09-09T00:00:00Z
        let rel = dest_rel_path("abc-123", "Lesson 4.MP4", 1_788_912_000_000);
        assert_eq!(rel, "2026-09/abc-123.mp4");
        assert_eq!(
            dest_rel_path("abc", "noextension", 1_788_912_000_000),
            "2026-09/abc"
        );
    }

    #[test]
    fn kind_inference_covers_the_formats_we_care_about() {
        assert_eq!(infer_kind("a.mp4"), AssetKind::Video);
        assert_eq!(infer_kind("a.MKV"), AssetKind::Video);
        assert_eq!(infer_kind("a.png"), AssetKind::Image);
        assert_eq!(infer_kind("a.pdf"), AssetKind::File);
        assert_eq!(infer_kind("a"), AssetKind::File);
    }

    #[test]
    fn streamed_copy_hashes_and_lands_atomically() {
        let dir = tempfile::tempdir().unwrap();
        // Deliberately larger than one chunk to exercise the loop.
        let data: Vec<u8> = (0..(CHUNK * 2 + 777)).map(|i| (i % 251) as u8).collect();
        let src = tmp_file(dir.path(), "src.bin", &data);
        let dest = dir.path().join("out").join("dest.bin");

        let mut ticks = Vec::new();
        let outcome = copy_streaming(
            &src,
            &dest,
            data.len() as i64,
            &CancelFlag::new(),
            |d, t| ticks.push((d, t)),
        )
        .unwrap();

        assert_eq!(outcome.bytes, data.len() as i64);
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        assert!(
            !with_part_suffix(&dest).exists(),
            "part file was left behind"
        );
        let (direct, len) = hash_file(&src, &CancelFlag::new(), |_, _| {}).unwrap();
        assert_eq!(outcome.sha256, direct);
        assert_eq!(len, data.len() as i64);
        assert_eq!(ticks.last().unwrap().0, data.len() as i64);
    }

    #[test]
    fn cancelling_partway_through_leaves_nothing_behind() {
        let dir = tempfile::tempdir().unwrap();
        let data = vec![7u8; CHUNK * 3];
        let src = tmp_file(dir.path(), "src.bin", &data);
        let dest = dir.path().join("dest.bin");

        // `copy_streaming` creates the `.part` file, reports 0, then checks the
        // cancel flag at the top of every chunk. Cancelling from that first
        // callback aborts after the partial file exists but before it is
        // complete — deterministically, with no reliance on copy speed.
        let cancel = CancelFlag::new();
        let c2 = cancel.clone();
        let err = copy_streaming(&src, &dest, data.len() as i64, &cancel, move |_, _| {
            c2.cancel();
        })
        .unwrap_err();

        assert!(matches!(err, CoreError::Cancelled(_)), "got {err:?}");
        assert!(
            !dest.exists(),
            "cancelled import left the destination behind"
        );
        assert!(
            !with_part_suffix(&dest).exists(),
            "cancelled import left a .part file"
        );
    }

    #[test]
    fn cancelling_before_it_starts_writes_nothing_at_all() {
        let dir = tempfile::tempdir().unwrap();
        let src = tmp_file(dir.path(), "src.bin", b"some bytes");
        let dest = dir.path().join("dest.bin");
        let cancel = CancelFlag::new();
        cancel.cancel();

        let err = copy_streaming(&src, &dest, 10, &cancel, |_, _| {}).unwrap_err();
        assert!(matches!(err, CoreError::Cancelled(_)), "got {err:?}");
        assert!(!dest.exists());
        assert!(!with_part_suffix(&dest).exists());

        // Hashing is cancellable on the same terms.
        assert!(matches!(
            hash_file(&src, &cancel, |_, _| {}).unwrap_err(),
            CoreError::Cancelled(_)
        ));
    }

    #[test]
    fn identical_content_hashes_identically() {
        let dir = tempfile::tempdir().unwrap();
        let a = tmp_file(dir.path(), "lesson.mp4", b"the same bytes");
        let b = tmp_file(dir.path(), "lesson-final-v2.mp4", b"the same bytes");
        let c = tmp_file(dir.path(), "other.mp4", b"different bytes");
        let (ha, _) = hash_file(&a, &CancelFlag::new(), |_, _| {}).unwrap();
        let (hb, _) = hash_file(&b, &CancelFlag::new(), |_, _| {}).unwrap();
        let (hc, _) = hash_file(&c, &CancelFlag::new(), |_, _| {}).unwrap();
        assert_eq!(ha, hb);
        assert_ne!(ha, hc);
        // Known vector, so a change of hash implementation is caught.
        assert_eq!(
            hash_file(
                &tmp_file(dir.path(), "e", b""),
                &CancelFlag::new(),
                |_, _| {}
            )
            .unwrap()
            .0,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn base64_round_trips_the_canvas_payload_shape() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        assert_eq!(
            base64_decode("data:image/jpeg;base64,aGVsbG8=").unwrap(),
            b"hello"
        );
        assert_eq!(base64_decode("").unwrap(), Vec::<u8>::new());
        assert!(base64_decode("not base64!!").is_err());
    }
}
