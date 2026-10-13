//! Coverage against the curriculum.

use rustclass_library::coverage::{compare, parse_curriculum};
use rustclass_library::{parse_files, Library};
use std::path::PathBuf;

const SMALL_CURRICULUM: &str = include_str!("fixtures/small-curriculum.md");

fn parse(content: &str) -> Library {
    parse_files(&[("c.md".to_string(), content.to_string())])
        .unwrap_or_else(|e| panic!("library should parse: {e:?}"))
}

fn real_curriculum() -> Option<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../library/CURRICULUM.md");
    match std::fs::read_to_string(&path) {
        Ok(md) => Some(md),
        Err(_) => {
            println!("skipping: {} is not there", path.display());
            None
        }
    }
}

#[test]
fn coverage_reports_missing_and_mismatch() {
    let expected = parse_curriculum(SMALL_CURRICULUM).expect("fixture curriculum");
    assert_eq!(expected.len(), 6, "{expected:#?}");

    // Section headings carry the section for part 0 and 1, the `section`
    // column for part 2, and the Totals table is not read as rows.
    let by_display = |d: &str| expected.iter().find(|e| e.display == d).expect(d);
    assert_eq!(by_display("0.1.1").part, "0 · Start Here");
    assert_eq!(by_display("0.1.1").section, "Orientation");
    assert_eq!(by_display("0.1.1").kind, "Own");
    assert_eq!(by_display("1.1.3").section, "Module 1: Basics");
    assert_eq!(by_display("2.1.1").part, "2 · Fixture Webinars");
    assert_eq!(by_display("2.1.1").section, "First webinar");
    assert_eq!(by_display("2.2.1").title, "Webinar two");

    let lib = parse(
        "@@part 0 · Start Here\n@@section Orientation\n\
         @@chapter 0.1.1 | How to use this library | 5\n@@text x\nbody\n\
         @@part 1 · Fixture Crash Course\n@@section Module 1: Basics\n\
         @@chapter 1.1.1 | Lesson 1.1 - Variable | 5\n@@text x\nbody\n\
         @@section Module 2: Elsewhere\n\
         @@chapter 1.1.2 | Lesson 1.2 - Cargo | 5\n@@text x\nbody\n\
         @@part 3 · Nowhere\n\
         @@chapter 9.9.9 | Not in the curriculum | 5\n@@text x\nbody\n\
         @@chapter | Nobody numbered me | 5\n@@text x\nbody\n\
         @@part 1 · Fixture Crash Course\n@@section Module 1: Basics\n\
         @@chapter 0.1.1 | How to use this library | 5\n@@text x\nbody\n",
    );
    let cov = compare(&expected, &lib);

    assert_eq!(cov.expected, 6);
    assert_eq!(cov.present, ["0.1.1"]);

    let missing: Vec<&str> = cov.missing.iter().map(|e| e.display.as_str()).collect();
    assert_eq!(missing, ["1.1.3", "2.1.1", "2.2.1"]);

    assert_eq!(cov.mismatch.len(), 2);
    let title = &cov.mismatch[0];
    assert_eq!(title.display, "1.1.1");
    assert_eq!(title.differences.len(), 1);
    assert_eq!(title.differences[0].field, "title");
    assert_eq!(title.differences[0].expected, "Lesson 1.1 - Variables");
    assert_eq!(title.differences[0].found, "Lesson 1.1 - Variable");
    let section = &cov.mismatch[1];
    assert_eq!(section.display, "1.1.2");
    assert_eq!(section.differences.len(), 1);
    assert_eq!(section.differences[0].field, "section");
    assert_eq!(section.differences[0].expected, "Module 1: Basics");
    assert_eq!(section.differences[0].found, "Module 2: Elsewhere");

    let unexpected: Vec<(Option<&str>, &str)> = cov
        .unexpected
        .iter()
        .map(|u| (u.display.as_deref(), u.reason))
        .collect();
    assert_eq!(
        unexpected,
        [
            (Some("9.9.9"), "is not in the curriculum"),
            (None, "has no display number"),
            (Some("0.1.1"), "repeats an earlier display number"),
        ]
    );
    assert_eq!(cov.unexpected[0].file, "c.md");
    assert_eq!(cov.unexpected[0].line, 16);

    // present + mismatch + missing is everything that was expected.
    assert_eq!(
        cov.present.len() + cov.mismatch.len() + cov.missing.len(),
        cov.expected
    );
    let parts: Vec<(&str, usize, usize, usize, usize)> = cov
        .by_part
        .iter()
        .map(|p| {
            (
                p.part.as_str(),
                p.expected,
                p.present,
                p.mismatch,
                p.missing,
            )
        })
        .collect();
    assert_eq!(
        parts,
        [
            ("0 · Start Here", 1, 1, 0, 0),
            ("1 · Fixture Crash Course", 3, 0, 2, 1),
            ("2 · Fixture Webinars", 2, 0, 0, 2),
        ]
    );

    // A part mismatch is named too.
    let lib = parse(
        "@@part 9 · Wrong part\n@@section Orientation\n\
         @@chapter 0.1.1 | How to use this library | 5\n@@text x\nbody\n",
    );
    let cov = compare(&expected, &lib);
    assert_eq!(cov.mismatch.len(), 1);
    assert_eq!(cov.mismatch[0].differences[0].field, "part");
    assert_eq!(cov.mismatch[0].differences[0].found, "9 · Wrong part");

    // And what the CLI prints says both sides.
    let shown = compare(
        &expected,
        &parse("@@chapter 1.1.1 | Other | 5\n@@text x\nbody\n"),
    )
    .to_string();
    assert!(
        shown.starts_with("0 present, 5 missing, 1 mismatch, 0 unexpected (of 6 expected)"),
        "{shown}"
    );
    assert!(
        shown.contains("expected: Lesson 1.1 - Variables"),
        "{shown}"
    );
    assert!(shown.contains("found:    Other"), "{shown}");
    assert!(shown.contains("found:    (none)"), "{shown}");
    assert!(
        shown.contains("1 · Fixture Crash Course / Module 1: Basics (2): 1.1.2 1.1.3"),
        "{shown}"
    );
}

