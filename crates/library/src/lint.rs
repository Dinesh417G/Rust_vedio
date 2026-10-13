//! Page-structure lint: does every chapter follow `library/STYLE.md`,
//! "Topic page — fixed block order"?
//!
//! The page type is not guessed from the content: it comes from the type
//! column of the curriculum row with the same display number, so a Video page
//! that was written as a text page is a violation, not a different page type.
//! Own pages (part 0) are free form and are not linted.

use crate::coverage::Expected;
use crate::format::{LibBlock, LibChapter, Library};
use rustclass_core::models::BlockKind;
use std::collections::HashMap;
use std::fmt;

/// One violation. Rendered as `display title: problem`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintIssue {
    /// `(no number)` when the chapter has none.
    pub display: String,
    pub title: String,
    pub file: String,
    /// The chapter's `@@chapter` line, or the block's `@@` line when the
    /// problem is about one block.
    pub line: usize,
    pub problem: String,
}

impl fmt::Display for LintIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.display, self.title, self.problem)
    }
}

/// A block the style guide asks for.
#[derive(Clone, Copy)]
struct Want {
    kind: BlockKind,
    /// Callout tone (`note`, `tip`, ...), only for callouts.
    tone: Option<&'static str>,
    /// Code language, only for code blocks.
    lang: Option<&'static str>,
    /// `None` accepts any heading.
    heading: Option<&'static str>,
    /// The heading only has to start with `heading` (`Diagram:`, `Try it:`).
    prefix: bool,
}

impl Want {
    const fn new(kind: BlockKind, heading: &'static str) -> Self {
        Self {
            kind,
            tone: None,
            lang: None,
            heading: Some(heading),
            prefix: false,
        }
    }

    const fn callout(tone: &'static str, heading: &'static str) -> Self {
        Self {
            tone: Some(tone),
            ..Self::new(BlockKind::Callout, heading)
        }
    }

    const fn code(lang: &'static str, heading: &'static str, prefix: bool) -> Self {
        Self {
            lang: Some(lang),
            prefix,
            ..Self::new(BlockKind::Code, heading)
        }
    }

    const fn any(kind: BlockKind) -> Self {
        Self {
            heading: None,
            ..Self::new(kind, "")
        }
    }

    /// How the problem messages name the block: `callout(note) "Watch out"`.
    fn label(&self) -> String {
        let kind = self.kind.as_str();
        let qualifier = match (self.tone, self.lang) {
            (Some(t), _) => format!("({t})"),
            (_, Some(l)) => format!("({l})"),
            _ => String::new(),
        };
        match self.heading {
            Some(h) if self.prefix => format!("{kind}{qualifier} starting \"{h}\""),
            Some(h) => format!("{kind}{qualifier} \"{h}\""),
            None => format!("{kind}{qualifier}"),
        }
    }

    fn matches(&self, block: &LibBlock) -> bool {
        if block.kind != self.kind {
            return false;
        }
        if let Some(tone) = self.tone {
            if tone_of(block).as_deref() != Some(tone) {
                return false;
            }
        }
        if let Some(lang) = self.lang {
            if block.code_lang.as_deref() != Some(lang) {
                return false;
            }
        }
        match self.heading {
            None => true,
            Some(want) => {
                let got = normalise(block.heading.as_deref().unwrap_or(""));
                if self.prefix {
                    got.starts_with(want)
                } else {
                    got == want
                }
            }
        }
    }
}

const LESSON_VIDEO: Want = Want::new(BlockKind::Video, "Lesson video");
const PLANT_SITUATION: Want = Want::callout("note", "Plant situation");
const TEACHERS_NOTES: Want = Want::new(BlockKind::Text, "Teacher's notes");
const PLC_VS_RUST: Want = Want::new(BlockKind::Text, "PLC ↔ Rust");
const DIAGRAM: Want = Want::code("mermaid", "Diagram:", true);
const TRY_IT: Want = Want::code("rust", "Try it:", true);
const WATCH_OUT: Want = Want::callout("warning", "Watch out");
const EXERCISE_BRIEF: Want = Want::new(BlockKind::Text, "Exercise brief");
const HINTS: Want = Want::callout("tip", "Hints");
const SOLUTION: Want = Want::code("rust", "Solution", true);
const TEXT_PAGE_NOTE: Want = Want::callout("note", "On Ardan this is a text page");
const ATTACHMENT: Want = Want::any(BlockKind::Attachment);
const CHEAT_SHEET: Want = Want::new(BlockKind::Text, "Cheat sheet");
const MODULE_MAP: Want = Want::code("mermaid", "Module map", false);
const CHECK_YOURSELF: Want = Want::new(BlockKind::Quiz, "Check yourself");

