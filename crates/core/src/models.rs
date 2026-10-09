use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enumerations
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockKind {
    Video,
    Text,
    Image,
    Code,
    Callout,
    Attachment,
    Quiz,
    Divider,
}

impl BlockKind {
    pub fn as_str(self) -> &'static str {
        match self {
            BlockKind::Video => "video",
            BlockKind::Text => "text",
            BlockKind::Image => "image",
            BlockKind::Code => "code",
            BlockKind::Callout => "callout",
            BlockKind::Attachment => "attachment",
            BlockKind::Quiz => "quiz",
            BlockKind::Divider => "divider",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "video" => BlockKind::Video,
            "text" => BlockKind::Text,
            "image" => BlockKind::Image,
            "code" => BlockKind::Code,
            "callout" => BlockKind::Callout,
            "attachment" => BlockKind::Attachment,
            "quiz" => BlockKind::Quiz,
            "divider" => BlockKind::Divider,
            _ => return None,
        })
    }

    /// Kinds whose payload is a file on disk.
    pub fn is_asset_backed(self) -> bool {
        matches!(
            self,
            BlockKind::Video | BlockKind::Image | BlockKind::Attachment
        )
    }

    /// Which asset kind this block accepts, if any.
    pub fn asset_kind(self) -> Option<AssetKind> {
        match self {
            BlockKind::Video => Some(AssetKind::Video),
            BlockKind::Image => Some(AssetKind::Image),
            BlockKind::Attachment => Some(AssetKind::File),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetKind {
    Video,
    Image,
    File,
}

impl AssetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AssetKind::Video => "video",
            AssetKind::Image => "image",
            AssetKind::File => "file",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "video" => AssetKind::Video,
            "image" => AssetKind::Image,
            "file" => AssetKind::File,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImportMode {
    #[default]
    Copy,
    Reference,
}

impl ImportMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ImportMode::Copy => "copy",
            ImportMode::Reference => "reference",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "copy" => ImportMode::Copy,
            "reference" => ImportMode::Reference,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ChapterStatus {
    #[default]
    Draft,
    Published,
}

impl ChapterStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ChapterStatus::Draft => "draft",
            ChapterStatus::Published => "published",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "published" => ChapterStatus::Published,
            _ => ChapterStatus::Draft,
        }
    }
}

/// Computed, never stored. See `state::compute`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockState {
    /// Block declared, required payload absent — the placeholder case.
    Empty,
    /// Payload present and, if asset-backed, the file exists on disk.
    Ready,
    /// Asset row exists but the file is gone or unreadable.
    Missing,
    /// An import job for this block is in flight.
    Importing,
}

