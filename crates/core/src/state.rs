//! `BlockState` is derived on every read. Persisting it is forbidden: it would
//! drift the instant a file is deleted outside the app.

use crate::models::{Block, BlockKind, BlockState};

/// What the storage layer knows about a block's asset at read time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetStatus {
    /// `asset_id IS NULL` — nothing has been attached yet.
    None,
    /// Asset row exists and its file is readable.
    Present,
    /// Asset row exists but the file is gone or unreadable.
    FileMissing,
}

/// Does this block's `meta_json` describe at least one quiz question?
pub fn quiz_has_questions(meta_json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(meta_json)
        .ok()
        .and_then(|v| {
            v.get("questions")
                .and_then(|q| q.as_array())
                .map(|a| !a.is_empty())
        })
        .unwrap_or(false)
}

fn blank(s: &Option<String>) -> bool {
    s.as_deref().map(|t| t.trim().is_empty()).unwrap_or(true)
}

/// The one place block state is decided.
pub fn compute(block: &Block, asset: AssetStatus, importing: bool) -> BlockState {
    if importing {
        return BlockState::Importing;
    }
    match block.kind {
        BlockKind::Video | BlockKind::Image | BlockKind::Attachment => match asset {
            AssetStatus::None => BlockState::Empty,
            AssetStatus::FileMissing => BlockState::Missing,
            AssetStatus::Present => BlockState::Ready,
        },
        BlockKind::Text | BlockKind::Callout => {
            if blank(&block.body_md) {
                BlockState::Empty
            } else {
                BlockState::Ready
            }
        }
        BlockKind::Code => {
            if blank(&block.code_text) {
                BlockState::Empty
            } else {
                BlockState::Ready
            }
        }
        BlockKind::Quiz => {
            if quiz_has_questions(&block.meta_json) {
                BlockState::Ready
            } else {
                BlockState::Empty
            }
        }
        BlockKind::Divider => BlockState::Ready,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(kind: BlockKind) -> Block {
        Block {
            id: "b".into(),
            chapter_id: "c".into(),
            kind,
            heading: None,
            caption_md: None,
            body_md: None,
            code_text: None,
            code_lang: Some("rust".into()),
            asset_id: None,
            meta_json: "{}".into(),
            sort_index: 1024.0,
            created_at: 0,
            updated_at: 0,
            deleted_at: None,
        }
    }

    const ALL: [BlockKind; 8] = [
        BlockKind::Video,
        BlockKind::Text,
        BlockKind::Image,
        BlockKind::Code,
        BlockKind::Callout,
        BlockKind::Attachment,
        BlockKind::Quiz,
        BlockKind::Divider,
    ];

    #[test]
    fn every_kind_with_no_payload_is_empty_except_divider() {
        for kind in ALL {
            let expected = if kind == BlockKind::Divider {
                BlockState::Ready
            } else {
                BlockState::Empty
            };
            assert_eq!(
                compute(&block(kind), AssetStatus::None, false),
                expected,
                "kind {kind:?} with no payload"
            );
        }
    }

    #[test]
    fn every_kind_with_payload_is_ready() {
        for kind in ALL {
            let mut b = block(kind);
            b.body_md = Some("some prose".into());
            b.code_text = Some("fn main() {}".into());
            b.meta_json = r#"{"questions":[{"q":"?"}]}"#.into();
            let asset = if kind.is_asset_backed() {
                b.asset_id = Some("a".into());
                AssetStatus::Present
            } else {
                AssetStatus::None
            };
            assert_eq!(
                compute(&b, asset, false),
                BlockState::Ready,
                "kind {kind:?}"
            );
        }
    }

    #[test]
    fn asset_backed_kinds_report_missing_when_the_file_is_gone() {
        for kind in [BlockKind::Video, BlockKind::Image, BlockKind::Attachment] {
            let mut b = block(kind);
            b.asset_id = Some("a".into());
            assert_eq!(
                compute(&b, AssetStatus::FileMissing, false),
                BlockState::Missing
            );
        }
    }

    #[test]
    fn importing_wins_over_everything() {
        for kind in ALL {
            assert_eq!(
                compute(&block(kind), AssetStatus::None, true),
                BlockState::Importing
            );
        }
    }

    #[test]
    fn whitespace_only_payload_counts_as_empty() {
        let mut b = block(BlockKind::Text);
        b.body_md = Some("   \n\t ".into());
        assert_eq!(compute(&b, AssetStatus::None, false), BlockState::Empty);

        let mut c = block(BlockKind::Code);
        c.code_text = Some("\n".into());
        assert_eq!(compute(&c, AssetStatus::None, false), BlockState::Empty);
    }

    #[test]
    fn quiz_needs_a_non_empty_question_array() {
        let mut b = block(BlockKind::Quiz);
        for (meta, ready) in [
            ("{}", false),
            (r#"{"questions":[]}"#, false),
            (r#"{"questions":null}"#, false),
            ("not json", false),
            (r#"{"questions":[{"prompt":"Why?"}]}"#, true),
        ] {
            b.meta_json = meta.into();
            let want = if ready {
                BlockState::Ready
            } else {
                BlockState::Empty
            };
            assert_eq!(compute(&b, AssetStatus::None, false), want, "meta {meta}");
        }
    }
}
