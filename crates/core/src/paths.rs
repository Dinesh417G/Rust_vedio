//! Filesystem layout and the safety rules around it.
//!
//! Everything the app owns lives under one root:
//! ```text
//! <app_data>/
//!   rustclass.db
//!   media/<yyyy-mm>/<uuid>.<ext>
//!   media/thumbs/<uuid>.jpg
//!   logs/
//! ```

use crate::error::{CoreError, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Paths {
    pub app_data: PathBuf,
}

impl Paths {
    pub fn new(app_data: impl Into<PathBuf>) -> Self {
        Self {
            app_data: app_data.into(),
        }
    }

    pub fn db_file(&self) -> PathBuf {
        self.app_data.join("rustclass.db")
    }

    pub fn media_root(&self) -> PathBuf {
        self.app_data.join("media")
    }

    pub fn thumbs_root(&self) -> PathBuf {
        self.media_root().join("thumbs")
    }

    pub fn logs_root(&self) -> PathBuf {
        self.app_data.join("logs")
    }

    pub fn ensure(&self) -> Result<()> {
        std::fs::create_dir_all(self.media_root())?;
        std::fs::create_dir_all(self.thumbs_root())?;
        std::fs::create_dir_all(self.logs_root())?;
        Ok(())
    }

    /// Resolves a `rel_path` stored in the database against the media root.
    /// Relative storage is what lets the whole app data folder move machines.
    pub fn resolve_rel(&self, rel: &str) -> PathBuf {
        self.media_root().join(rel.replace('\\', "/"))
    }

    /// Refuses any path that would escape the app data directory.
    pub fn guard_inside(&self, path: &Path) -> Result<()> {
        let root = normalize(&self.app_data);
        let target = normalize(path);
        if target.starts_with(&root) {
            Ok(())
        } else {
            Err(CoreError::invalid(format!(
                "refusing to touch `{}`: it is outside the RustClass data folder",
                path.display()
            )))
        }
    }
}

/// Lexical normalisation — `canonicalize` is unusable here because the target
/// may not exist yet (we are often about to create it).
pub fn normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Windows refuses these characters and these names outright; other platforms
/// tolerate them but a `.rcbundle` has to survive the round trip either way.
const FORBIDDEN: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
const RESERVED: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Makes an arbitrary user filename safe to write, preserving non-ASCII text.
pub fn sanitize_filename(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| {
            if FORBIDDEN.contains(&c) || (c as u32) < 0x20 {
                '_'
            } else {
                c
            }
        })
        .collect();
    out = out.trim().trim_end_matches('.').to_string();
    let stem_upper = out
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    if RESERVED.contains(&stem_upper.as_str()) {
        out = format!("_{out}");
    }
    if out.is_empty() {
        out = "file".to_string();
    }
    // Leave room for the directory part of a long Windows path.
    if out.chars().count() > 120 {
        let ext = extension_of(&out);
        let keep: String = out.chars().take(110).collect();
        out = match ext {
            Some(e) => format!("{keep}.{e}"),
            None => keep,
        };
    }
    out
}

/// Lowercased extension without the dot, if the name has a usable one.
pub fn extension_of(name: &str) -> Option<String> {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .filter(|e| !e.is_empty() && e.chars().count() <= 12)
}

/// Windows chokes on paths over 260 characters unless they carry the verbatim
/// `\\?\` prefix. `dunce` handles the inverse (stripping it for display).
#[cfg(windows)]
pub fn long_path(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    if s.len() < 250 || s.starts_with(r"\\?\") {
        return p.to_path_buf();
    }
    match p.is_absolute() {
        true => PathBuf::from(format!(r"\\?\{}", s.replace('/', "\\"))),
        false => p.to_path_buf(),
    }
}

#[cfg(not(windows))]
pub fn long_path(p: &Path) -> PathBuf {
    p.to_path_buf()
}

/// The path shape we hand to the webview and show to the user.
pub fn display_path(p: &Path) -> String {
    dunce::simplified(p).to_string_lossy().to_string()
}

