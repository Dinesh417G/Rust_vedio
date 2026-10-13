//! Loading a parsed library into a real (in-memory) course database.

use rusqlite::Connection;
use rustclass_core::models::*;
use rustclass_core::paths::Paths;
use rustclass_core::repo::{block, chapter, course, search, ReadCtx};
use rustclass_core::state::{self, AssetStatus};
use rustclass_core::{db, CoreError};
use rustclass_library::format::{LibBlock, LibChapter};
use rustclass_library::{load, parse_files, read_dir, Library, LoadMode};
use std::collections::HashSet;
use std::path::PathBuf;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/small-library")
}

fn parse(list: &[(&str, &str)]) -> Library {
    let owned: Vec<(String, String)> = list
        .iter()
        .map(|(n, c)| (n.to_string(), c.to_string()))
        .collect();
    parse_files(&owned).unwrap_or_else(|e| panic!("library should parse: {e:?}"))
}

fn fixture() -> Library {
    parse_files(&read_dir(&fixture_dir()).expect("fixture dir")).expect("fixture parses")
}

fn memory() -> Connection {
    db::open_memory().expect("in-memory database")
}

/// A chapter made by hand, the way a teacher does it in the app.
fn hand_made(conn: &mut Connection, title: &str) -> String {
    let paths = Paths::new("unused");
    let none = HashSet::new();
    let ctx = ReadCtx::new(&paths, &none);
    chapter::create(
        conn,
        &ctx,
        NewChapter {
            title: title.into(),
            display_number: None,
            summary_md: None,
            objectives_md: None,
            estimated_minutes: None,
            template: None,
            after_id: None,
            part: None,
            section: None,
        },
    )
    .expect("create chapter")
    .chapter
    .id
}

fn live(conn: &Connection) -> Vec<Chapter> {
    chapter::list_rows(conn, false).expect("list")
}

fn views(conn: &Connection, chapter_id: &str) -> Vec<BlockView> {
    let paths = Paths::new("unused");
    let none = HashSet::new();
    block::list(conn, &ReadCtx::new(&paths, &none), chapter_id).expect("blocks")
}

fn titles(chapters: &[Chapter]) -> Vec<&str> {
    chapters.iter().map(|c| c.title.as_str()).collect()
}

#[test]
fn load_replace_soft_deletes_previous_chapters() {
    let mut conn = memory();
    let alpha = hand_made(&mut conn, "Alpha plan");
    let beta = hand_made(&mut conn, "Beta plan");
    assert_eq!(search::query(&conn, "Alpha", 10).unwrap().len(), 1);

    let lib = fixture();
    let report = load(&mut conn, &lib, LoadMode::Replace).expect("load");
    assert_eq!(report.chapters_removed, 2);
    assert_eq!(report.chapters_created, 3);

    // The two old chapters are in the trash, not gone.
    assert_eq!(
        titles(&live(&conn)),
        [
            "How to use this library",
            "Lesson 1.1 - Variables",
            "Lesson 1.2 - Cargo"
        ]
    );
    let all = chapter::list_rows(&conn, true).unwrap();
    let trashed: Vec<&str> = all
        .iter()
        .filter(|c| c.deleted_at.is_some())
        .map(|c| c.title.as_str())
        .collect();
    assert_eq!(trashed, ["Alpha plan", "Beta plan"]);
    assert!(all.iter().any(|c| c.id == alpha && c.deleted_at.is_some()));
    assert!(all.iter().any(|c| c.id == beta && c.deleted_at.is_some()));

    // The chapter rows of the old chapters left the search index; the new
    // ones are in it.
    assert!(search::query(&conn, "Alpha", 10).unwrap().is_empty());
    let hits = search::query(&conn, "Cargo", 10).unwrap();
    assert!(
        hits.iter().any(|h| h.entity_type == "chapter"),
        "the new chapters are searchable: {hits:?}"
    );

    // Replacing again trashes the previous library's own chapters.
    let again = load(&mut conn, &lib, LoadMode::Replace).expect("second load");
    assert_eq!(again.chapters_removed, 3);
    assert_eq!(again.chapters_created, 3);
    assert_eq!(live(&conn).len(), 3);
    let trashed = chapter::list_rows(&conn, true)
        .unwrap()
        .into_iter()
        .filter(|c| c.deleted_at.is_some())
        .count();
    assert_eq!(trashed, 5);
}

#[test]
fn append_adds_after_the_existing_chapters_and_removes_nothing() {
    let mut conn = memory();
    hand_made(&mut conn, "Alpha plan");
    hand_made(&mut conn, "Beta plan");

    let report = load(&mut conn, &fixture(), LoadMode::Append).expect("load");
    assert_eq!((report.chapters_removed, report.chapters_created), (0, 3));
    assert_eq!(
        titles(&live(&conn)),
        [
            "Alpha plan",
            "Beta plan",
            "How to use this library",
            "Lesson 1.1 - Variables",
            "Lesson 1.2 - Cargo"
        ]
    );
}

