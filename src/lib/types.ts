/**
 * Mirrors `crates/core/src/models.rs`. The Rust side serialises camelCase, so
 * these names match field-for-field. Keep the two in step by hand — that is
 * the cost of a TypeScript frontend, and it is paid here in one file.
 */

export type BlockKind =
  | "video"
  | "text"
  | "image"
  | "code"
  | "callout"
  | "attachment"
  | "quiz"
  | "divider";

export type BlockState = "empty" | "ready" | "missing" | "importing";
export type AssetKind = "video" | "image" | "file";
export type ImportMode = "copy" | "reference";
export type ChapterStatus = "draft" | "published";
export type ChapterTemplate = "blank" | "lesson" | "concept" | "practice";
export type OutlineMode = "append" | "replace";
export type BundleMode = "merge" | "replace";

export interface CourseView {
  id: string;
  title: string;
  subtitle: string | null;
  author: string | null;
  descriptionMd: string | null;
  coverAssetId: string | null;
  createdAt: number;
  updatedAt: number;
}

export interface CoursePatch {
  title?: string;
  subtitle?: string | null;
  author?: string | null;
  descriptionMd?: string | null;
  coverAssetId?: string | null;
}

export interface Chapter {
  id: string;
  courseId: string;
  displayNumber: string | null;
  title: string;
  summaryMd: string | null;
  objectivesMd: string | null;
  estimatedMinutes: number | null;
  status: ChapterStatus;
  sortIndex: number;
  createdAt: number;
  updatedAt: number;
  deletedAt: number | null;
}

export interface ChapterSummary extends Chapter {
  blockCount: number;
  readyCount: number;
  emptyCount: number;
  missingCount: number;
  completedCount: number;
}

export interface ChapterView extends Chapter {
  blocks: BlockView[];
}

export interface NewChapter {
  title: string;
  displayNumber?: string | null;
  summaryMd?: string | null;
  objectivesMd?: string | null;
  estimatedMinutes?: number | null;
  template?: ChapterTemplate | null;
  afterId?: string | null;
}

export interface ChapterPatch {
  title?: string;
  displayNumber?: string | null;
  summaryMd?: string | null;
  objectivesMd?: string | null;
  estimatedMinutes?: number | null;
  status?: ChapterStatus;
}

export interface Block {
  id: string;
  chapterId: string;
  kind: BlockKind;
  heading: string | null;
  captionMd: string | null;
  bodyMd: string | null;
  codeText: string | null;
  codeLang: string | null;
  assetId: string | null;
  metaJson: string;
  sortIndex: number;
  createdAt: number;
  updatedAt: number;
  deletedAt: number | null;
}

export interface BlockView extends Block {
  state: BlockState;
  asset: AssetView | null;
  progress: ProgressView | null;
  markers: Marker[];
}

export interface BlockPatch {
  heading?: string | null;
  captionMd?: string | null;
  bodyMd?: string | null;
  codeText?: string | null;
  codeLang?: string | null;
  metaJson?: string;
}

export interface BlockSeed {
  heading?: string | null;
  captionMd?: string | null;
  bodyMd?: string | null;
  codeText?: string | null;
  codeLang?: string | null;
  metaJson?: string | null;
}

export interface AssetView {
  id: string;
  kind: AssetKind;
  originalName: string;
  relPath: string;
  absPath: string | null;
  importMode: ImportMode;
  mime: string | null;
  bytes: number;
  sha256: string;
  durationSecs: number | null;
  width: number | null;
  height: number | null;
  thumbRelPath: string | null;
  importedAt: number;
  fileExists: boolean;
  resolvedPath: string;
  thumbPath: string | null;
}

export interface ProgressView {
  blockId: string;
  completed: boolean;
  lastPositionSecs: number;
  watchedSecs: number;
  updatedAt: number;
}

export interface ProgressPatch {
  completed?: boolean;
  lastPositionSecs?: number;
  watchedSecs?: number;
}

export interface Marker {
  id: string;
  blockId: string;
  atSecs: number;
  label: string;
  noteMd: string | null;
}

export interface Note {
  id: string;
  chapterId: string;
  bodyMd: string;
  createdAt: number;
  updatedAt: number;
}

export interface ChapterProgress {
  chapterId: string;
  title: string;
  displayNumber: string | null;
  totalBlocks: number;
  completedBlocks: number;
  percent: number;
}

