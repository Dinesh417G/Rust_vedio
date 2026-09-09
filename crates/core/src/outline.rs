//! Markdown outline -> course structure.
//!
//! ```text
//! # Rust from the ground up          -> course title
//! ## Ownership & Borrowing           -> chapter
//! - [video] Move semantics           -> empty video block
//! - [text] Why the borrow checker    -> empty text block
//! - Untagged bullets                 -> empty text block
//! ```

use crate::models::{BlockKind, BlockSeed};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutlineMode {
    /// Add the parsed chapters after whatever already exists.
    Append,
    /// Soft-delete every existing chapter first.
    Replace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedBlock {
    pub kind: BlockKind,
    pub heading: String,
    pub caption_md: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedChapter {
    pub display_number: Option<String>,
    pub title: String,
    pub summary_md: Option<String>,
    pub blocks: Vec<ParsedBlock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedOutline {
    pub course_title: Option<String>,
    pub chapters: Vec<ParsedChapter>,
    /// Lines the parser could not place, reported in the preview.
    pub warnings: Vec<String>,
}

impl ParsedOutline {
    pub fn block_count(&self) -> usize {
        self.chapters.iter().map(|c| c.blocks.len()).sum()
    }

    /// Counts per kind, for the preview diff.
    pub fn counts_by_kind(&self) -> Vec<(BlockKind, usize)> {
        let order = [
            BlockKind::Video,
            BlockKind::Text,
            BlockKind::Image,
            BlockKind::Code,
            BlockKind::Callout,
            BlockKind::Attachment,
            BlockKind::Quiz,
            BlockKind::Divider,
        ];
        order
            .into_iter()
            .filter_map(|k| {
                let n = self
                    .chapters
                    .iter()
                    .flat_map(|c| &c.blocks)
                    .filter(|b| b.kind == k)
                    .count();
                (n > 0).then_some((k, n))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportReport {
    pub chapters_created: u32,
    pub blocks_created: u32,
    pub chapters_removed: u32,
    pub course_title_changed: bool,
    pub warnings: Vec<String>,
}

/// Drops the punctuation that separates a number from the title it labels.
/// Em and en dashes count: `Appendix A — Cheatsheet` is titled `Cheatsheet`.
fn strip_separator(s: &str) -> String {
    s.trim_matches(|c: char| {
        c.is_whitespace() || matches!(c, '.' | ')' | '-' | ':' | '\u{2013}' | '\u{2014}')
    })
    .to_string()
}

/// Splits `4.2 Ownership` into `("4.2", "Ownership")`. A heading with no
/// leading number keeps its whole text as the title.
fn split_number(text: &str) -> (Option<String>, String) {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("Appendix ") {
        if let Some((letter, title)) = rest.split_once([' ', '-', ':']) {
            if !letter.is_empty() {
                return (
                    Some(format!("Appendix {}", letter.trim_end_matches('.'))),
                    strip_separator(title),
                );
            }
        }
    }
    let head: String = trimmed
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let head = head.trim_end_matches('.');
    if head.is_empty() || !head.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return (None, trimmed.to_string());
    }
    let rest = strip_separator(&trimmed[head.len()..]);
    let rest = rest.as_str();
    if rest.is_empty() {
        return (None, trimmed.to_string());
    }
    (Some(head.to_string()), rest.to_string())
}

/// Pulls a leading `[kind]` tag off a bullet. Untagged bullets become text.
fn split_tag(text: &str) -> (BlockKind, String) {
    let t = text.trim();
    if let Some(close) = t.find(']') {
        if t.starts_with('[') {
            let tag = t[1..close].trim().to_ascii_lowercase();
            if let Some(kind) = BlockKind::parse(&tag) {
                return (kind, t[close + 1..].trim().to_string());
            }
        }
    }
    (BlockKind::Text, t.to_string())
}

fn bullet_body(line: &str) -> Option<&str> {
    let t = line.trim_start();
    for p in ["- ", "* ", "+ "] {
        if let Some(rest) = t.strip_prefix(p) {
            return Some(rest);
        }
    }
    // `1. Something` ordered bullets
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() {
        if let Some(rest) = t[digits.len()..].strip_prefix(". ") {
            return Some(rest);
        }
    }
    None
}

pub fn parse(md: &str) -> ParsedOutline {
    let mut out = ParsedOutline {
        course_title: None,
        chapters: Vec::new(),
        warnings: Vec::new(),
    };
    let mut in_fence = false;

    for (lineno, raw) in md.lines().enumerate() {
        let line = raw.trim_end();
        let trimmed = line.trim();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence || trimmed.is_empty() {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("## ") {
            let (number, title) = split_number(rest);
            if title.is_empty() {
                out.warnings
                    .push(format!("line {}: chapter heading has no title", lineno + 1));
                continue;
            }
            out.chapters.push(ParsedChapter {
                display_number: number,
                title,
                summary_md: None,
                blocks: Vec::new(),
            });
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("# ") {
            if out.course_title.is_none() {
                out.course_title = Some(rest.trim().to_string());
            } else {
                out.warnings.push(format!(
                    "line {}: ignored a second course title `{}`",
                    lineno + 1,
                    rest.trim()
                ));
            }
            continue;
        }

        // Deeper headings are treated as chapters too rather than dropped.
        if let Some(rest) = trimmed.strip_prefix("### ") {
            let (number, title) = split_number(rest);
            out.chapters.push(ParsedChapter {
                display_number: number,
                title,
                summary_md: None,
                blocks: Vec::new(),
            });
            continue;
        }

        if let Some(body) = bullet_body(line) {
            let Some(chapter) = out.chapters.last_mut() else {
                out.warnings.push(format!(
                    "line {}: `{}` appears before any `## chapter` heading and was skipped",
                    lineno + 1,
                    body.trim()
                ));
                continue;
            };
            let (kind, mut heading) = split_tag(body);
            let caption = match heading
                .split_once(" — ")
                .or_else(|| heading.split_once(" -- "))
            {
                Some((h, c)) => {
                    let c = c.trim().to_string();
                    heading = h.trim().to_string();
                    Some(c)
                }
                None => None,
            };
            if heading.is_empty() {
                out.warnings
                    .push(format!("line {}: bullet has no text", lineno + 1));
                continue;
            }
            chapter.blocks.push(ParsedBlock {
                kind,
                heading,
                caption_md: caption,
            });
            continue;
        }

        // Free prose directly under a chapter heading becomes its summary.
        if let Some(chapter) = out.chapters.last_mut() {
            if chapter.blocks.is_empty() {
                match &mut chapter.summary_md {
                    Some(existing) => {
                        existing.push('\n');
                        existing.push_str(trimmed);
                    }
                    None => chapter.summary_md = Some(trimmed.to_string()),
                }
                continue;
            }
        }
        out.warnings.push(format!(
            "line {}: `{}` was not recognised as a heading or bullet",
            lineno + 1,
            trimmed
        ));
    }

    out
}

/// Turns a parsed bullet into the seed `block_create` wants.
pub fn seed_for(b: &ParsedBlock) -> BlockSeed {
    BlockSeed {
        heading: Some(b.heading.clone()),
        caption_md: b.caption_md.clone(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
# Rust from the ground up

## 1. Getting set up
Everything you need before writing a line of Rust.
- [text] Installing the toolchain
- [code] Hello, world

## 4. Ownership & Borrowing
- [video] Move semantics — This video covers moves, clones and the borrow checker
- [text] Why the borrow checker exists
- [image] Stack vs heap diagram
- [quiz] Check yourself
- An untagged bullet
"#;

    #[test]
    fn parses_the_documented_shape() {
        let o = parse(SAMPLE);
        assert_eq!(o.course_title.as_deref(), Some("Rust from the ground up"));
        assert_eq!(o.chapters.len(), 2);

        let c0 = &o.chapters[0];
        assert_eq!(c0.display_number.as_deref(), Some("1"));
        assert_eq!(c0.title, "Getting set up");
        assert_eq!(
            c0.summary_md.as_deref(),
            Some("Everything you need before writing a line of Rust.")
        );
        assert_eq!(c0.blocks.len(), 2);

        let c1 = &o.chapters[1];
        assert_eq!(c1.display_number.as_deref(), Some("4"));
        assert_eq!(c1.title, "Ownership & Borrowing");
        assert_eq!(c1.blocks[0].kind, BlockKind::Video);
        assert_eq!(c1.blocks[0].heading, "Move semantics");
        assert_eq!(
            c1.blocks[0].caption_md.as_deref(),
            Some("This video covers moves, clones and the borrow checker")
        );
        assert_eq!(
            c1.blocks[4].kind,
            BlockKind::Text,
            "untagged defaults to text"
        );
        assert_eq!(c1.blocks[4].heading, "An untagged bullet");
        assert!(
            o.warnings.is_empty(),
            "unexpected warnings: {:?}",
            o.warnings
        );
    }

    #[test]
    fn counts_are_reported_for_the_preview() {
        let o = parse(SAMPLE);
        assert_eq!(o.block_count(), 7);
        let counts = o.counts_by_kind();
        assert!(counts.contains(&(BlockKind::Video, 1)));
        assert!(counts.contains(&(BlockKind::Text, 3)));
        assert!(counts.contains(&(BlockKind::Quiz, 1)));
    }

    #[test]
    fn ten_chapter_outline_creates_ten_chapters() {
        let mut md = String::from("# Course\n");
        for i in 1..=10 {
            md.push_str(&format!(
                "## {i}. Chapter {i}\n- [video] A\n- [text] B\n- [code] C\n"
            ));
        }
        let o = parse(&md);
        assert_eq!(o.chapters.len(), 10);
        assert_eq!(o.block_count(), 30);
        assert!(o.warnings.is_empty());
    }

    #[test]
    fn bullets_before_any_chapter_warn_instead_of_vanishing() {
        let o = parse("- [video] Orphan bullet");
        assert!(o.chapters.is_empty());
        assert_eq!(o.warnings.len(), 1);
        assert!(o.warnings[0].contains("Orphan bullet"));
    }

    #[test]
    fn unknown_tags_fall_back_to_text_and_keep_the_tag_text() {
        let o = parse("## C\n- [podcast] Interview with Niko");
        assert_eq!(o.chapters[0].blocks[0].kind, BlockKind::Text);
        assert_eq!(
            o.chapters[0].blocks[0].heading,
            "[podcast] Interview with Niko"
        );
    }

    #[test]
    fn code_fences_are_not_parsed_as_structure() {
        let o = parse("## C\n```rust\n// # not a title\n- not a bullet\n```\n- [text] Real bullet");
        assert_eq!(o.chapters.len(), 1);
        assert_eq!(o.chapters[0].blocks.len(), 1);
        assert_eq!(o.chapters[0].blocks[0].heading, "Real bullet");
    }

    #[test]
    fn appendix_and_dotted_numbers_survive() {
        assert_eq!(
            split_number("4.2 Interior mutability"),
            (Some("4.2".into()), "Interior mutability".into())
        );
        assert_eq!(
            split_number("Appendix A — Cheatsheet"),
            (Some("Appendix A".into()), "Cheatsheet".into())
        );
        assert_eq!(split_number("Ownership"), (None, "Ownership".into()));
    }

    #[test]
    fn ordered_bullets_are_accepted() {
        let o = parse("## C\n1. [video] First\n2. Second");
        assert_eq!(o.chapters[0].blocks.len(), 2);
        assert_eq!(o.chapters[0].blocks[0].kind, BlockKind::Video);
    }
}
