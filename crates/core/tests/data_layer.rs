//! End-to-end tests over a real SQLite file and a real media directory.

use rustclass_core::assets::{CancelFlag, ImportRequest};
use rustclass_core::bundle::{self, BundleMode};
use rustclass_core::models::*;
use rustclass_core::outline::{self, OutlineMode};
use rustclass_core::paths::Paths;
use rustclass_core::repo::{
    asset as asset_repo, block, chapter, course, progress, search, ReadCtx,
};
use rustclass_core::templates::ChapterTemplate;
use rustclass_core::{db, gc, integrity, state};

/// Chapter title paired with (kind, heading, state) for each of its blocks —
/// the shape a bundle round trip has to reproduce exactly.
type Shape = Vec<(String, Vec<(BlockKind, Option<String>, BlockState)>)>;
use std::collections::HashSet;

struct Harness {
    _dir: tempfile::TempDir,
    conn: rusqlite::Connection,
    paths: Paths,
}

impl Harness {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        let paths = Paths::new(dir.path().join("appdata"));
        paths.ensure().expect("create app dirs");
        let conn = db::open(&paths.db_file()).expect("open db");
        course::get_or_seed(&conn).expect("seed course");
        Self {
            _dir: dir,
            conn,
            paths,
        }
    }

    /// Reopens the database from disk — this is what "restart the app" means.
    fn restart(self) -> Self {
        drop(self.conn);
        let conn = db::open(&self.paths.db_file()).expect("reopen db");
        Self {
            _dir: self._dir,
            conn,
            paths: self.paths,
        }
    }

    fn new_chapter(&mut self, title: &str, template: Option<ChapterTemplate>) -> String {
        let none = HashSet::new();
        let ctx = ReadCtx::new(&self.paths, &none);
        chapter::create(
            &mut self.conn,
            &ctx,
            NewChapter {
                title: title.into(),
                display_number: None,
                summary_md: None,
                objectives_md: None,
                estimated_minutes: None,
                template,
                after_id: None,
            },
        )
        .expect("create chapter")
        .chapter
        .id
    }

    fn add_block(&mut self, chapter_id: &str, kind: BlockKind, heading: &str) -> String {
        let none = HashSet::new();
        let ctx = ReadCtx::new(&self.paths, &none);
        let last = block::list(&self.conn, &ctx, chapter_id)
            .unwrap()
            .last()
            .map(|b| b.block.id.clone());
        block::create(
            &mut self.conn,
            &ctx,
            chapter_id,
            kind,
            last.as_deref(),
            Some(BlockSeed {
                heading: Some(heading.into()),
                ..Default::default()
            }),
        )
        .expect("create block")
        .block
        .id
    }

    fn block_ids(&self, chapter_id: &str) -> Vec<String> {
        let none = HashSet::new();
        let ctx = ReadCtx::new(&self.paths, &none);
        block::list(&self.conn, &ctx, chapter_id)
            .unwrap()
            .into_iter()
            .map(|b| b.block.id)
            .collect()
    }

    fn headings(&self, chapter_id: &str) -> Vec<String> {
        let none = HashSet::new();
        let ctx = ReadCtx::new(&self.paths, &none);
        block::list(&self.conn, &ctx, chapter_id)
            .unwrap()
            .into_iter()
            .map(|b| b.block.heading.unwrap_or_default())
            .collect()
    }

    fn import(&mut self, path: &std::path::Path, block_id: Option<&str>) -> (String, bool) {
        let req = ImportRequest {
            path: path.to_string_lossy().to_string(),
            block_id: block_id.map(String::from),
            kind: None,
            mode: ImportMode::Copy,
        };
        let out = rustclass_core::assets::import(
            &self.conn,
            &self.paths,
            &req,
            &CancelFlag::new(),
            |_, _| {},
        )
        .expect("import");
        if let Some(b) = block_id {
            let none = HashSet::new();
            let ctx = ReadCtx::new(&self.paths, &none);
            block::set_asset(&self.conn, &ctx, b, Some(&out.asset_id)).expect("attach");
        }
        (out.asset_id, out.deduped)
    }
}

fn write_file(dir: &std::path::Path, name: &str, bytes: &[u8]) -> std::path::PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let p = dir.join(name);
    std::fs::write(&p, bytes).unwrap();
    p
}

// ---------------------------------------------------------------------------
// Acceptance test 1 — structure without content, surviving a restart
// ---------------------------------------------------------------------------

