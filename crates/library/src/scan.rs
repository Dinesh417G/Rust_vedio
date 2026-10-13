//! Which files make up a library folder.
//!
//! Shared by `build.rs`, which embeds the built-in library, and by
//! `read_dir`, which loads a folder at run time, so the two can never disagree
//! about what counts as a content file.

use std::io;
use std::path::{Path, PathBuf};

/// `*.md` files directly inside `dir`, in lexical filename order. Names that
/// start with `_` are drafts and notes and are skipped.
pub fn list_markdown(dir: &Path) -> io::Result<Vec<(String, PathBuf)>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()).map(String::from) else {
            continue;
        };
        let is_markdown = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("md"));
        if is_markdown && !name.starts_with('_') && path.is_file() {
            files.push((name, path));
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}