#[test]
fn load_keeps_file_order_and_grouping() {
    // Three files, in an order a hash map or a sort by title would scramble,
    // and 150 chapters in the last one to wear out the sort-key arithmetic.
    let mut third = String::from("@@part Part C\n@@section Section C\n");
    for n in 1..=150 {
        third.push_str(&format!(
            "@@chapter 3.1.{n} | Zed {n} | 5\n@@text Notes\nbody {n}\n"
        ));
    }
    let lib = parse(&[
        (
            "1-first.md",
            "@@part Part A\n@@section Section A1\n@@chapter 1.1.1 | Zulu | 5\n@@text x\nbody\n\
             @@chapter 1.1.2 | Alpha | 5\n@@text x\nbody\n\
             @@section Section A2\n@@chapter 1.2.1 | Mike | 5\n@@text x\nbody\n",
        ),
        (
            "2-second.md",
            "@@part Part B\n@@section Section B1\n@@chapter 2.1.1 | Bravo | 5\n@@text x\nbody\n",
        ),
        ("3-third.md", &third),
    ]);

    let mut conn = memory();
    hand_made(&mut conn, "Existing chapter");
    load(&mut conn, &lib, LoadMode::Append).expect("load");

    let rows = live(&conn);
    assert_eq!(rows.len(), 1 + 4 + 150);
    assert_eq!(rows[0].title, "Existing chapter");

    // Same order as the files, chapter by chapter.
    let loaded = &rows[1..];
    let expected: Vec<&LibChapter> = lib.chapters.iter().collect();
    assert_eq!(loaded.len(), expected.len());
    for (row, want) in loaded.iter().zip(&expected) {
        assert_eq!(row.title, want.title);
        assert_eq!(row.display_number, want.display_number);
        assert_eq!(row.part, want.part);
        assert_eq!(row.section, want.section);
        assert_eq!(row.estimated_minutes, want.estimated_minutes);
        assert_eq!(row.status, ChapterStatus::Draft);
    }
    assert_eq!(
        titles(&loaded[..4]),
        ["Zulu", "Alpha", "Mike", "Bravo"],
        "file order, not alphabetical"
    );
    assert_eq!(loaded[2].section.as_deref(), Some("Section A2"));
    assert_eq!(loaded[3].part.as_deref(), Some("Part B"));
    assert_eq!(loaded[153].title, "Zed 150");
    assert_eq!(loaded[153].section.as_deref(), Some("Section C"));

    // Strictly increasing sort keys.
    assert!(rows.windows(2).all(|w| w[0].sort_index < w[1].sort_index));

    // Blocks keep their order inside a chapter, too.
    let fixture_lib = fixture();
    let mut conn = memory();
    load(&mut conn, &fixture_lib, LoadMode::Replace).expect("load");
    let rows = live(&conn);
    assert_eq!(rows.len(), fixture_lib.chapters.len());
    for (row, want) in rows.iter().zip(&fixture_lib.chapters) {
        let got: Vec<(BlockKind, Option<String>)> = views(&conn, &row.id)
            .into_iter()
            .map(|v| (v.block.kind, v.block.heading))
            .collect();
        let want: Vec<(BlockKind, Option<String>)> = want
            .blocks
            .iter()
            .map(|b| (b.kind, b.heading.clone()))
            .collect();
        assert_eq!(got, want, "{}", row.title);
    }
}

