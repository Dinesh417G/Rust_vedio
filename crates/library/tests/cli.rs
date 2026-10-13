//! The `rustclass-library` binary, run the way the lead runs it.

use rustclass_library::check::rustc_available;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixture() -> PathBuf {
    manifest().join("tests/fixtures/small-library")
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustclass-library"))
        .args(args)
        // An empty working directory proves nothing here depends on it.
        .current_dir(std::env::temp_dir())
        .output()
        .expect("run the binary")
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

fn path(p: &Path) -> &str {
    p.to_str().expect("utf-8 path")
}

fn live_chapters(db: &Path) -> i64 {
    let conn = rusqlite::Connection::open(db).expect("open db");
    conn.query_row(
        "SELECT count(*) FROM chapter WHERE deleted_at IS NULL",
        [],
        |r| r.get(0),
    )
    .expect("count")
}

#[test]
fn help_prints_usage_and_exits_zero() {
    for flag in ["--help", "-h", "help"] {
        let out = run(&[flag]);
        assert!(out.status.success(), "{flag}");
        let text = stdout(&out);
        for needle in [
            "check", "coverage", "lint", "load", "--dir", "--db", "--append",
        ] {
            assert!(
                text.contains(needle),
                "{flag}: usage should mention {needle}"
            );
        }
    }
    let out = run(&["load", "--help"]);
    assert!(out.status.success());
}

#[test]
fn bad_usage_exits_two() {
    for args in [
        &[][..],
        &["frobnicate"][..],
        &["check", "--nope"][..],
        &["check", "--dir"][..],
        &["check", "stray"][..],
    ] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", stderr(&out));
        assert!(stderr(&out).contains("error:"), "{args:?}");
    }
}

#[test]
fn load_prints_the_fixture_counts() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("nested").join("rustclass.db");

    let out = run(&["load", "--dir", path(&fixture()), "--db", path(&db)]);
    assert!(out.status.success(), "{}", stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("chapters created  3"), "{text}");
    assert!(text.contains("blocks created    20"), "{text}");
    assert!(text.contains("chapters removed  0"), "{text}");
    assert!(text.contains("files             2"), "{text}");
    assert!(text.contains("mode              replace"), "{text}");
    assert_eq!(live_chapters(&db), 3);

    // Replace again: the three are trashed, three new ones take their place.
    let out = run(&["load", "--dir", path(&fixture()), "--db", path(&db)]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert!(
        stdout(&out).contains("chapters removed  3"),
        "{}",
        stdout(&out)
    );
    assert_eq!(live_chapters(&db), 3);

    // Append adds after them and removes nothing.
    let out = run(&[
        "load",
        "--dir",
        path(&fixture()),
        "--db",
        path(&db),
        "--append",
    ]);
    assert!(out.status.success(), "{}", stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("mode              append"), "{text}");
    assert!(text.contains("chapters removed  0"), "{text}");
    assert_eq!(live_chapters(&db), 6);
}

#[test]
fn load_refuses_a_library_with_errors_and_writes_nothing() {
    let lib = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        lib.path().join("1-bad.md"),
        "@@part P\n@@section S\n@@chapter 1.1.1 | T | 5\n@@text x\nbody\n@@bogus\n",
    )
    .unwrap();
    let dbdir = tempfile::tempdir().expect("tempdir");
    let db = dbdir.path().join("rustclass.db");

    let out = run(&["load", "--dir", path(lib.path()), "--db", path(&db)]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        stderr(&out).contains("1-bad.md:6: unknown directive `@@bogus`"),
        "{}",
        stderr(&out)
    );
    assert!(!db.exists(), "the database is not even created");
}

#[test]
fn check_passes_on_the_fixture() {
    let dir = fixture();
    let mut args = vec!["check", "--dir", path(&dir)];
    if !rustc_available() {
        println!("rustc is not on PATH: checking the syntax only");
        args.push("--parse-only");
    }
    let out = run(&args);
    assert!(out.status.success(), "{}{}", stdout(&out), stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("2 files, 3 chapters, 20 blocks"), "{text}");
    assert!(
        text.contains("OK") || text.contains("--parse-only"),
        "{text}"
    );
}

