//! RustClass teaching-library loader.
//!
//! A library is a folder of markdown files written in the `@@` directive
//! format of `library/FORMAT.md`. This crate parses and validates them
//! ([`format`]), checks that their Rust snippets compile ([`check`]), reports
//! coverage against `library/CURRICULUM.md` ([`coverage`]), lints the page
//! structure against `library/STYLE.md` ([`lint`]) and loads them into
//! the course database ([`load`]), from the `rustclass-library` CLI and from
//! the app's `library_load_builtin` command. The built-in library is baked
//! into the binary by `build.rs`.

pub mod check;
pub mod coverage;
pub mod format;
pub mod lint;
pub mod load;
mod scan;

pub use format::{errors_to_core, parse_files, LibError, LibWarning, Library};
pub use load::{load, LoadMode, LoadReport};

use std::path::Path;
use std::{fs, io};

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/builtin.rs"));
}

/// The library shipped inside the binary: `(file name, content)` pairs in
/// lexical filename order, ready for [`parse_files`]. Empty when
/// `library/ultimate-rust-bundle/` had no content files at build time.
pub fn builtin() -> Vec<(String, String)> {
    embedded::BUILTIN
        .iter()
        .map(|(name, content)| ((*name).to_string(), (*content).to_string()))
        .collect()
}

/// Reads a library folder: every `*.md` file in it in lexical filename order,
/// skipping names that start with `_`.
pub fn read_dir(dir: &Path) -> io::Result<Vec<(String, String)>> {
    let named =
        |e: io::Error, path: &Path| io::Error::new(e.kind(), format!("{}: {e}", path.display()));
    scan::list_markdown(dir)
        .map_err(|e| named(e, dir))?
        .into_iter()
        .map(|(name, path)| {
            fs::read_to_string(&path)
                .map(|text| (name, text))
                .map_err(|e| named(e, &path))
        })
        .collect()
}