#[test]
fn blocks_are_created_empty_or_filled_as_specified() {
    let lib = parse(&[(
        "kinds.md",
        "@@part P\n@@section S\n@@chapter 1.1.1 | Every kind | 10\n\
         @@video Lesson video | Attach the MP4 here.\n\
         @@image Layout | A screenshot goes here.\n\
         @@attachment Sheet | The PDF goes here.\n\
         @@text Notes\nSome prose.\n\
         @@text Empty notes\n\
         @@callout tip | Hint\nTry it.\n\
         @@callout note | Blank callout\n\
         @@code rust | Try it\nfn main() {}\n\
         @@code mermaid | Diagram\nflowchart LR\n  A[\"a\"] --> B[\"b\"]\n\
         @@code bash | Empty code\n\
         @@quiz Check\n? Q?\n+ yes\n- no\n= because\n\
         @@divider\n",
    )]);
    let mut conn = memory();
    load(&mut conn, &lib, LoadMode::Replace).expect("load");

    let chapter_id = live(&conn).remove(0).id;
    let blocks = views(&conn, &chapter_id);
    let shape: Vec<(BlockKind, BlockState)> =
        blocks.iter().map(|v| (v.block.kind, v.state)).collect();
    assert_eq!(
        shape,
        [
            (BlockKind::Video, BlockState::Empty),
            (BlockKind::Image, BlockState::Empty),
            (BlockKind::Attachment, BlockState::Empty),
            (BlockKind::Text, BlockState::Ready),
            (BlockKind::Text, BlockState::Empty),
            (BlockKind::Callout, BlockState::Ready),
            (BlockKind::Callout, BlockState::Empty),
            (BlockKind::Code, BlockState::Ready),
            (BlockKind::Code, BlockState::Ready),
            (BlockKind::Code, BlockState::Empty),
            (BlockKind::Quiz, BlockState::Ready),
            (BlockKind::Divider, BlockState::Ready),
        ]
    );

    // The state really is derived from what was stored, by the core's one rule.
    for v in &blocks {
        assert_eq!(
            state::compute(&v.block, AssetStatus::None, false),
            v.state,
            "{:?}",
            v.block.kind
        );
    }

    // Asset blocks carry their placeholder text and no file.
    let video = &blocks[0].block;
    assert!(video.asset_id.is_none() && blocks[0].asset.is_none());
    assert_eq!(video.heading.as_deref(), Some("Lesson video"));
    assert_eq!(video.caption_md.as_deref(), Some("Attach the MP4 here."));
    assert_eq!(
        blocks[2].block.caption_md.as_deref(),
        Some("The PDF goes here.")
    );

    // Prose, code and metadata landed in the columns the frontend reads.
    let text = &blocks[3].block;
    assert_eq!(text.body_md.as_deref(), Some("Some prose."));
    assert!(text.code_text.is_none() && text.code_lang.is_none());
    assert_eq!(text.meta_json, "{}");

    let callout = &blocks[5].block;
    assert_eq!(callout.body_md.as_deref(), Some("Try it."));
    assert_eq!(callout.meta_json, r#"{"tone":"tip"}"#);

    let rust = &blocks[7].block;
    assert_eq!(rust.code_text.as_deref(), Some("fn main() {}"));
    assert_eq!(rust.code_lang.as_deref(), Some("rust"));
    assert!(rust.body_md.is_none());
    assert_eq!(blocks[8].block.code_lang.as_deref(), Some("mermaid"));
    assert_eq!(blocks[9].block.code_lang.as_deref(), Some("bash"));

    let quiz = &blocks[10].block;
    assert!(state::quiz_has_questions(&quiz.meta_json));
    let meta: serde_json::Value = serde_json::from_str(&quiz.meta_json).unwrap();
    assert_eq!(meta["questions"][0]["answerIndex"], 0);
    assert_eq!(meta["questions"][0]["explanation"], "because");

    // The chapter row carries the header fields.
    let chapter_row = &live(&conn)[0];
    assert_eq!(chapter_row.display_number.as_deref(), Some("1.1.1"));
    assert_eq!(chapter_row.estimated_minutes, Some(10));
}

#[test]
fn the_fixture_library_loads_end_to_end() {
    let files = read_dir(&fixture_dir()).expect("fixture dir");
    let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        names,
        ["0-00-start-here.md", "1-01-basics.md"],
        "lexical order, and _notes.md is skipped"
    );
    let lib = parse_files(&files).expect("fixture parses");
    assert!(lib.warnings.is_empty(), "{:?}", lib.warnings);

    let mut conn = memory();
    let report = load(&mut conn, &lib, LoadMode::Replace).expect("load");
    assert_eq!(report.chapters_created, 3);
    assert_eq!(report.blocks_created, 20);
    assert_eq!(report.chapters_removed, 0);
    assert_eq!(report.files, 2);
    assert!(report.warnings.is_empty());

    // Every block kind is in there.
    let mut kinds = HashSet::new();
    for c in live(&conn) {
        for v in views(&conn, &c.id) {
            kinds.insert(v.block.kind);
        }
    }
    assert_eq!(kinds.len(), 8, "{kinds:?}");

    // The chapter's own header fields.
    let rows = live(&conn);
    assert_eq!(
        rows[1].objectives_md.as_deref().map(|o| o.lines().count()),
        Some(2)
    );
    assert_eq!(
        rows[1].summary_md.as_deref(),
        Some("Bindings, immutability, `mut` and constants.")
    );
}

