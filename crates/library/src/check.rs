//! Compiles every Rust snippet in a library, so no lesson teaches code that
//! does not build.
//!
//! Each `@@code rust` block is written to a temp dir and handed to
//! `rustc --edition 2021 --crate-type bin --emit=metadata`: parsing, type
//! checking and borrow checking, no code generation and no linking, so a
//! snippet costs a fraction of a second. Warnings are fine, errors are not.
//!
//! Two blocks are exempt, as `library/STYLE.md` says: one whose first line is
//! `// no-check` (a fragment that cannot stand alone), and one that carries a
//! `// Cargo.toml:` line (it needs a crate that plain `rustc` cannot fetch).

use crate::format::Library;
use rustclass_core::models::BlockKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io, thread};

/// How many lines of rustc's stderr a failure keeps.
const STDERR_LINES: usize = 15;

/// One Rust block that will be compiled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snippet {
    pub file: String,
    /// Line of the `@@code` directive.
    pub line: usize,
    /// Line of the first line of code.
    pub body_line: usize,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CheckReport {
    /// Snippets compiled.
    pub checked: usize,
    /// Rust blocks exempted by `// no-check` or `// Cargo.toml:`, or empty.
    pub skipped: usize,
    /// `file:line: <first 15 lines of rustc stderr>`, one per broken snippet.
    pub failures: Vec<String>,
}

fn is_exempt(code: &str) -> bool {
    let first = code.lines().next().unwrap_or("").trim();
    first.starts_with("// no-check")
        || code
            .lines()
            .any(|l| l.trim_start().starts_with("// Cargo.toml:"))
}

/// The Rust blocks to compile, and how many were left out.
pub fn select(lib: &Library) -> (Vec<Snippet>, usize) {
    let mut snippets = Vec::new();
    let mut skipped = 0;
    for chapter in &lib.chapters {
        for block in &chapter.blocks {
            if block.kind != BlockKind::Code || block.code_lang.as_deref() != Some("rust") {
                continue;
            }
            match block.code_text.as_deref() {
                Some(code) if !is_exempt(code) => snippets.push(Snippet {
                    file: chapter.file.clone(),
                    line: block.line,
                    body_line: block.body_line,
                    code: code.to_string(),
                }),
                _ => skipped += 1,
            }
        }
    }
    (snippets, skipped)
}

/// Can `rustc` be started at all?
pub fn rustc_available() -> bool {
    Command::new("rustc")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
}

/// A scratch directory that removes itself, however the check ends.
struct TempDir(PathBuf);

impl TempDir {
    fn new() -> io::Result<Self> {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let path = std::env::temp_dir().join(format!(
            "rustclass-library-check-{}-{nanos}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path)?;
        Ok(Self(path))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

type Outcome = io::Result<Option<String>>;

fn check_one(root: &Path, index: usize, snippet: &Snippet) -> Outcome {
    let dir = root.join(index.to_string());
    fs::create_dir_all(&dir)?;
    // Named after the library file, and padded with blank lines so rustc's own
    // `--> file:line:col` points at the line in the library file.
    let name = Path::new(&snippet.file)
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|n| !n.is_empty())
        .unwrap_or("snippet.rs");
    let source = format!(
        "{}{}\n",
        "\n".repeat(snippet.body_line.saturating_sub(1)),
        snippet.code
    );
    fs::write(dir.join(name), source)?;

    let out = Command::new("rustc")
        .current_dir(&dir)
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "bin",
            "--crate-name",
            "snippet",
            "--emit=metadata",
            "-A",
            "warnings",
            "-o",
            "snippet.rmeta",
        ])
        .arg(name)
        .output()?;
    if out.status.success() {
        return Ok(None);
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    let head: Vec<&str> = stderr.lines().take(STDERR_LINES).collect();
    Ok(Some(format!(
        "{}:{}: {}",
        snippet.file,
        snippet.line,
        head.join("\n")
    )))
}

/// Compiles every selectable snippet, several at a time. `Err` means rustc
/// could not be run at all; broken snippets are reported in the `Ok` value.
pub fn check_snippets(lib: &Library) -> io::Result<CheckReport> {
    let (snippets, skipped) = select(lib);
    if snippets.is_empty() {
        return Ok(CheckReport {
            checked: 0,
            skipped,
            failures: Vec::new(),
        });
    }

    let root = TempDir::new()?;
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<Outcome>>> = Mutex::new(snippets.iter().map(|_| None).collect());
    let workers = thread::available_parallelism()
        .map_or(2, |n| n.get())
        .clamp(1, 8)
        .min(snippets.len());
    thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(snippet) = snippets.get(i) else {
                    break;
                };
                let outcome = check_one(&root.0, i, snippet);
                results.lock().expect("results lock")[i] = Some(outcome);
            });
        }
    });

    let mut failures = Vec::new();
    for outcome in results
        .into_inner()
        .expect("results lock")
        .into_iter()
        .flatten()
    {
        if let Some(message) = outcome? {
            failures.push(message);
        }
    }
    Ok(CheckReport {
        checked: snippets.len(),
        skipped,
        failures,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemptions_follow_style_md() {
        assert!(is_exempt("// no-check\nimpl Foo {}"));
        assert!(is_exempt("// Cargo.toml: tokio = \"1\"\nuse tokio;"));
        assert!(is_exempt(
            "//! intro\n// Cargo.toml: serde = \"1\"\nfn main() {}"
        ));
        assert!(!is_exempt("fn main() {}\n// no-check"));
        assert!(!is_exempt("fn main() {}"));
    }
}