/// Curly quotes in a heading are the same heading.
fn normalise(heading: &str) -> String {
    heading.trim().replace(['\u{2019}', '\u{2018}'], "'")
}

fn tone_of(block: &LibBlock) -> Option<String> {
    let meta: serde_json::Value = serde_json::from_str(block.meta_json.as_deref()?).ok()?;
    meta.get("tone")?.as_str().map(String::from)
}

fn question_count(block: &LibBlock) -> usize {
    block
        .meta_json
        .as_deref()
        .and_then(|m| serde_json::from_str::<serde_json::Value>(m).ok())
        .and_then(|m| m.get("questions").and_then(|q| q.as_array().map(Vec::len)))
        .unwrap_or(0)
}

fn word_count(text: Option<&str>) -> usize {
    text.map_or(0, |t| t.split_whitespace().count())
}

/// A markdown table: a header row followed by a `|---|---|` separator row.
fn has_table(text: Option<&str>) -> bool {
    text.is_some_and(|t| {
        t.lines().any(|l| {
            let l = l.trim();
            l.starts_with('|')
                && l.contains('-')
                && l.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
        })
    })
}

struct Linter<'a> {
    chapter: &'a LibChapter,
    display: String,
    issues: Vec<LintIssue>,
}

impl<'a> Linter<'a> {
    fn new(chapter: &'a LibChapter) -> Self {
        Self {
            chapter,
            display: chapter
                .display_number
                .clone()
                .unwrap_or_else(|| "(no number)".to_string()),
            issues: Vec::new(),
        }
    }

    fn report(&mut self, line: usize, problem: impl Into<String>) {
        self.issues.push(LintIssue {
            display: self.display.clone(),
            title: self.chapter.title.clone(),
            file: self.chapter.file.clone(),
            line,
            problem: problem.into(),
        });
    }

    fn find(&self, want: &Want) -> Option<(usize, &'a LibBlock)> {
        self.chapter
            .blocks
            .iter()
            .enumerate()
            .find(|(_, b)| want.matches(b))
    }

    /// The blocks must all be there and in this order; the first
    /// `fixed_prefix` of them must also be the first blocks of the page.
    fn ordered(&mut self, wants: &[Want], fixed_prefix: usize) {
        let mut found: Vec<(usize, usize)> = Vec::new(); // (index in wants, block index)
        for (i, want) in wants.iter().enumerate() {
            match self.find(want) {
                Some((at, _)) => found.push((i, at)),
                None => self.report(self.chapter.line, format!("missing {}", want.label())),
            }
        }
        let mut misordered = false;
        for pair in found.windows(2) {
            let ((a, at_a), (b, at_b)) = (pair[0], pair[1]);
            if at_b < at_a {
                misordered = true;
                self.report(
                    self.chapter.blocks[at_b].line,
                    format!(
                        "{} comes before {}, expected after it",
                        wants[b].label(),
                        wants[a].label()
                    ),
                );
            }
        }
        // A missing block shifts every later one, so positions only mean
        // something once all of them are there and in order.
        if misordered || found.len() < wants.len() {
            return;
        }
        if let Some(&(i, at)) = found.iter().find(|&&(i, at)| i < fixed_prefix && at != i) {
            self.report(
                self.chapter.blocks[at].line,
                format!(
                    "{} is block {}, expected block {}",
                    wants[i].label(),
                    at + 1,
                    i + 1
                ),
            );
        }
    }