export interface CourseProgress {
  chapters: ChapterProgress[];
  totalBlocks: number;
  completedBlocks: number;
  percent: number;
  lastChapterId: string | null;
  lastBlockId: string | null;
}

export interface SearchHit {
  entityId: string;
  entityType: "chapter" | "block";
  chapterId: string;
  chapterTitle: string;
  chapterNumber: string | null;
  title: string;
  snippet: string;
}

export interface IntegrityIssue {
  kind: string;
  severity: "error" | "warning";
  entityId: string;
  entityType: string;
  message: string;
  path: string | null;
}

export interface GcEntry {
  id: string;
  label: string;
  path: string;
  bytes: number;
}

export interface GcReport {
  dryRun: boolean;
  orphanFiles: GcEntry[];
  unreferencedAssets: GcEntry[];
  partialFiles: GcEntry[];
  bytesReclaimable: number;
}

export interface SystemInfo {
  appVersion: string;
  schemaVersion: number;
  bundleSchemaVersion: number;
  appDataDir: string;
  dbPath: string;
  mediaDir: string;
  logDir: string;
  dbBytes: number;
  mediaBytes: number;
  assetBytesTracked: number;
  chapterCount: number;
  blockCount: number;
  assetCount: number;
  trashedChapters: number;
  trashedBlocks: number;
}

export interface ParsedBlock {
  kind: BlockKind;
  heading: string;
  captionMd: string | null;
}

export interface ParsedChapter {
  displayNumber: string | null;
  title: string;
  summaryMd: string | null;
  blocks: ParsedBlock[];
}

export interface ParsedOutline {
  courseTitle: string | null;
  chapters: ParsedChapter[];
  warnings: string[];
}

export interface OutlineImportReport {
  chaptersCreated: number;
  blocksCreated: number;
  chaptersRemoved: number;
  courseTitleChanged: boolean;
  warnings: string[];
}

export interface ExportReport {
  path: string;
  bytes: number;
  chapters: number;
  blocks: number;
  assets: number;
  mediaFiles: number;
  mediaSkipped: string[];
}

export interface BundleImportReport {
  chapters: number;
  blocks: number;
  assets: number;
  mediaFiles: number;
  replaced: boolean;
  warnings: string[];
}

export interface TrashedBlock extends Block {
  chapterTitle: string;
}

export interface TrashView {
  chapters: ChapterSummary[];
  blocks: TrashedBlock[];
  retentionDays: number;
}

export interface ImportRequest {
  path: string;
  blockId?: string | null;
  kind?: AssetKind | null;
  mode: ImportMode;
}

export interface ImportTicket {
  ticketId: string;
  blockId: string | null;
  originalName: string;
  bytesTotal: number;
  dedupedAssetId: string | null;
}

export interface ImportProgress {
  ticketId: string;
  blockId: string | null;
  assetId: string | null;
  bytesDone: number;
  bytesTotal: number;
  pct: number;
}

export interface ImportDone {
  ticketId: string;
  blockId: string | null;
  assetId: string;
  deduped: boolean;
}

export interface ImportFailed {
  ticketId: string;
  blockId: string | null;
  code: string;
  message: string;
}

export interface DataChanged {
  entity: "course" | "chapter" | "block" | "asset" | "progress" | "marker" | "note";
  id: string | null;
}

/** The `{ code, message, detail }` shape every command failure serialises to. */
export interface AppError {
  code: string;
  message: string;
  detail: string | null;
}

/** Quiz payload, stored inside a quiz block's `metaJson`. */
export interface QuizQuestion {
  id: string;
  prompt: string;
  choices: string[];
  answerIndex: number;
  explanation?: string;
}

export interface QuizMeta {
  questions: QuizQuestion[];
}

/** Callout payload, stored inside a callout block's `metaJson`. */
export type CalloutTone = "note" | "tip" | "warning" | "danger";
export interface CalloutMeta {
  tone?: CalloutTone;
}

/** Image payload, stored inside an image block's `metaJson`. */
export interface ImageMeta {
  fit?: "contain" | "cover";
}

export const BLOCK_KINDS: BlockKind[] = [
  "text",
  "video",
  "image",
  "code",
  "callout",
  "attachment",
  "quiz",
  "divider",
];

export const ASSET_BACKED: BlockKind[] = ["video", "image", "attachment"];