// ---------------------------------------------------------------------------
// Rows
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseView {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub description_md: Option<String>,
    pub cover_asset_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoursePatch {
    pub title: Option<String>,
    #[serde(default, deserialize_with = "double_option")]
    pub subtitle: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub author: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub description_md: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub cover_asset_id: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub id: String,
    pub course_id: String,
    pub display_number: Option<String>,
    pub title: String,
    pub summary_md: Option<String>,
    pub objectives_md: Option<String>,
    pub estimated_minutes: Option<i64>,
    pub status: ChapterStatus,
    pub sort_index: f64,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
    /// Top grouping level, e.g. one course inside a bundle.
    pub part: Option<String>,
    /// Second grouping level inside a part, e.g. a module.
    pub section: Option<String>,
}

/// Chapter plus the block-state histogram the sidebar needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterSummary {
    #[serde(flatten)]
    pub chapter: Chapter,
    pub block_count: u32,
    pub ready_count: u32,
    pub empty_count: u32,
    pub missing_count: u32,
    pub completed_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterView {
    #[serde(flatten)]
    pub chapter: Chapter,
    pub blocks: Vec<BlockView>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChapter {
    pub title: String,
    #[serde(default)]
    pub display_number: Option<String>,
    #[serde(default)]
    pub summary_md: Option<String>,
    #[serde(default)]
    pub objectives_md: Option<String>,
    #[serde(default)]
    pub estimated_minutes: Option<i64>,
    /// Optional scaffold that pre-creates a set of empty blocks.
    #[serde(default)]
    pub template: Option<crate::templates::ChapterTemplate>,
    /// Insert directly after this chapter; `None` appends.
    #[serde(default)]
    pub after_id: Option<String>,
    #[serde(default)]
    pub part: Option<String>,
    #[serde(default)]
    pub section: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterPatch {
    pub title: Option<String>,
    #[serde(default, deserialize_with = "double_option")]
    pub display_number: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub summary_md: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub objectives_md: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub estimated_minutes: Option<Option<i64>>,
    pub status: Option<ChapterStatus>,
    #[serde(default, deserialize_with = "double_option")]
    pub part: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub section: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub chapter_id: String,
    pub kind: BlockKind,
    pub heading: Option<String>,
    pub caption_md: Option<String>,
    pub body_md: Option<String>,
    pub code_text: Option<String>,
    pub code_lang: Option<String>,
    pub asset_id: Option<String>,
    pub meta_json: String,
    pub sort_index: f64,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockView {
    #[serde(flatten)]
    pub block: Block,
    /// Computed on every read — never persisted.
    pub state: BlockState,
    pub asset: Option<AssetView>,
    pub progress: Option<ProgressView>,
    pub markers: Vec<Marker>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPatch {
    #[serde(default, deserialize_with = "double_option")]
    pub heading: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub caption_md: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub body_md: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub code_text: Option<Option<String>>,
    #[serde(default, deserialize_with = "double_option")]
    pub code_lang: Option<Option<String>>,
    pub meta_json: Option<String>,
}

/// Initial content handed to `block_create`, used by templates and outline import.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockSeed {
    #[serde(default)]
    pub heading: Option<String>,
    #[serde(default)]
    pub caption_md: Option<String>,
    #[serde(default)]
    pub body_md: Option<String>,
    #[serde(default)]
    pub code_text: Option<String>,
    #[serde(default)]
    pub code_lang: Option<String>,
    #[serde(default)]
    pub meta_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetView {
    pub id: String,
    pub kind: AssetKind,
    pub original_name: String,
    pub rel_path: String,
    pub abs_path: Option<String>,
    pub import_mode: ImportMode,
    pub mime: Option<String>,
    pub bytes: i64,
    pub sha256: String,
    pub duration_secs: Option<f64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumb_rel_path: Option<String>,
    pub imported_at: i64,
    /// Resolved at read time: does the file actually exist right now?
    pub file_exists: bool,
    /// Absolute path the webview should convert with `convertFileSrc`.
    pub resolved_path: String,
    pub thumb_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressView {
    pub block_id: String,
    pub completed: bool,
    pub last_position_secs: f64,
    pub watched_secs: f64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressPatch {
    pub completed: Option<bool>,
    pub last_position_secs: Option<f64>,
    pub watched_secs: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub id: String,
    pub block_id: String,
    pub at_secs: f64,
    pub label: String,
    pub note_md: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub chapter_id: String,
    pub body_md: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterProgress {
    pub chapter_id: String,
    pub title: String,
    pub display_number: Option<String>,
    pub total_blocks: u32,
    pub completed_blocks: u32,
    pub percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseProgress {
    pub chapters: Vec<ChapterProgress>,
    pub total_blocks: u32,
    pub completed_blocks: u32,
    pub percent: f64,
    /// Where "continue where you left off" should jump to.
    pub last_chapter_id: Option<String>,
    pub last_block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub entity_id: String,
    pub entity_type: String,
    pub chapter_id: String,
    pub chapter_title: String,
    pub chapter_number: Option<String>,
    pub title: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityIssue {
    pub kind: String,
    pub severity: String,
    pub entity_id: String,
    pub entity_type: String,
    pub message: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GcReport {
    pub dry_run: bool,
    pub orphan_files: Vec<GcEntry>,
    pub unreferenced_assets: Vec<GcEntry>,
    pub partial_files: Vec<GcEntry>,
    pub bytes_reclaimable: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcEntry {
    pub id: String,
    pub label: String,
    pub path: String,
    pub bytes: i64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Distinguishes "field absent from the patch" (`None`) from "field explicitly
/// set to null" (`Some(None)`), which is how a caption gets cleared.
fn double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Deserialize::deserialize(de).map(Some)
}