    /// The page must end with a quiz; `want` narrows which quiz.
    fn ends_with_quiz(&mut self, want: &Want) -> Option<&'a LibBlock> {
        let Some(last) = self.chapter.blocks.last() else {
            self.report(self.chapter.line, format!("missing {}", want.label()));
            return None;
        };
        if want.matches(last) {
            return Some(last);
        }
        match self.find(want) {
            Some((_, quiz)) => self.report(
                quiz.line,
                format!("{} must be the last block", want.label()),
            ),
            None if want.heading.is_some() && last.kind == BlockKind::Quiz => self.report(
                last.line,
                format!(
                    "the last quiz is headed \"{}\", expected {}",
                    last.heading.as_deref().unwrap_or(""),
                    want.label()
                ),
            ),
            None => self.report(self.chapter.line, format!("missing {}", want.label())),
        }
        None
    }

    fn needs(&mut self, want: &Want) -> Option<&'a LibBlock> {
        let found = self.find(want).map(|(_, b)| b);
        if found.is_none() {
            self.report(self.chapter.line, format!("missing {}", want.label()));
        }
        found
    }

    fn first_is(&mut self, want: &Want) {
        match self.chapter.blocks.first() {
            None => self.report(self.chapter.line, format!("missing {}", want.label())),
            Some(first) if want.matches(first) => {}
            Some(first) => self.report(
                first.line,
                format!("the first block must be {}", want.label()),
            ),
        }
    }

    fn table_in(&mut self, want: &Want) {
        if let Some(block) = self.needs(want) {
            if !has_table(block.body_md.as_deref()) {
                self.report(block.line, format!("{} has no table", want.label()));
            }
        }
    }

    fn video_page(&mut self, exercise: bool) {
        if exercise {
            self.ordered(
                &[
                    LESSON_VIDEO,
                    PLANT_SITUATION,
                    EXERCISE_BRIEF,
                    HINTS,
                    SOLUTION,
                    WATCH_OUT,
                ],
                2,
            );
            self.ends_with_quiz(&CHECK_YOURSELF);
            return;
        }
        self.ordered(
            &[
                LESSON_VIDEO,
                PLANT_SITUATION,
                TEACHERS_NOTES,
                PLC_VS_RUST,
                DIAGRAM,
                TRY_IT,
                WATCH_OUT,
            ],
            7,
        );
        if let Some((_, notes)) = self.find(&TEACHERS_NOTES) {
            let words = word_count(notes.body_md.as_deref());
            if words < 200 {
                self.report(
                    notes.line,
                    format!(
                        "{} has {words} words, needs at least 200",
                        TEACHERS_NOTES.label()
                    ),
                );
            }
        }
        if let Some((_, table)) = self.find(&PLC_VS_RUST) {
            if !has_table(table.body_md.as_deref()) {
                self.report(table.line, format!("{} has no table", PLC_VS_RUST.label()));
            }
        }
        if let Some(quiz) = self.ends_with_quiz(&CHECK_YOURSELF) {
            let n = question_count(quiz);
            if !(2..=3).contains(&n) {
                self.report(
                    quiz.line,
                    format!("{} has {n} questions, needs 2 or 3", CHECK_YOURSELF.label()),
                );
            }
        }
    }

    fn text_page(&mut self) {
        self.first_is(&TEXT_PAGE_NOTE);
        self.needs(&TEACHERS_NOTES);
        self.ends_with_quiz(&Want::any(BlockKind::Quiz));
    }

    fn pdf_page(&mut self) {
        self.first_is(&ATTACHMENT);
        self.table_in(&CHEAT_SHEET);
        self.needs(&MODULE_MAP);
    }

    fn quiz_page(&mut self, final_exam: bool) {
        let quizzes: Vec<&LibBlock> = self
            .chapter
            .blocks
            .iter()
            .filter(|b| b.kind == BlockKind::Quiz)
            .collect();
        if quizzes.is_empty() {
            self.report(self.chapter.line, "missing a quiz block");
            return;
        }
        let total: usize = quizzes.iter().map(|q| question_count(q)).sum();
        let need = if final_exam { 15 } else { 8 };
        if total < need {
            self.report(
                quizzes[0].line,
                format!(
                    "{} has {total} quiz questions, needs at least {need}",
                    if final_exam {
                        "the final exam"
                    } else {
                        "the page"
                    }
                ),
            );
        }
    }

    /// Every Video page names its length in the caption.
    fn video_length(&mut self, length: &str) {
        let Some(video) = self
            .chapter
            .blocks
            .iter()
            .find(|b| b.kind == BlockKind::Video)
        else {
            return; // already reported as a missing block
        };
        if !video.caption_md.as_deref().unwrap_or("").contains(length) {
            self.report(
                video.line,
                format!("the video caption does not contain the length {length}"),
            );
        }
    }
}

/// Checks every chapter of `lib` against the topic-page layout of its
/// curriculum type. Issues come back in library order.
pub fn lint(expected: &[Expected], lib: &Library) -> Vec<LintIssue> {
    let by_display: HashMap<&str, &Expected> =
        expected.iter().map(|e| (e.display.as_str(), e)).collect();
    let mut issues = Vec::new();
    for chapter in &lib.chapters {
        let mut linter = Linter::new(chapter);
        let row = chapter
            .display_number
            .as_deref()
            .and_then(|d| by_display.get(d));
        match row {
            None => linter.report(
                chapter.line,
                "not in the curriculum, so its page type is unknown",
            ),
            Some(row) => match row.kind.as_str() {
                "Video" => {
                    linter.video_page(chapter.title.contains("Exercise"));
                    if let Some(length) = &row.video {
                        linter.video_length(length);
                    }
                }
                "Text" => linter.text_page(),
                "PDF" => linter.pdf_page(),
                "Quiz" => linter.quiz_page(chapter.title.contains("Final Exam")),
                // Own pages are free form.
                _ => {}
            },
        }
        issues.extend(linter.issues);
    }
    issues
}