#[test]
fn the_course_directive_updates_the_course_row() {
    let mut conn = memory();
    let before = course::get_or_seed(&conn).unwrap();
    assert_eq!(before.title, "My Rust Course");

    load(&mut conn, &fixture(), LoadMode::Replace).expect("load");
    let after = course::get_or_seed(&conn).unwrap();
    assert_eq!(after.id, before.id, "still one course");
    assert_eq!(after.title, "Fixture Course");
    assert_eq!(
        after.subtitle.as_deref(),
        Some("A tiny library for loader tests")
    );
    assert_eq!(after.author.as_deref(), Some("RustClass tests"));
    assert!(after
        .description_md
        .as_deref()
        .is_some_and(|d| d.starts_with("This course exists only so the loader")));

    // A library without @@course leaves the course alone.
    let plain = parse(&[(
        "a.md",
        "@@part P\n@@section S\n@@chapter 1.1.1 | T | 5\n@@text x\nbody\n",
    )]);
    load(&mut conn, &plain, LoadMode::Append).expect("load");
    assert_eq!(course::get_or_seed(&conn).unwrap().title, "Fixture Course");
}

#[test]
fn replacing_with_an_empty_library_is_refused_and_deletes_nothing() {
    let mut conn = memory();
    hand_made(&mut conn, "Alpha plan");
    let empty = parse(&[]);
    assert!(empty.chapters.is_empty());

    let err = load(&mut conn, &empty, LoadMode::Replace).expect_err("must refuse");
    assert!(matches!(err, CoreError::Invalid(_)), "{err:?}");
    assert_eq!(titles(&live(&conn)), ["Alpha plan"]);

    // Appending nothing is harmless.
    let report = load(&mut conn, &empty, LoadMode::Append).expect("append nothing");
    assert_eq!(
        (report.chapters_created, report.blocks_created, report.files),
        (0, 0, 0)
    );
    assert_eq!(live(&conn).len(), 1);
}

#[test]
fn a_failed_load_changes_nothing() {
    let mut conn = memory();
    hand_made(&mut conn, "Alpha plan");

    // The parser cannot produce this, but a hand-built library can: the second
    // chapter's block carries metadata that is not JSON, so the insert fails
    // after the first chapter was already written.
    let chapter = |title: &str, meta: Option<&str>| LibChapter {
        file: "x.md".into(),
        line: 1,
        part: None,
        section: None,
        display_number: None,
        title: title.into(),
        summary_md: None,
        objectives_md: None,
        estimated_minutes: None,
        blocks: vec![LibBlock {
            line: 2,
            body_line: 3,
            kind: BlockKind::Text,
            heading: None,
            caption_md: None,
            body_md: Some("body".into()),
            code_text: None,
            code_lang: None,
            meta_json: meta.map(String::from),
        }],
    };
    let lib = Library {
        course: Some(rustclass_library::format::CourseInfo {
            title: Some("Changed title".into()),
            ..Default::default()
        }),
        files: vec!["x.md".into()],
        chapters: vec![chapter("Good", None), chapter("Bad", Some("not json"))],
        warnings: Vec::new(),
    };

    let err = load(&mut conn, &lib, LoadMode::Replace).expect_err("must fail");
    assert!(matches!(err, CoreError::Invalid(_)), "{err:?}");

    // Rolled back: the old chapter is still live, nothing new exists, and the
    // course title is untouched.
    assert_eq!(titles(&live(&conn)), ["Alpha plan"]);
    assert_eq!(chapter::list_rows(&conn, true).unwrap().len(), 1);
    assert_eq!(course::get_or_seed(&conn).unwrap().title, "My Rust Course");
    assert_eq!(search::query(&conn, "Alpha", 10).unwrap().len(), 1);
}

#[test]
fn the_report_and_the_mode_cross_the_ipc_boundary_as_the_frontend_expects() {
    let mut conn = memory();
    let report = load(&mut conn, &fixture(), LoadMode::Replace).expect("load");
    let json = serde_json::to_value(&report).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "chaptersCreated": 3,
            "blocksCreated": 20,
            "chaptersRemoved": 0,
            "files": 2,
            "warnings": []
        })
    );

    assert_eq!(
        serde_json::from_str::<LoadMode>("\"replace\"").unwrap(),
        LoadMode::Replace
    );
    assert_eq!(
        serde_json::from_str::<LoadMode>("\"append\"").unwrap(),
        LoadMode::Append
    );
    assert!(serde_json::from_str::<LoadMode>("\"Replace\"").is_err());
}

#[test]
fn warnings_travel_with_the_report() {
    let lib = parse(&[("w.md", "@@chapter 1.1.1 | Bare | 5\n")]);
    let mut conn = memory();
    let report = load(&mut conn, &lib, LoadMode::Replace).expect("warnings do not block a load");
    assert_eq!(report.chapters_created, 1);
    assert_eq!(report.warnings.len(), 2, "{:?}", report.warnings);
    assert!(report.warnings[0].starts_with("w.md:1: "));
    assert!(report.warnings.iter().any(|w| w.contains("no blocks")));
}