#[test]
fn chapter_four_renders_with_no_video_uploaded_and_survives_a_restart() {
    let mut h = Harness::new();
    let none = HashSet::new();

    let chapter_id = {
        let ctx = ReadCtx::new(&h.paths, &none);
        chapter::create(
            &mut h.conn,
            &ctx,
            NewChapter {
                title: "Ownership & Borrowing".into(),
                display_number: Some("4".into()),
                summary_md: Some("How Rust tracks who owns what.".into()),
                objectives_md: None,
                estimated_minutes: Some(35),
                template: None,
                after_id: None,
            },
        )
        .unwrap()
        .chapter
        .id
    };

    let block_id = {
        let ctx = ReadCtx::new(&h.paths, &none);
        block::create(
            &mut h.conn,
            &ctx,
            &chapter_id,
            BlockKind::Video,
            None,
            Some(BlockSeed {
                heading: Some("Move semantics".into()),
                caption_md: Some("This video covers moves, clones and the borrow checker".into()),
                ..Default::default()
            }),
        )
        .unwrap()
        .block
        .id
    };

    // Restart: close the connection and reopen the file from disk.
    let h = h.restart();
    let ctx = ReadCtx::new(&h.paths, &none);

    let view = chapter::get(&h.conn, &ctx, &chapter_id).unwrap();
    assert_eq!(view.chapter.title, "Ownership & Borrowing");
    assert_eq!(view.chapter.display_number.as_deref(), Some("4"));
    assert_eq!(view.chapter.estimated_minutes, Some(35));
    assert_eq!(view.blocks.len(), 1);

    let b = &view.blocks[0];
    assert_eq!(b.block.id, block_id);
    assert_eq!(
        b.state,
        BlockState::Empty,
        "an unfilled video block is Empty"
    );
    assert_eq!(b.block.heading.as_deref(), Some("Move semantics"));
    assert_eq!(
        b.block.caption_md.as_deref(),
        Some("This video covers moves, clones and the borrow checker"),
        "the caption is the whole point of planning first — it must survive"
    );
    assert!(b.asset.is_none());

    // The sidebar histogram reports the hollow dot.
    let summaries = chapter::list(&h.conn, &ctx, false).unwrap();
    let s = summaries
        .iter()
        .find(|s| s.chapter.id == chapter_id)
        .unwrap();
    assert_eq!((s.block_count, s.empty_count, s.ready_count), (1, 1, 0));
}

#[test]
fn a_whole_twelve_chapter_course_can_be_planned_with_no_files() {
    let mut h = Harness::new();
    let none = HashSet::new();

    for i in 1..=12 {
        let template = match i % 3 {
            0 => ChapterTemplate::Practice,
            1 => ChapterTemplate::Lesson,
            _ => ChapterTemplate::Concept,
        };
        h.new_chapter(&format!("Chapter {i}"), Some(template));
    }

    let h = h.restart();
    let ctx = ReadCtx::new(&h.paths, &none);
    let chapters = chapter::list(&h.conn, &ctx, false).unwrap();
    assert_eq!(chapters.len(), 12);

    let total_blocks: u32 = chapters.iter().map(|c| c.block_count).sum();
    assert!(
        total_blocks >= 40,
        "scaffolds should produce real structure"
    );
    // Nothing has been uploaded, so nothing may be Ready or Missing.
    for c in &chapters {
        assert_eq!(
            c.empty_count, c.block_count,
            "chapter `{}` has non-empty blocks despite no uploads",
            c.chapter.title
        );
        assert_eq!(c.missing_count, 0);
    }
    assert_eq!(
        chapters
            .iter()
            .map(|c| c.chapter.title.clone())
            .collect::<Vec<_>>(),
        (1..=12).map(|i| format!("Chapter {i}")).collect::<Vec<_>>(),
        "chapter order must be stable across a restart"
    );
}

// ---------------------------------------------------------------------------
// Ordering
// ---------------------------------------------------------------------------

#[test]
fn blocks_reorder_within_a_chapter_and_persist() {
    let mut h = Harness::new();
    let c = h.new_chapter("Ordering", None);
    let a = h.add_block(&c, BlockKind::Text, "A");
    let b = h.add_block(&c, BlockKind::Text, "B");
    let d = h.add_block(&c, BlockKind::Text, "C");
    assert_eq!(h.headings(&c), vec!["A", "B", "C"]);

    // Move C to the front.
    block::reorder(&mut h.conn, &d, &c, None).unwrap();
    assert_eq!(h.headings(&c), vec!["C", "A", "B"]);

    // Move A after B.
    block::reorder(&mut h.conn, &a, &c, Some(&b)).unwrap();
    assert_eq!(h.headings(&c), vec!["C", "B", "A"]);

    let h = h.restart();
    assert_eq!(
        h.headings(&c),
        vec!["C", "B", "A"],
        "order must survive a restart"
    );
}

#[test]
fn blocks_move_across_a_chapter_boundary_and_persist() {
    let mut h = Harness::new();
    let c1 = h.new_chapter("One", None);
    let c2 = h.new_chapter("Two", None);
    let a = h.add_block(&c1, BlockKind::Text, "A");
    h.add_block(&c1, BlockKind::Text, "B");
    let x = h.add_block(&c2, BlockKind::Text, "X");

    block::reorder(&mut h.conn, &a, &c2, Some(&x)).unwrap();

    assert_eq!(h.headings(&c1), vec!["B"]);
    assert_eq!(h.headings(&c2), vec!["X", "A"]);

    let h = h.restart();
    assert_eq!(h.headings(&c1), vec!["B"]);
    assert_eq!(h.headings(&c2), vec!["X", "A"]);
}

