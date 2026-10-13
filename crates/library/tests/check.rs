//! The snippet checker.

use rustclass_library::check::{check_snippets, rustc_available, select};
use rustclass_library::{parse_files, read_dir, Library};
use std::path::PathBuf;

fn parse(content: &str) -> Library {
    parse_files(&[("1-02-lesson.md".to_string(), content.to_string())])
        .unwrap_or_else(|e| panic!("library should parse: {e:?}"))
}

/// Part, section and chapter: three lines, so line 4 is the first block.
const HEADER: &str = "@@part P\n@@section S\n@@chapter 1.1.1 | Title | 5\n";

#[test]
fn snippet_checker_flags_a_broken_snippet() {
    if !rustc_available() {
        println!("skipping: rustc is not on PATH, so snippets cannot be compiled here");
        return;
    }
    // One fine block, one use after move (E0382), and two more broken ones
    // that are exempt: a fragment and one that needs a crate.
    let content = format!(
        "{HEADER}\
         @@code rust | Fine\n\
         fn main() {{\n    let fill_ml = 500;\n    println!(\"{{fill_ml}}\");\n}}\n\
         @@code rust | Broken\n\
         fn main() {{\n    let recipe = String::from(\"A\");\n    let station = recipe;\n    println!(\"{{recipe}} {{station}}\");\n}}\n\
         @@code rust | Fragment\n// no-check\nimpl Station {{}}\n\
         @@code rust | Needs a crate\n// Cargo.toml: tokio = \"1\"\nuse tokio::time;\n\
         @@code bash | Not rust\nthis is not rust at all\n"
    );
    let line_of = |needle: &str| content.lines().position(|l| l.contains(needle)).unwrap() + 1;
    let broken_at = line_of("@@code rust | Broken");
    let use_at = line_of("println!(\"{recipe} {station}\")");

    let report = check_snippets(&parse(&content)).expect("rustc runs");
    assert_eq!(report.checked, 2, "{report:?}");
    assert_eq!(report.skipped, 2, "{report:?}");
    assert_eq!(report.failures.len(), 1, "{:#?}", report.failures);

    // `file:line:` of the broken block's @@code directive, then rustc's words.
    let failure = &report.failures[0];
    assert!(
        failure.starts_with(&format!("1-02-lesson.md:{broken_at}: ")),
        "wrong place: {failure}"
    );
    assert!(failure.contains("E0382"), "{failure}");
    assert!(failure.contains("borrow of moved value"), "{failure}");
    assert!(
        failure.lines().count() <= 15,
        "at most 15 lines of stderr: {failure}"
    );
    // rustc's own `-->` pointer is a line of the library file, not of a temp file.
    assert!(
        failure.contains(&format!("--> 1-02-lesson.md:{use_at}:")),
        "expected a pointer at line {use_at}: {failure}"
    );
}

#[test]
fn a_library_whose_snippets_all_compile_passes() {
    if !rustc_available() {
        println!("skipping: rustc is not on PATH");
        return;
    }
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/small-library");
    let lib = parse_files(&read_dir(&dir).unwrap()).unwrap();
    let report = check_snippets(&lib).expect("rustc runs");
    assert!(report.failures.is_empty(), "{:#?}", report.failures);
    // One real snippet; the `// Cargo.toml:` one and the `// no-check` one are exempt.
    assert_eq!((report.checked, report.skipped), (1, 2));
}

#[test]
fn only_plain_rust_blocks_are_selected() {
    let lib = parse(&format!(
        "{HEADER}\
         @@code rust | A\nfn main() {{}}\n\
         @@code rust | B\n// no-check\nfn broken(\n\
         @@code rust | C\n// Cargo.toml: serde = \"1\"\nuse serde::Serialize;\n\
         @@code rust | D\n\n\
         @@code toml | E\n[package]\n\
         @@text F\nfn main() {{}}\n"
    ));
    let (snippets, skipped) = select(&lib);
    assert_eq!(snippets.len(), 1);
    assert_eq!(skipped, 3, "no-check, Cargo.toml and an empty block");
    assert_eq!(snippets[0].code, "fn main() {}");
    assert_eq!(snippets[0].file, "1-02-lesson.md");
    assert_eq!(snippets[0].line, 4);
    assert_eq!(snippets[0].body_line, 5);
}

#[test]
fn a_library_with_no_snippets_never_needs_rustc() {
    let report = check_snippets(&Library::default()).expect("nothing to run");
    assert_eq!((report.checked, report.skipped), (0, 0));
    assert!(report.failures.is_empty());
}