/// A best-effort MIME type from the extension. Used only as a hint; the
/// webview decides what it can actually decode.
pub fn guess_mime(name: &str) -> Option<&'static str> {
    Some(match extension_of(name)?.as_str() {
        "mp4" | "m4v" => "video/mp4",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "mov" => "video/quicktime",
        "avi" => "video/x-msvideo",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "avif" => "image/avif",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "txt" | "md" => "text/plain",
        "rs" => "text/x-rust",
        "json" => "application/json",
        "toml" => "text/x-toml",
        _ => return None,
    })
}

/// Formats WebView2 can actually play. Anything else gets the fallback banner.
pub fn is_webview_playable(name: &str) -> bool {
    matches!(
        extension_of(name).as_deref(),
        Some("mp4") | Some("m4v") | Some("webm") | Some("ogg") | Some("ogv")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_characters_are_replaced_but_unicode_survives() {
        assert_eq!(sanitize_filename("a/b:c*d.mp4"), "a_b_c_d.mp4");
        assert_eq!(
            sanitize_filename("Владение и заимствование.mp4"),
            "Владение и заимствование.mp4"
        );
        assert_eq!(sanitize_filename("第4章 所有権.mp4"), "第4章 所有権.mp4");
        assert_eq!(
            sanitize_filename("spaces are fine.mp4"),
            "spaces are fine.mp4"
        );
    }

    #[test]
    fn reserved_windows_names_are_escaped() {
        assert_eq!(sanitize_filename("CON.txt"), "_CON.txt");
        assert_eq!(sanitize_filename("nul"), "_nul");
        assert_eq!(sanitize_filename("console.txt"), "console.txt");
    }

    #[test]
    fn empty_and_dot_names_get_a_fallback() {
        assert_eq!(sanitize_filename(""), "file");
        assert_eq!(sanitize_filename("   "), "file");
        assert_eq!(sanitize_filename("..."), "file");
    }

    #[test]
    fn very_long_names_are_truncated_keeping_the_extension() {
        let long = format!("{}.mp4", "ы".repeat(400));
        let s = sanitize_filename(&long);
        assert!(s.chars().count() <= 120, "got {} chars", s.chars().count());
        assert!(s.ends_with(".mp4"));
    }

    #[test]
    fn guard_rejects_traversal() {
        let p = Paths::new("/data/rustclass");
        assert!(p
            .guard_inside(Path::new("/data/rustclass/media/x.mp4"))
            .is_ok());
        assert!(p
            .guard_inside(Path::new("/data/rustclass/media/../../etc/passwd"))
            .is_err());
        assert!(p.guard_inside(Path::new("/etc/passwd")).is_err());
    }

    #[test]
    fn relative_paths_resolve_under_the_media_root() {
        let p = Paths::new("/data/rustclass");
        assert_eq!(
            p.resolve_rel("2026-09/abc.mp4"),
            PathBuf::from("/data/rustclass/media/2026-09/abc.mp4")
        );
        // Windows-style separators from an older bundle still resolve.
        assert_eq!(
            p.resolve_rel(r"2026-09\abc.mp4"),
            PathBuf::from("/data/rustclass/media/2026-09/abc.mp4")
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_long_paths_get_the_verbatim_prefix() {
        // Over 260 characters Windows refuses the path outright unless it
        // carries `\\?\`. This only compiles and runs on Windows, so it is the
        // cross-compile check that keeps it honest.
        let long = PathBuf::from(format!(r"C:\media\{}\file.mp4", "d".repeat(300)));
        let prefixed = long_path(&long);
        assert!(prefixed.to_string_lossy().starts_with(r"\\?\"));

        // Short paths are left exactly as they are.
        let short = PathBuf::from(r"C:\media\a.mp4");
        assert_eq!(long_path(&short), short);

        // And an already-prefixed path is not prefixed twice.
        let already = PathBuf::from(format!(r"\\?\C:\media\{}\file.mp4", "d".repeat(300)));
        assert_eq!(long_path(&already), already);
    }

    #[test]
    fn codec_support_matches_webview2_reality() {
        assert!(is_webview_playable("lesson.mp4"));
        assert!(is_webview_playable("lesson.webm"));
        assert!(!is_webview_playable("lesson.mkv"));
        assert!(!is_webview_playable("lesson.avi"));
    }
}