#[test]
fn repeated_inserts_at_one_spot_trigger_a_rebalance_without_losing_order() {
    let mut h = Harness::new();
    let c = h.new_chapter("Dense", None);
    let first = h.add_block(&c, BlockKind::Text, "first");
    h.add_block(&c, BlockKind::Text, "last");

    // Always insert directly after `first`, which halves the same gap each time.
    let mut expected = vec!["first".to_string()];
    for i in 0..80 {
        let name = format!("mid{i}");
        let none = HashSet::new();
        let ctx = ReadCtx::new(&h.paths, &none);
        block::create(
            &mut h.conn,
            &ctx,
            &c,
            BlockKind::Text,
            Some(&first),
            Some(BlockSeed {
                heading: Some(name.clone()),
                ..Default::default()
            }),
        )
        .unwrap();
        expected.insert(1, name);
    }
    expected.push("last".into());

    assert_eq!(h.headings(&c), expected, "order held through the rebalance");

    // And the keys are healthy again rather than sitting on top of each other.
    let keys: Vec<f64> = {
        let mut s = h
            .conn
            .prepare("SELECT sort_index FROM block WHERE chapter_id = ?1 ORDER BY sort_index")
            .unwrap();
        let v = s
            .query_map([&c], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<Vec<f64>>>()
            .unwrap();
        v
    };
    assert!(
        !rustclass_core::sortindex::needs_rebalance(&keys),
        "keys should have been spread out again"
    );
}

#[test]
fn chapters_reorder_and_persist() {
    let mut h = Harness::new();
    let a = h.new_chapter("A", None);
    let b = h.new_chapter("B", None);
    let c = h.new_chapter("C", None);
    chapter::reorder(&mut h.conn, &c, None).unwrap();

    let none = HashSet::new();
    let titles = |h: &Harness| {
        let ctx = ReadCtx::new(&h.paths, &none);
        chapter::list(&h.conn, &ctx, false)
            .unwrap()
            .into_iter()
            .map(|c| c.chapter.title)
            .collect::<Vec<_>>()
    };
    assert_eq!(titles(&h), vec!["C", "A", "B"]);
    chapter::reorder(&mut h.conn, &a, Some(&b)).unwrap();
    assert_eq!(titles(&h), vec!["C", "B", "A"]);
    let h = h.restart();
    assert_eq!(titles(&h), vec!["C", "B", "A"]);
}

// ---------------------------------------------------------------------------
// Block state across the payload matrix
// ---------------------------------------------------------------------------

#[test]
fn filling_a_block_flips_it_from_empty_to_ready() {
    let mut h = Harness::new();
    let c = h.new_chapter("States", None);
    let none = HashSet::new();

    let text = h.add_block(&c, BlockKind::Text, "Prose");
    let code = h.add_block(&c, BlockKind::Code, "Sample");
    let quiz = h.add_block(&c, BlockKind::Quiz, "Check");
    let divider = h.add_block(&c, BlockKind::Divider, "");

    {
        let ctx = ReadCtx::new(&h.paths, &none);
        for id in [&text, &code, &quiz] {
            assert_eq!(
                block::get(&h.conn, &ctx, id).unwrap().state,
                BlockState::Empty
            );
        }
        assert_eq!(
            block::get(&h.conn, &ctx, &divider).unwrap().state,
            BlockState::Ready,
            "a divider has no payload to be missing"
        );
    }

    {
        let ctx = ReadCtx::new(&h.paths, &none);
        block::update(
            &h.conn,
            &ctx,
            &text,
            BlockPatch {
                body_md: Some(Some("Ownership means...".into())),
                ..Default::default()
            },
        )
        .unwrap();
        block::update(
            &h.conn,
            &ctx,
            &code,
            BlockPatch {
                code_text: Some(Some("fn main() {}".into())),
                ..Default::default()
            },
        )
        .unwrap();
        block::update(
            &h.conn,
            &ctx,
            &quiz,
            BlockPatch {
                meta_json: Some(r#"{"questions":[{"prompt":"Who owns it?"}]}"#.into()),
                ..Default::default()
            },
        )
        .unwrap();
    }

    let h = h.restart();
    let ctx = ReadCtx::new(&h.paths, &none);
    for id in [&text, &code, &quiz] {
        assert_eq!(
            block::get(&h.conn, &ctx, id).unwrap().state,
            BlockState::Ready
        );
    }
}

#[test]
fn an_importing_block_reports_importing_regardless_of_payload() {
    let mut h = Harness::new();
    let c = h.new_chapter("Importing", None);
    let v = h.add_block(&c, BlockKind::Video, "Lesson");
    let importing: HashSet<String> = [v.clone()].into_iter().collect();
    let ctx = ReadCtx::new(&h.paths, &importing);
    assert_eq!(
        block::get(&h.conn, &ctx, &v).unwrap().state,
        BlockState::Importing
    );
}

#[test]
fn converting_a_block_keeps_the_planning_layer_and_reports_what_it_drops() {
    let mut h = Harness::new();
    let c = h.new_chapter("Convert", None);
    let none = HashSet::new();
    let id = h.add_block(&c, BlockKind::Text, "Intro");
    {
        let ctx = ReadCtx::new(&h.paths, &none);
        block::update(
            &h.conn,
            &ctx,
            &id,
            BlockPatch {
                caption_md: Some(Some("what this section is about".into())),
                body_md: Some(Some("a paragraph I already wrote".into())),
                ..Default::default()
            },
        )
        .unwrap();
    }

    let row = block::get_row(&h.conn, &id).unwrap();
    assert_eq!(
        block::convert_losses(&row, BlockKind::Video),
        vec!["the written body"]
    );
    assert!(
        block::convert_losses(&row, BlockKind::Callout).is_empty(),
        "text -> callout keeps the body"
    );

    let ctx = ReadCtx::new(&h.paths, &none);
    let out = block::convert(&h.conn, &ctx, &id, BlockKind::Video).unwrap();
    assert_eq!(out.block.kind, BlockKind::Video);
    assert_eq!(out.block.heading.as_deref(), Some("Intro"));
    assert_eq!(
        out.block.caption_md.as_deref(),
        Some("what this section is about"),
        "heading and caption outlive any payload"
    );
    assert_eq!(out.state, BlockState::Empty);
}

// ---------------------------------------------------------------------------
// Assets
// ---------------------------------------------------------------------------

#[test]
fn importing_a_file_makes_the_block_ready_and_a_deleted_file_makes_it_missing() {
    let mut h = Harness::new();
    let src_dir = h._dir.path().join("source");
    let src = write_file(&src_dir, "lesson.mp4", b"pretend this is an mp4");
    let c = h.new_chapter("Media", None);
    let v = h.add_block(&c, BlockKind::Video, "Move semantics");
    let none = HashSet::new();

    let (asset_id, deduped) = h.import(&src, Some(&v));
    assert!(!deduped);

    {
        let ctx = ReadCtx::new(&h.paths, &none);
        let view = block::get(&h.conn, &ctx, &v).unwrap();
        assert_eq!(view.state, BlockState::Ready);
        let a = view.asset.expect("asset attached");
        assert!(a.file_exists);
        assert_eq!(a.original_name, "lesson.mp4");
        assert!(
            a.rel_path.ends_with(".mp4") && a.rel_path.contains('/'),
            "stored path should be media/<yyyy-mm>/<id>.mp4, got {}",
            a.rel_path
        );
        assert!(
            std::path::Path::new(&a.resolved_path).starts_with(h.paths.media_root()),
            "a copied asset must live inside the media root"
        );
    }

    // The original is never touched by an import.
    assert!(src.is_file(), "the user's own file must survive the import");

    // Now delete the managed copy from underneath the app.
    let managed = asset_repo::get_row(&h.conn, &asset_id)
        .unwrap()
        .absolute(&h.paths);
    std::fs::remove_file(&managed).unwrap();

    let h = h.restart();
    let ctx = ReadCtx::new(&h.paths, &none);
    let view = block::get(&h.conn, &ctx, &v).unwrap();
    assert_eq!(view.state, BlockState::Missing);
    assert!(!view.asset.as_ref().unwrap().file_exists);

    let issues = integrity::check(&h.conn, &h.paths).unwrap();
    assert!(
        issues.iter().any(|i| i.kind == "missing_file"),
        "integrity_check should report the missing file: {issues:?}"
    );

    // Relinking to a recovered copy brings it back to Ready.
    asset_repo::relink(&h.conn, &h.paths, &asset_id, &src.to_string_lossy()).unwrap();
    let view = block::get(&h.conn, &ctx, &v).unwrap();
    assert_eq!(view.state, BlockState::Ready);
}

#[test]
fn the_same_file_under_a_different_name_is_deduped() {
    let mut h = Harness::new();
    let src_dir = h._dir.path().join("source");
    let a = write_file(&src_dir, "lesson.mp4", b"identical bytes for both copies");
    let b = write_file(
        &src_dir,
        "lesson-final-v2.mp4",
        b"identical bytes for both copies",
    );

    let c = h.new_chapter("Dedupe", None);
    let v1 = h.add_block(&c, BlockKind::Video, "First");
    let v2 = h.add_block(&c, BlockKind::Video, "Second");

    let (id1, dedup1) = h.import(&a, Some(&v1));
    let (id2, dedup2) = h.import(&b, Some(&v2));

    assert!(!dedup1);
    assert!(dedup2, "matching SHA-256 and length must reuse the row");
    assert_eq!(id1, id2);

    let files = integrity::walk_media(&h.paths).unwrap();
    assert_eq!(files.len(), 1, "the media folder must not grow: {files:?}");

    // Both blocks are Ready and point at the same asset.
    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    for v in [&v1, &v2] {
        let view = block::get(&h.conn, &ctx, v).unwrap();
        assert_eq!(view.state, BlockState::Ready);
        assert_eq!(view.asset.unwrap().id, id1);
    }
}

#[test]
fn detaching_an_asset_returns_the_block_to_empty_and_keeps_the_file() {
    let mut h = Harness::new();
    let src = write_file(&h._dir.path().join("source"), "diagram.png", b"png bytes");
    let c = h.new_chapter("Detach", None);
    let img = h.add_block(&c, BlockKind::Image, "Stack vs heap");
    let (asset_id, _) = h.import(&src, Some(&img));

    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    let out = block::set_asset(&h.conn, &ctx, &img, None).unwrap();
    assert_eq!(out.state, BlockState::Empty);
    assert!(out.asset.is_none());

    let row = asset_repo::get_row(&h.conn, &asset_id).unwrap();
    assert!(
        row.absolute(&h.paths).is_file(),
        "detaching must not delete the file"
    );
}

#[test]
fn a_block_refuses_an_asset_of_the_wrong_kind() {
    let mut h = Harness::new();
    let png = write_file(&h._dir.path().join("source"), "diagram.png", b"png bytes");
    let c = h.new_chapter("Kinds", None);
    let video_block = h.add_block(&c, BlockKind::Video, "Lesson");
    // A text block has nowhere to put a file at all.
    let text = h.add_block(&c, BlockKind::Text, "Prose");
    let (asset_id, _) = h.import(&png, None);

    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    let err = block::set_asset(&h.conn, &ctx, &video_block, Some(&asset_id)).unwrap_err();
    assert!(
        matches!(err, rustclass_core::CoreError::Invalid(_)),
        "{err:?}"
    );
    assert!(block::set_asset(&h.conn, &ctx, &text, Some(&asset_id)).is_err());
}

#[test]
fn deleting_a_chapter_never_touches_its_asset_files() {
    let mut h = Harness::new();
    let src = write_file(&h._dir.path().join("source"), "lesson.mp4", b"video bytes");
    let c = h.new_chapter("Doomed", None);
    let v = h.add_block(&c, BlockKind::Video, "Lesson");
    let (asset_id, _) = h.import(&src, Some(&v));
    let managed = asset_repo::get_row(&h.conn, &asset_id)
        .unwrap()
        .absolute(&h.paths);

    chapter::soft_delete(&h.conn, &c).unwrap();
    assert!(managed.is_file());
    chapter::purge(&h.conn, &c).unwrap();
    assert!(managed.is_file(), "purging a chapter must not delete bytes");

    // Only GC does, and only once it is told to.
    let report = gc::run(&h.conn, &h.paths, true).unwrap();
    assert_eq!(report.unreferenced_assets.len(), 1);
    assert!(managed.is_file(), "a dry run must not delete anything");

    gc::run(&h.conn, &h.paths, false).unwrap();
    assert!(!managed.is_file(), "a confirmed GC does delete it");
}

#[test]
fn gc_reports_orphan_files_and_leaves_referenced_ones_alone() {
    let mut h = Harness::new();
    let src = write_file(&h._dir.path().join("source"), "keep.mp4", b"keep me");
    let c = h.new_chapter("GC", None);
    let v = h.add_block(&c, BlockKind::Video, "Lesson");
    h.import(&src, Some(&v));

    // A stray file nothing points at, plus a leftover .part from a crash.
    let stray = write_file(&h.paths.media_root().join("2020-01"), "stray.mp4", b"junk");
    let part = write_file(
        &h.paths.media_root().join("2020-01"),
        "half.mp4.part",
        b"incomplete",
    );

    let report = gc::run(&h.conn, &h.paths, true).unwrap();
    assert_eq!(report.orphan_files.len(), 1);
    assert_eq!(report.partial_files.len(), 1);
    assert!(report.unreferenced_assets.is_empty());
    assert!(report.bytes_reclaimable > 0);

    gc::run(&h.conn, &h.paths, false).unwrap();
    assert!(!stray.exists());
    assert!(!part.exists());
    let files = integrity::walk_media(&h.paths).unwrap();
    assert_eq!(files.len(), 1, "the referenced file must survive");
}

#[test]
fn startup_sweeps_partial_imports_left_by_a_crash() {
    let h = Harness::new();
    write_file(&h.paths.media_root().join("2026-01"), "a.mp4.part", b"x");
    write_file(&h.paths.media_root().join("2026-01"), "b.mp4", b"y");
    assert_eq!(gc::sweep_partials(&h.paths).unwrap(), 1);
    let files = integrity::walk_media(&h.paths).unwrap();
    assert_eq!(files.len(), 1);
}

// ---------------------------------------------------------------------------
// Trash
// ---------------------------------------------------------------------------

#[test]
fn soft_deleting_a_chapter_hides_it_then_restores_it_intact() {
    let mut h = Harness::new();
    let src = write_file(&h._dir.path().join("source"), "lesson.mp4", b"video bytes");
    let c = h.new_chapter("Trashed", None);
    let v = h.add_block(&c, BlockKind::Video, "Lesson");
    h.add_block(&c, BlockKind::Text, "Notes");
    h.import(&src, Some(&v));

    chapter::soft_delete(&h.conn, &c).unwrap();
    let none = HashSet::new();
    {
        let ctx = ReadCtx::new(&h.paths, &none);
        assert!(chapter::list(&h.conn, &ctx, false).unwrap().is_empty());
        assert_eq!(chapter::list(&h.conn, &ctx, true).unwrap().len(), 1);
        // A deleted chapter drops out of search.
        assert!(search::query(&h.conn, "Trashed", 10).unwrap().is_empty());
    }

    let h = h.restart();
    chapter::restore(&h.conn, &c).unwrap();
    let ctx = ReadCtx::new(&h.paths, &none);
    let view = chapter::get(&h.conn, &ctx, &c).unwrap();
    assert_eq!(view.blocks.len(), 2, "blocks come back with the chapter");
    assert_eq!(
        view.blocks[0].state,
        BlockState::Ready,
        "the asset is still attached"
    );
    assert!(!search::query(&h.conn, "Trashed", 10).unwrap().is_empty());
}

#[test]
fn expired_trash_is_reported_and_fresh_trash_is_not() {
    let mut h = Harness::new();
    let old = h.new_chapter("Old", None);
    let fresh = h.new_chapter("Fresh", None);
    chapter::soft_delete(&h.conn, &old).unwrap();
    chapter::soft_delete(&h.conn, &fresh).unwrap();
    // Backdate one deletion past the retention window.
    h.conn
        .execute(
            "UPDATE chapter SET deleted_at = ?2 WHERE id = ?1",
            rusqlite::params![
                &old,
                rustclass_core::time::now_ms() - gc::TRASH_RETENTION_MS - 1
            ],
        )
        .unwrap();
    let (chapters, _) = gc::expired_trash(&h.conn, gc::TRASH_RETENTION_MS).unwrap();
    assert_eq!(chapters, vec![old]);
}

// ---------------------------------------------------------------------------
// Outline import
// ---------------------------------------------------------------------------

#[test]
fn a_ten_chapter_outline_creates_every_chapter_and_typed_block() {
    let mut h = Harness::new();
    let mut md = String::from("# Rust from the ground up\n");
    for i in 1..=10 {
        md.push_str(&format!(
            "## {i}. Chapter {i}\n- [video] Lesson {i}\n- [text] Notes {i}\n- [code] Example {i}\n"
        ));
    }
    let parsed = outline::parse(&md);
    assert_eq!(parsed.chapters.len(), 10);
    assert_eq!(parsed.block_count(), 30);

    let report = rustclass_core::apply_outline(&mut h.conn, &parsed, OutlineMode::Append).unwrap();
    assert_eq!(report.chapters_created, 10);
    assert_eq!(report.blocks_created, 30);
    assert!(report.course_title_changed);

    let h = h.restart();
    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    assert_eq!(
        course::get_or_seed(&h.conn).unwrap().title,
        "Rust from the ground up"
    );
    let chapters = chapter::list(&h.conn, &ctx, false).unwrap();
    assert_eq!(chapters.len(), 10);
    assert_eq!(
        chapters
            .iter()
            .map(|c| c.chapter.title.clone())
            .collect::<Vec<_>>(),
        (1..=10).map(|i| format!("Chapter {i}")).collect::<Vec<_>>(),
        "outline order is preserved"
    );

    let first = chapter::get(&h.conn, &ctx, &chapters[0].chapter.id).unwrap();
    assert_eq!(
        first
            .blocks
            .iter()
            .map(|b| b.block.kind)
            .collect::<Vec<_>>(),
        vec![BlockKind::Video, BlockKind::Text, BlockKind::Code],
        "block kinds and order come straight from the tags"
    );
    assert_eq!(first.blocks[0].block.heading.as_deref(), Some("Lesson 1"));
    // Every generated block is a placeholder, which is the whole point.
    assert!(first.blocks.iter().all(|b| b.state == BlockState::Empty));
}

#[test]
fn replace_mode_moves_the_old_chapters_to_the_trash_rather_than_erasing_them() {
    let mut h = Harness::new();
    h.new_chapter("Original", None);
    let parsed = outline::parse("## Fresh start\n- [text] Hello");
    let report = rustclass_core::apply_outline(&mut h.conn, &parsed, OutlineMode::Replace).unwrap();
    assert_eq!(report.chapters_removed, 1);

    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    let live = chapter::list(&h.conn, &ctx, false).unwrap();
    assert_eq!(live.len(), 1);
    assert_eq!(live[0].chapter.title, "Fresh start");
    let all = chapter::list(&h.conn, &ctx, true).unwrap();
    assert_eq!(
        all.len(),
        2,
        "the replaced chapter is recoverable from Trash"
    );
}

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

#[test]
fn search_finds_chapters_and_blocks_and_follows_edits() {
    let mut h = Harness::new();
    let c = h.new_chapter("Ownership and Borrowing", None);
    let b = h.add_block(&c, BlockKind::Text, "Move semantics");
    let none = HashSet::new();
    {
        let ctx = ReadCtx::new(&h.paths, &none);
        block::update(
            &h.conn,
            &ctx,
            &b,
            BlockPatch {
                body_md: Some(Some("A move transfers ownership of the allocation.".into())),
                ..Default::default()
            },
        )
        .unwrap();
    }

    let hits = search::query(&h.conn, "ownership", 20).unwrap();
    assert!(
        hits.len() >= 2,
        "chapter and block should both match: {hits:?}"
    );
    assert!(hits.iter().all(|hit| hit.chapter_id == c));

    // Editing the body re-indexes it.
    {
        let ctx = ReadCtx::new(&h.paths, &none);
        block::update(
            &h.conn,
            &ctx,
            &b,
            BlockPatch {
                body_md: Some(Some("Completely different words now.".into())),
                ..Default::default()
            },
        )
        .unwrap();
    }
    let hits = search::query(&h.conn, "allocation", 20).unwrap();
    assert!(hits.is_empty(), "stale text must leave the index: {hits:?}");
    assert!(!search::query(&h.conn, "different", 20).unwrap().is_empty());

    // Deleting removes it entirely.
    block::soft_delete(&h.conn, &b).unwrap();
    assert!(search::query(&h.conn, "different", 20).unwrap().is_empty());
}

#[test]
fn search_does_not_break_on_punctuation() {
    let mut h = Harness::new();
    h.new_chapter("Ownership & Borrowing", None);
    // Any of these would be an FTS5 syntax error if passed through raw.
    for q in [r#"" OR ""#, "*", "AND", "a(b", "-", "ownership \"and"] {
        assert!(
            search::query(&h.conn, q, 10).is_ok(),
            "query `{q}` must not error"
        );
    }
}

// ---------------------------------------------------------------------------
// Progress
// ---------------------------------------------------------------------------

#[test]
fn progress_accumulates_and_rolls_up_to_the_course() {
    let mut h = Harness::new();
    let c1 = h.new_chapter("One", None);
    let c2 = h.new_chapter("Two", None);
    let a = h.add_block(&c1, BlockKind::Text, "A");
    let b = h.add_block(&c1, BlockKind::Text, "B");
    h.add_block(&c2, BlockKind::Text, "C");
    h.add_block(&c1, BlockKind::Divider, "");

    progress::set(
        &h.conn,
        &a,
        ProgressPatch {
            completed: Some(true),
            last_position_secs: Some(124.0),
            watched_secs: Some(120.0),
        },
    )
    .unwrap();

    // Scrubbing backwards must not reduce accumulated watch time.
    progress::set(
        &h.conn,
        &a,
        ProgressPatch {
            last_position_secs: Some(10.0),
            watched_secs: Some(5.0),
            ..Default::default()
        },
    )
    .unwrap();
    let p = progress::get(&h.conn, &a).unwrap().unwrap();
    assert_eq!(p.watched_secs, 120.0);
    assert_eq!(
        p.last_position_secs, 10.0,
        "resume position does follow the scrub"
    );
    assert!(p.completed);

    let course_progress = chapter::progress(&h.conn).unwrap();
    assert_eq!(
        course_progress.total_blocks, 3,
        "dividers are not units of work"
    );
    assert_eq!(course_progress.completed_blocks, 1);
    assert_eq!(course_progress.last_block_id.as_deref(), Some(a.as_str()));

    progress::set_chapter(&h.conn, &c1, true).unwrap();
    let course_progress = chapter::progress(&h.conn).unwrap();
    assert_eq!(course_progress.completed_blocks, 2);
    let ch1 = course_progress
        .chapters
        .iter()
        .find(|x| x.chapter_id == c1)
        .unwrap();
    assert_eq!(ch1.percent, 100.0);
    assert!(progress::get(&h.conn, &b).unwrap().unwrap().completed);
}

// ---------------------------------------------------------------------------
// Bundle round trip
// ---------------------------------------------------------------------------

#[test]
fn a_bundle_round_trip_reproduces_the_tree_and_the_media() {
    let mut h = Harness::new();
    let src_dir = h._dir.path().join("source");
    let video = write_file(&src_dir, "lesson.mp4", b"a video worth keeping");

    let c1 = h.new_chapter("Ownership", Some(ChapterTemplate::Lesson));
    let c2 = h.new_chapter("Borrowing", None);
    let v = h.block_ids(&c1)[1].clone(); // the Video block from the Lesson scaffold
    h.import(&video, Some(&v));
    let text = h.add_block(&c2, BlockKind::Text, "Notes");
    let none = HashSet::new();
    {
        let ctx = ReadCtx::new(&h.paths, &none);
        block::update(
            &h.conn,
            &ctx,
            &text,
            BlockPatch {
                body_md: Some(Some("Borrowing lets you look without taking.".into())),
                ..Default::default()
            },
        )
        .unwrap();
    }
    progress::set(
        &h.conn,
        &text,
        ProgressPatch {
            completed: Some(true),
            ..Default::default()
        },
    )
    .unwrap();

    let before: Shape = {
        let ctx = ReadCtx::new(&h.paths, &none);
        chapter::list(&h.conn, &ctx, false)
            .unwrap()
            .into_iter()
            .map(|c| {
                let blocks = chapter::get(&h.conn, &ctx, &c.chapter.id)
                    .unwrap()
                    .blocks
                    .into_iter()
                    .map(|b| (b.block.kind, b.block.heading, b.state))
                    .collect();
                (c.chapter.title, blocks)
            })
            .collect()
    };

    let bundle_path = h._dir.path().join("course.rcbundle");
    let report = bundle::export(&h.conn, &h.paths, &bundle_path, true, "test").unwrap();
    assert_eq!(report.chapters, 2);
    assert_eq!(report.media_files, 1);
    assert!(report.bytes > 0);
    assert!(bundle_path.is_file());

    // Import into a completely fresh app data directory.
    let mut fresh = Harness::new();
    let import_report = bundle::import(
        &mut fresh.conn,
        &fresh.paths,
        &bundle_path,
        BundleMode::Replace,
    )
    .unwrap();
    assert_eq!(import_report.chapters, 2);
    assert_eq!(import_report.media_files, 1);

    let fresh = fresh.restart();
    let ctx = ReadCtx::new(&fresh.paths, &none);
    let after: Shape = chapter::list(&fresh.conn, &ctx, false)
        .unwrap()
        .into_iter()
        .map(|c| {
            let blocks = chapter::get(&fresh.conn, &ctx, &c.chapter.id)
                .unwrap()
                .blocks
                .into_iter()
                .map(|b| (b.block.kind, b.block.heading, b.state))
                .collect();
            (c.chapter.title, blocks)
        })
        .collect();

    assert_eq!(
        after, before,
        "the imported tree must match the exported one"
    );

    // The video really is playable in the new location, not just referenced.
    let restored = chapter::list(&fresh.conn, &ctx, false).unwrap();
    let ch = chapter::get(&fresh.conn, &ctx, &restored[0].chapter.id).unwrap();
    let vb = ch
        .blocks
        .iter()
        .find(|b| b.block.kind == BlockKind::Video)
        .unwrap();
    assert_eq!(vb.state, BlockState::Ready);
    let asset = vb.asset.as_ref().unwrap();
    assert!(asset.file_exists);
    assert_eq!(
        std::fs::read(&asset.resolved_path).unwrap(),
        b"a video worth keeping"
    );
    assert!(
        std::path::Path::new(&asset.resolved_path).starts_with(fresh.paths.media_root()),
        "imported media must land under the new media root"
    );

    // Search works against the imported course, so the index was rebuilt.
    assert!(!search::query(&fresh.conn, "borrowing", 10)
        .unwrap()
        .is_empty());
    // And progress came across.
    assert_eq!(chapter::progress(&fresh.conn).unwrap().completed_blocks, 1);
}

#[test]
fn a_bundle_without_media_imports_and_flags_the_blocks_as_missing() {
    let mut h = Harness::new();
    let video = write_file(&h._dir.path().join("source"), "lesson.mp4", b"bytes");
    let c = h.new_chapter("Ownership", None);
    let v = h.add_block(&c, BlockKind::Video, "Lesson");
    h.import(&video, Some(&v));

    let path = h._dir.path().join("nomedia.rcbundle");
    bundle::export(&h.conn, &h.paths, &path, false, "test").unwrap();

    let mut fresh = Harness::new();
    let report = bundle::import(&mut fresh.conn, &fresh.paths, &path, BundleMode::Replace).unwrap();
    assert_eq!(report.media_files, 0);
    assert!(
        !report.warnings.is_empty(),
        "the user must be told media is absent"
    );

    let none = HashSet::new();
    let ctx = ReadCtx::new(&fresh.paths, &none);
    let chapters = chapter::list(&fresh.conn, &ctx, false).unwrap();
    let ch = chapter::get(&fresh.conn, &ctx, &chapters[0].chapter.id).unwrap();
    assert_eq!(ch.blocks[0].state, BlockState::Missing);
}

#[test]
fn merge_mode_keeps_what_is_already_there() {
    let mut h = Harness::new();
    h.new_chapter("Existing", None);
    let path = h._dir.path().join("one.rcbundle");
    bundle::export(&h.conn, &h.paths, &path, false, "test").unwrap();

    // Import the bundle back into the same database.
    bundle::import(&mut h.conn, &h.paths, &path, BundleMode::Merge).unwrap();
    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    let chapters = chapter::list(&h.conn, &ctx, false).unwrap();
    assert_eq!(chapters.len(), 2, "merge appends rather than replacing");
    assert_eq!(chapters[0].chapter.title, "Existing");
    assert_eq!(chapters[1].chapter.title, "Existing");
    assert_ne!(
        chapters[0].chapter.id, chapters[1].chapter.id,
        "IDs are freshened"
    );
}

#[test]
fn a_bundle_from_a_newer_schema_is_refused_with_an_explanation() {
    let h = Harness::new();
    let path = h._dir.path().join("future.rcbundle");
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file::<_, ()>("manifest.json", zip::write::FileOptions::default())
        .unwrap();
    use std::io::Write;
    zip.write_all(
        br#"{"schemaVersion":999,"appVersion":"9.9","exportedAt":0,"includesMedia":false,"tables":{}}"#,
    )
    .unwrap();
    zip.finish().unwrap();

    let mut h = h;
    let err = bundle::import(&mut h.conn, &h.paths, &path, BundleMode::Merge).unwrap_err();
    assert!(err.to_string().contains("newer RustClass"), "{err}");
}

// ---------------------------------------------------------------------------
// Misc
// ---------------------------------------------------------------------------

#[test]
fn duplicating_a_chapter_copies_structure_and_shares_assets() {
    let mut h = Harness::new();
    let src = write_file(&h._dir.path().join("source"), "lesson.mp4", b"video");
    let c = h.new_chapter("Template", Some(ChapterTemplate::Lesson));
    let v = h.block_ids(&c)[1].clone();
    let (asset_id, _) = h.import(&src, Some(&v));

    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    let copy = chapter::duplicate(&mut h.conn, &ctx, &c).unwrap();
    assert_eq!(copy.chapter.title, "Template (copy)");
    assert_eq!(copy.blocks.len(), 4);
    assert_ne!(copy.blocks[1].block.id, v, "blocks get fresh IDs");
    assert_eq!(
        copy.blocks[1].asset.as_ref().unwrap().id,
        asset_id,
        "the asset is shared, not re-copied"
    );
    let files = integrity::walk_media(&h.paths).unwrap();
    assert_eq!(files.len(), 1, "duplicating must not duplicate bytes");
}

#[test]
fn a_non_ascii_filename_survives_the_round_trip() {
    let mut h = Harness::new();
    let src = write_file(
        &h._dir.path().join("source"),
        "Владение и заимствование.mp4",
        b"bytes",
    );
    let c = h.new_chapter("Unicode", None);
    let v = h.add_block(&c, BlockKind::Video, "Lesson");
    let (asset_id, _) = h.import(&src, Some(&v));

    let row = asset_repo::get_row(&h.conn, &asset_id).unwrap();
    assert_eq!(row.original_name, "Владение и заимствование.mp4");
    assert!(row.absolute(&h.paths).is_file());
    let none = HashSet::new();
    let ctx = ReadCtx::new(&h.paths, &none);
    assert_eq!(
        block::get(&h.conn, &ctx, &v).unwrap().state,
        BlockState::Ready
    );
}

#[test]
fn system_info_reports_real_numbers() {
    let mut h = Harness::new();
    let src = write_file(&h._dir.path().join("source"), "lesson.mp4", b"0123456789");
    let c = h.new_chapter("Info", Some(ChapterTemplate::Lesson));
    let v = h.block_ids(&c)[1].clone();
    h.import(&src, Some(&v));

    let info = rustclass_core::system_info(&h.conn, &h.paths, "0.1.0").unwrap();
    assert_eq!(info.chapter_count, 1);
    assert_eq!(info.block_count, 4);
    assert_eq!(info.asset_count, 1);
    assert_eq!(info.asset_bytes_tracked, 10);
    assert_eq!(info.media_bytes, 10);
    assert!(info.db_bytes > 0);
    assert_eq!(info.schema_version, db::migrations::target_version());
}

#[test]
fn block_state_never_reaches_the_database() {
    // A guard against the one modelling mistake this app cannot afford.
    let h = Harness::new();
    let mut stmt = h
        .conn
        .prepare("SELECT sql FROM sqlite_master WHERE type = 'table'")
        .unwrap();
    let schema: Vec<String> = stmt
        .query_map([], |r| r.get::<_, Option<String>>(0))
        .unwrap()
        .filter_map(|s| s.ok().flatten())
        .collect();
    let joined = schema.join("\n").to_lowercase();
    assert!(
        !joined.contains("block_state") && !joined.contains(" state "),
        "BlockState must stay computed, never stored:\n{joined}"
    );
    // And the computation is reachable without touching storage at all.
    let b = Block {
        id: "x".into(),
        chapter_id: "c".into(),
        kind: BlockKind::Video,
        heading: None,
        caption_md: None,
        body_md: None,
        code_text: None,
        code_lang: None,
        asset_id: None,
        meta_json: "{}".into(),
        sort_index: 1024.0,
        created_at: 0,
        updated_at: 0,
        deleted_at: None,
    };
    assert_eq!(
        state::compute(&b, state::AssetStatus::None, false),
        BlockState::Empty
    );
}