#[test]
fn the_real_curriculum_has_304_chapters() {
    let Some(md) = real_curriculum() else { return };
    let rows = parse_curriculum(&md).expect("CURRICULUM.md parses");
    assert_eq!(rows.len(), 304);

    // The Totals table at the end of CURRICULUM.md says 4+45+89+8+30+57+66+5.
    let per_part: Vec<usize> = (0..=7)
        .map(|p| {
            rows.iter()
                .filter(|r| r.display.starts_with(&format!("{p}.")))
                .count()
        })
        .collect();
    assert_eq!(per_part, [4, 45, 89, 8, 30, 57, 66, 5]);

    let mut displays: Vec<&str> = rows.iter().map(|r| r.display.as_str()).collect();
    displays.sort_unstable();
    displays.dedup();
    assert_eq!(displays.len(), 304, "display numbers are unique");

    // Every row got a part and a section, and part 7 read its section column.
    assert!(rows
        .iter()
        .all(|r| !r.part.is_empty() && !r.section.is_empty() && !r.title.is_empty()));
    let webinar = rows.iter().find(|r| r.display == "7.5.1").expect("7.5.1");
    assert_eq!(webinar.part, "7 · Rust Webinars");
    assert_eq!(webinar.section, "Debugging & Optimizing Rust");
    assert_eq!(rows.iter().filter(|r| r.kind == "Own").count(), 3);

    let first = &rows[0];
    assert_eq!(
        (
            first.display.as_str(),
            first.part.as_str(),
            first.section.as_str()
        ),
        ("0.0.1", "0 · Start Here", "Ultimate Rust Bundle Guide")
    );
}

#[test]
fn an_empty_library_has_all_304_missing_and_none_present() {
    let Some(md) = real_curriculum() else { return };
    let rows = parse_curriculum(&md).expect("CURRICULUM.md parses");
    let cov = compare(&rows, &Library::default());
    assert_eq!(cov.missing.len(), 304);
    assert!(cov.present.is_empty());
    assert!(cov.mismatch.is_empty() && cov.unexpected.is_empty());
    assert_eq!(cov.by_part.len(), 8);
    assert!(cov
        .to_string()
        .starts_with("0 present, 304 missing, 0 mismatch, 0 unexpected (of 304 expected)"));
}

#[test]
fn a_curriculum_without_rows_is_an_error_not_an_empty_success() {
    assert!(parse_curriculum("# Nothing here\n\nJust prose.\n").is_err());
    // A row with no part above it cannot be placed.
    let err =
        parse_curriculum("| display | title | type |\n|---|---|---|\n| 1.1.1 | T | Video |\n")
            .unwrap_err();
    assert!(err.contains("line 3"), "{err}");
}
