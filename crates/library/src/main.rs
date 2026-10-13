//! `rustclass-library`: check, measure and load a teaching library.

use rustclass_library::{
    check, coverage, lint, load, parse_files, read_dir, LibError, Library, LoadMode,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const DEFAULT_DIR: &str = "library/ultimate-rust-bundle";
const DEFAULT_CURRICULUM: &str = "library/CURRICULUM.md";
/// The folder Tauri derives from `identifier` in `tauri.conf.json`.
const APP_IDENTIFIER: &str = "in.electronix.rustclass";

const USAGE: &str = "\
rustclass-library: check, measure and load a RustClass teaching library

USAGE
  rustclass-library check [--dir <dir>] [--parse-only]
      Parse the library and compile every Rust snippet. Exit 1 on any error.
  rustclass-library coverage [--dir <dir>] [--curriculum <file>]
      Compare the library with the curriculum: present, missing, unexpected,
      mismatch. Always exits 0 when it can read both.
  rustclass-library lint [--dir <dir>] [--curriculum <file>]
      Check every chapter against the topic-page block order of
      library/STYLE.md, using the page type of its curriculum row. Prints
      `display title: problem` per violation. Exit 1 on any violation.
  rustclass-library load [--dir <dir>] [--db <path>] [--append]
      Load the library into a course database. By default it REPLACES the
      course: every live chapter is moved to the trash first. --append adds
      the library after the existing chapters instead.

OPTIONS
  --dir <dir>          library folder        (default: library/ultimate-rust-bundle)
  --curriculum <file>  curriculum markdown   (default: library/CURRICULUM.md)
  --db <path>          course database       (default: %APPDATA%\\in.electronix.rustclass\\rustclass.db)
  --parse-only         check: skip the rustc snippet check
  --append             load: add instead of replace
  -h, --help           show this text

The library folder holds *.md files in the @@ directive format of
library/FORMAT.md. Files load in filename order; names starting with _ are
skipped.
";

enum Failure {
    /// Bad command line: prints the usage too, exits 2.
    Usage(String),
    /// Something went wrong that is not the content's fault.
    Error(String),
    /// The library's own errors, one per line.
    Content(Vec<LibError>),
}

impl From<rustclass_core::CoreError> for Failure {
    fn from(e: rustclass_core::CoreError) -> Self {
        Failure::Error(e.to_string())
    }
}

struct Opts {
    values: HashMap<&'static str, String>,
    switches: Vec<&'static str>,
}

impl Opts {
    fn value(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    fn has(&self, name: &str) -> bool {
        self.switches.contains(&name)
    }
}

/// Hand-rolled on purpose: three subcommands do not justify a parser crate.
fn parse_opts(
    args: &[String],
    valued: &[&'static str],
    switches: &[&'static str],
) -> Result<Opts, Failure> {
    let mut opts = Opts {
        values: HashMap::new(),
        switches: Vec::new(),
    };
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        let Some(flag) = arg.strip_prefix("--") else {
            return Err(Failure::Usage(format!("unexpected argument `{arg}`")));
        };
        let (name, inline) = match flag.split_once('=') {
            Some((n, v)) => (n, Some(v.to_string())),
            None => (flag, None),
        };
        if let Some(&known) = valued.iter().find(|&&v| v == name) {
            let value = match inline {
                Some(v) => v,
                None => it
                    .next()
                    .cloned()
                    .ok_or_else(|| Failure::Usage(format!("`--{name}` needs a value")))?,
            };
            opts.values.insert(known, value);
        } else if let Some(&known) = switches.iter().find(|&&s| s == name) {
            if inline.is_some() {
                return Err(Failure::Usage(format!("`--{name}` takes no value")));
            }
            opts.switches.push(known);
        } else {
            return Err(Failure::Usage(format!("unknown option `--{name}`")));
        }
    }
    Ok(opts)
}

fn dir_of(opts: &Opts) -> PathBuf {
    PathBuf::from(opts.value("dir").unwrap_or(DEFAULT_DIR))
}

/// Reads and parses a library folder. Content errors come back as
/// `Failure::Content`; the warnings are returned inside the library.
fn read_library(dir: &Path) -> Result<Library, Failure> {
    let files = read_dir(dir).map_err(|e| Failure::Error(format!("cannot read {e}")))?;
    parse_files(&files).map_err(Failure::Content)
}

fn print_warnings(lib: &Library) {
    for w in &lib.warnings {
        println!("warning: {w}");
    }
}

fn cmd_check(args: &[String]) -> Result<ExitCode, Failure> {
    let opts = parse_opts(args, &["dir"], &["parse-only"])?;
    let dir = dir_of(&opts);
    let lib = read_library(&dir)?;
    println!(
        "{}: {} files, {} chapters, {} blocks",
        dir.display(),
        lib.files.len(),
        lib.chapters.len(),
        lib.block_count()
    );
    print_warnings(&lib);
    println!("{} warnings", lib.warnings.len());

    if opts.has("parse-only") {
        println!("snippets: not compiled (--parse-only)");
        return Ok(ExitCode::SUCCESS);
    }
    let report = check::check_snippets(&lib).map_err(|e| {
        Failure::Error(format!(
            "could not run rustc (is it on PATH?) to check the snippets: {e}"
        ))
    })?;
    println!(
        "snippets: {} compiled, {} skipped (// no-check, // Cargo.toml: or empty)",
        report.checked, report.skipped
    );
    if report.failures.is_empty() {
        println!("OK");
        return Ok(ExitCode::SUCCESS);
    }
    for failure in &report.failures {
        eprintln!("\n{failure}");
    }
    eprintln!(
        "\nFAILED: {} of {} snippets do not compile",
        report.failures.len(),
        report.checked
    );
    Ok(ExitCode::FAILURE)
}

/// Reads the library folder and the curriculum named by `--curriculum`.
fn read_both(opts: &Opts) -> Result<(Library, PathBuf, Vec<coverage::Expected>), Failure> {
    let lib = read_library(&dir_of(opts))?;
    let path = PathBuf::from(opts.value("curriculum").unwrap_or(DEFAULT_CURRICULUM));
    let md = std::fs::read_to_string(&path)
        .map_err(|e| Failure::Error(format!("cannot read {}: {e}", path.display())))?;
    let expected = coverage::parse_curriculum(&md).map_err(Failure::Error)?;
    Ok((lib, path, expected))
}

fn cmd_lint(args: &[String]) -> Result<ExitCode, Failure> {
    let opts = parse_opts(args, &["dir", "curriculum"], &[])?;
    let (lib, _, expected) = read_both(&opts)?;
    let issues = lint::lint(&expected, &lib);
    println!(
        "{}: {} chapters linted",
        dir_of(&opts).display(),
        lib.chapters.len()
    );
    if issues.is_empty() {
        println!("OK");
        return Ok(ExitCode::SUCCESS);
    }
    for issue in &issues {
        eprintln!("{issue}");
    }
    eprintln!("\nFAILED: {} violations", issues.len());
    Ok(ExitCode::FAILURE)
}

fn cmd_coverage(args: &[String]) -> Result<ExitCode, Failure> {
    let opts = parse_opts(args, &["dir", "curriculum"], &[])?;
    let (lib, path, expected) = read_both(&opts)?;
    let report = coverage::compare(&expected, &lib);
    println!("Coverage of {}", path.display());
    print!("{report}");
    Ok(ExitCode::SUCCESS)
}

fn default_db() -> Result<PathBuf, Failure> {
    let appdata = std::env::var_os("APPDATA")
        .ok_or_else(|| Failure::Usage("APPDATA is not set; pass --db <path>".into()))?;
    Ok(PathBuf::from(appdata)
        .join(APP_IDENTIFIER)
        .join("rustclass.db"))
}

fn cmd_load(args: &[String]) -> Result<ExitCode, Failure> {
    let opts = parse_opts(args, &["dir", "db"], &["append"])?;
    let dir = dir_of(&opts);
    let lib = read_library(&dir)?;
    let db = match opts.value("db") {
        Some(p) => PathBuf::from(p),
        None => default_db()?,
    };
    let mode = if opts.has("append") {
        LoadMode::Append
    } else {
        LoadMode::Replace
    };

    // `open` creates the folder and migrates; the course row must exist
    // before anything is loaded into it.
    let mut conn = rustclass_core::db::open(&db)
        .map_err(|e| Failure::Error(format!("cannot open {}: {e}", db.display())))?;
    rustclass_core::repo::course::get_or_seed(&conn)?;
    let report = load(&mut conn, &lib, mode)?;

    println!("library           {}", dir.display());
    println!("database          {}", db.display());
    println!(
        "mode              {}",
        match mode {
            LoadMode::Replace => "replace",
            LoadMode::Append => "append",
        }
    );
    println!("files             {}", report.files);
    println!("chapters created  {}", report.chapters_created);
    println!("blocks created    {}", report.blocks_created);
    println!("chapters removed  {}", report.chapters_removed);
    println!("warnings          {}", report.warnings.len());
    for w in &report.warnings {
        println!("  warning: {w}");
    }
    Ok(ExitCode::SUCCESS)
}

fn run(args: &[String]) -> Result<ExitCode, Failure> {
    let Some((command, rest)) = args.split_first() else {
        return Err(Failure::Usage("no command given".into()));
    };
    if matches!(command.as_str(), "-h" | "--help" | "help")
        || rest.iter().any(|a| a == "-h" || a == "--help")
    {
        print!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    match command.as_str() {
        "check" => cmd_check(rest),
        "coverage" => cmd_coverage(rest),
        "lint" => cmd_lint(rest),
        "load" => cmd_load(rest),
        other => Err(Failure::Usage(format!("unknown command `{other}`"))),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(code) => code,
        Err(Failure::Usage(msg)) => {
            eprintln!("error: {msg}\n\n{USAGE}");
            ExitCode::from(2)
        }
        Err(Failure::Error(msg)) => {
            eprintln!("error: {msg}");
            ExitCode::FAILURE
        }
        Err(Failure::Content(errors)) => {
            for e in &errors {
                eprintln!("error: {e}");
            }
            eprintln!(
                "\n{} error{} in the library; nothing was written",
                errors.len(),
                if errors.len() == 1 { "" } else { "s" }
            );
            ExitCode::FAILURE
        }
    }
}
