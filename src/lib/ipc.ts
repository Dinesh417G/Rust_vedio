/**
 * One typed wrapper per Tauri command. Nothing else in the app calls `invoke`
 * directly, so the whole IPC surface is visible in this file and a rename on
 * the Rust side breaks the build here rather than at runtime.
 */
import { invoke } from "@tauri-apps/api/core";
import type * as T from "./types";

/** Narrows an unknown throw into the `{ code, message, detail }` shape. */
export function asAppError(e: unknown): T.AppError {
  if (e && typeof e === "object" && "code" in e && "message" in e) {
    return e as T.AppError;
  }
  return {
    code: "unknown",
    message: e instanceof Error ? e.message : String(e),
    detail: null,
  };
}

async function call<R>(cmd: string, args?: Record<string, unknown>): Promise<R> {
  try {
    return await invoke<R>(cmd, args);
  } catch (e) {
    throw asAppError(e);
  }
}

// --- course -----------------------------------------------------------------

export const courseGet = () => call<T.CourseView>("course_get");
export const courseUpdate = (patch: T.CoursePatch) =>
  call<T.CourseView>("course_update", { patch });

// --- chapters ---------------------------------------------------------------

export const chapterList = (includeDeleted = false) =>
  call<T.ChapterSummary[]>("chapter_list", { includeDeleted });
export const chapterGet = (id: string) => call<T.ChapterView>("chapter_get", { id });
export const chapterCreate = (req: T.NewChapter) =>
  call<T.ChapterView>("chapter_create", { req });
export const chapterUpdate = (id: string, patch: T.ChapterPatch) =>
  call<T.ChapterView>("chapter_update", { id, patch });
export const chapterReorder = (id: string, afterId: string | null) =>
  call<void>("chapter_reorder", { id, afterId });
export const chapterDuplicate = (id: string) =>
  call<T.ChapterView>("chapter_duplicate", { id });
export const chapterDelete = (id: string) => call<void>("chapter_delete", { id });
export const chapterRestore = (id: string) => call<void>("chapter_restore", { id });
export const chapterPurge = (id: string) => call<void>("chapter_purge", { id });

// --- blocks -----------------------------------------------------------------

export const blockList = (chapterId: string) =>
  call<T.BlockView[]>("block_list", { chapterId });
export const blockGet = (id: string) => call<T.BlockView>("block_get", { id });

/**
 * `afterBlockId: null` appends. To insert above the very first block, create
 * then move to the front — see `blockCreateAtStart`.
 */
export const blockCreate = (
  chapterId: string,
  kind: T.BlockKind,
  afterBlockId: string | null,
  seed?: T.BlockSeed,
) => call<T.BlockView>("block_create", { chapterId, kind, afterBlockId, seed: seed ?? null });

export const blockUpdate = (id: string, patch: T.BlockPatch) =>
  call<T.BlockView>("block_update", { id, patch });
export const blockReorder = (
  id: string,
  chapterId: string,
  afterBlockId: string | null,
) => call<void>("block_reorder", { id, chapterId, afterBlockId });
export const blockConvert = (id: string, newKind: T.BlockKind) =>
  call<T.BlockView>("block_convert", { id, newKind });
export const blockConvertPreview = (id: string, newKind: T.BlockKind) =>
  call<string[]>("block_convert_preview", { id, newKind });
export const blockDelete = (id: string) => call<void>("block_delete", { id });
export const blockRestore = (id: string) => call<void>("block_restore", { id });
export const blockPurge = (id: string) => call<void>("block_purge", { id });

/** The one placement the command surface cannot express in a single call. */
export async function blockCreateAtStart(
  chapterId: string,
  kind: T.BlockKind,
  seed?: T.BlockSeed,
): Promise<T.BlockView> {
  const created = await blockCreate(chapterId, kind, null, seed);
  await blockReorder(created.id, chapterId, null);
  return blockGet(created.id);
}

// --- trash ------------------------------------------------------------------

export const trashList = () => call<T.TrashView>("trash_list");
export const trashSweep = () => call<number>("trash_sweep");

// --- assets -----------------------------------------------------------------

export const assetImport = (req: T.ImportRequest) =>
  call<T.ImportTicket>("asset_import", { req });
export const assetImportCancel = (ticketId: string) =>
  call<void>("asset_import_cancel", { ticketId });
export const assetAttach = (blockId: string, assetId: string) =>
  call<T.BlockView>("asset_attach", { blockId, assetId });
export const assetDetach = (blockId: string) =>
  call<T.BlockView>("asset_detach", { blockId });
export const assetSrc = (assetId: string) => call<string>("asset_src", { assetId });
export const assetGet = (assetId: string) => call<T.AssetView>("asset_get", { assetId });
export const assetRelink = (assetId: string, newPath: string) =>
  call<T.AssetView>("asset_relink", { assetId, newPath });
export const assetReveal = (assetId: string) => call<void>("asset_reveal", { assetId });
export const assetOpenExternal = (assetId: string) =>
  call<void>("asset_open_external", { assetId });
export const assetSetMediaMeta = (
  assetId: string,
  durationSecs: number | null,
  width: number | null,
  height: number | null,
) => call<void>("asset_set_media_meta", { assetId, durationSecs, width, height });
export const assetSetThumbnail = (assetId: string, pngBase64: string) =>
  call<string>("asset_set_thumbnail", { assetId, pngBase64 });
export const assetGc = (dryRun: boolean) => call<T.GcReport>("asset_gc", { dryRun });

// --- progress ---------------------------------------------------------------

export const progressSet = (blockId: string, patch: T.ProgressPatch) =>
  call<T.ProgressView>("progress_set", { blockId, patch });
export const progressSetChapter = (chapterId: string, completed: boolean) =>
  call<number>("progress_set_chapter", { chapterId, completed });
export const progressCourse = () => call<T.CourseProgress>("progress_course");

// --- markers and notes ------------------------------------------------------

export const markerCreate = (
  blockId: string,
  atSecs: number,
  label: string,
  noteMd: string | null = null,
) => call<T.Marker>("marker_create", { blockId, atSecs, label, noteMd });
export const markerDelete = (id: string) => call<void>("marker_delete", { id });
export const noteList = (chapterId: string) => call<T.Note[]>("note_list", { chapterId });
export const noteCreate = (chapterId: string, bodyMd: string) =>
  call<T.Note>("note_create", { chapterId, bodyMd });
export const noteUpdate = (id: string, bodyMd: string) =>
  call<T.Note>("note_update", { id, bodyMd });
export const noteDelete = (id: string) => call<void>("note_delete", { id });

// --- outline, search, bundle, system ----------------------------------------

export const outlinePreview = (md: string) => call<T.ParsedOutline>("outline_preview", { md });
export const outlineImportMarkdown = (md: string, mode: T.OutlineMode) =>
  call<T.OutlineImportReport>("outline_import_markdown", { md, mode });
export const searchQuery = (q: string, limit = 40) =>
  call<T.SearchHit[]>("search_query", { q, limit });
export const bundleExport = (destPath: string, includeMedia: boolean) =>
  call<T.ExportReport>("bundle_export", { destPath, includeMedia });
export const bundleImport = (srcPath: string, mode: T.BundleMode) =>
  call<T.BundleImportReport>("bundle_import", { srcPath, mode });
export const systemInfo = () => call<T.SystemInfo>("system_info");
export const integrityCheck = () => call<T.IntegrityIssue[]>("integrity_check");
export const openPath = (path: string) => call<void>("open_path", { path });
export const revealPath = (path: string) => call<void>("reveal_path", { path });