#[test]
fn check_exits_one_on_a_broken_snippet_and_on_a_bad_file() {
    let lib = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        lib.path().join("1-lesson.md"),
        "@@part P\n@@section S\n@@chapter 1.1.1 | T | 5\n\
         @@code rust | Broken\nfn main() {\n    let x: u32 = \"text\";\n}\n",
    )
    .unwrap();

    // Parse-only does not care that the snippet is broken.
    let out = run(&["check", "--dir", path(lib.path()), "--parse-only"]);
    assert!(out.status.success(), "{}", stderr(&out));

    if rustc_available() {
        let out = run(&["check", "--dir", path(lib.path())]);
        assert_eq!(out.status.code(), Some(1), "{}", stdout(&out));
        let err = stderr(&out);
        assert!(err.contains("1-lesson.md:4: "), "{err}");
        assert!(err.contains("E0308"), "{err}");
        assert!(err.contains("FAILED: 1 of 1 snippets"), "{err}");
    } else {
        println!("rustc is not on PATH: skipping the snippet half");
    }

    std::fs::write(lib.path().join("2-bad.md"), "@@bogus\n").unwrap();
    let out = run(&["check", "--dir", path(lib.path()), "--parse-only"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        stderr(&out).contains("2-bad.md:1: unknown directive"),
        "{}",
        stderr(&out)
    );
}

#[test]
fn check_on_an_empty_library_succeeds() {
    let lib = tempfile::tempdir().expect("tempdir");
    std::fs::write(lib.path().join(".gitkeep"), "").unwrap();
    std::fs::write(lib.path().join("_draft.md"), "@@bogus\n").unwrap();
    let out = run(&["check", "--dir", path(lib.path())]);
    assert!(out.status.success(), "{}{}", stdout(&out), stderr(&out));
    assert!(
        stdout(&out).contains("0 files, 0 chapters, 0 blocks"),
        "{}",
        stdout(&out)
    );
}

#[test]
fn coverage_prints_the_lists_and_exits_zero() {
    let curriculum = manifest().join("tests/fixtures/small-curriculum.md");
    let out = run(&[
        "coverage",
        "--dir",
        path(&fixture()),
        "--curriculum",
        path(&curriculum),
    ]);
    assert!(out.status.success(), "{}", stderr(&out));
    let text = stdout(&out);
    assert!(
        text.contains("3 present, 3 missing, 0 mismatch, 0 unexpected (of 6 expected)"),
        "{text}"
    );
    assert!(text.contains("Missing (3)"), "{text}");
    assert!(text.contains("1.1.3"), "{text}");
    assert!(
        text.contains("2 · Fixture Webinars / First webinar (1): 2.1.1"),
        "{text}"
    );
}

#[test]
fn lint_exits_one_with_violations_and_zero_on_an_empty_library() {
    let curriculum = manifest().join("tests/fixtures/small-curriculum.md");
    let out = run(&[
        "lint",
        "--dir",
        path(&fixture()),
        "--curriculum",
        path(&curriculum),
    ]);
    assert_eq!(out.status.code(), Some(1), "{}", stdout(&out));
    let err = stderr(&out);
    // The fixture lesson has no PLC table, no watch-out and short notes.
    assert!(
        err.contains("1.1.1 Lesson 1.1 - Variables: missing text \"PLC ↔ Rust\""),
        "{err}"
    );
    assert!(err.contains("FAILED: "), "{err}");

    let empty = tempfile::tempdir().expect("tempdir");
    let out = run(&[
        "lint",
        "--dir",
        path(empty.path()),
        "--curriculum",
        path(&curriculum),
    ]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert!(stdout(&out).contains("OK"), "{}", stdout(&out));
}

#[test]
fn coverage_of_the_real_curriculum_against_the_empty_builtin_folder() {
    let root = manifest().join("../..");
    let curriculum = root.join("library/CURRICULUM.md");
    let empty = tempfile::tempdir().expect("tempdir");
    if !curriculum.exists() {
        println!("skipping: {} is not there", curriculum.display());
        return;
    }
    let out = run(&[
        "coverage",
        "--dir",
        path(empty.path()),
        "--curriculum",
        path(&curriculum),
    ]);
    assert!(out.status.success(), "{}", stderr(&out));
    let text = stdout(&out);
    assert!(
        text.contains("0 present, 304 missing, 0 mismatch, 0 unexpected (of 304 expected)"),
        "{text}"
    );
}
