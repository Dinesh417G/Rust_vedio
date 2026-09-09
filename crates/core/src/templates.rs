//! Chapter scaffolds: plan a whole course in an afternoon, record later.

use crate::models::{BlockKind, BlockSeed};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ChapterTemplate {
    /// Nothing but the chapter itself.
    #[default]
    Blank,
    /// text intro -> video -> code -> text summary
    Lesson,
    /// text -> image -> callout
    Concept,
    /// text -> code -> quiz
    Practice,
}

impl ChapterTemplate {
    pub fn label(self) -> &'static str {
        match self {
            ChapterTemplate::Blank => "Blank",
            ChapterTemplate::Lesson => "Lesson",
            ChapterTemplate::Concept => "Concept",
            ChapterTemplate::Practice => "Practice",
        }
    }

    /// The blocks this template creates. Every one is deliberately *empty* —
    /// only headings and captions are filled in, so the page is plannable
    /// before a single file exists.
    pub fn blocks(self) -> Vec<(BlockKind, BlockSeed)> {
        fn seed(heading: &str, caption: &str) -> BlockSeed {
            BlockSeed {
                heading: Some(heading.to_string()),
                caption_md: Some(caption.to_string()),
                ..Default::default()
            }
        }
        match self {
            ChapterTemplate::Blank => vec![],
            ChapterTemplate::Lesson => vec![
                (
                    BlockKind::Text,
                    seed(
                        "Introduction",
                        "What this lesson covers and why it matters.",
                    ),
                ),
                (
                    BlockKind::Video,
                    seed("Walkthrough", "The recorded walkthrough for this lesson."),
                ),
                (
                    BlockKind::Code,
                    seed("Example", "The code written during the walkthrough."),
                ),
                (
                    BlockKind::Text,
                    seed("Summary", "The three things to remember from this lesson."),
                ),
            ],
            ChapterTemplate::Concept => vec![
                (
                    BlockKind::Text,
                    seed("The idea", "The concept stated in plain language."),
                ),
                (
                    BlockKind::Image,
                    seed("Diagram", "A picture of how the pieces fit together."),
                ),
                (
                    BlockKind::Callout,
                    seed("Watch out", "The mistake people make with this concept."),
                ),
            ],
            ChapterTemplate::Practice => vec![
                (
                    BlockKind::Text,
                    seed(
                        "The exercise",
                        "What to build and the rules for building it.",
                    ),
                ),
                (
                    BlockKind::Code,
                    seed("Starter code", "The scaffold to begin from."),
                ),
                (
                    BlockKind::Quiz,
                    seed(
                        "Check yourself",
                        "A few questions to confirm the idea landed.",
                    ),
                ),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lesson_scaffold_matches_the_spec() {
        let kinds: Vec<_> = ChapterTemplate::Lesson
            .blocks()
            .iter()
            .map(|(k, _)| *k)
            .collect();
        assert_eq!(
            kinds,
            vec![
                BlockKind::Text,
                BlockKind::Video,
                BlockKind::Code,
                BlockKind::Text
            ]
        );
    }

    #[test]
    fn scaffolded_blocks_carry_no_payload() {
        for t in [
            ChapterTemplate::Lesson,
            ChapterTemplate::Concept,
            ChapterTemplate::Practice,
        ] {
            for (_, seed) in t.blocks() {
                assert!(seed.heading.is_some());
                assert!(seed.caption_md.is_some());
                assert!(seed.body_md.is_none(), "{t:?} pre-filled a body");
                assert!(seed.code_text.is_none(), "{t:?} pre-filled code");
            }
        }
    }
}
