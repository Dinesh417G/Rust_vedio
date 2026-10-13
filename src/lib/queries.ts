/**
 * TanStack Query keys and hooks. `data://changed` from Rust drives the
 * invalidations, so a mutation in one panel refreshes every other view without
 * any of them knowing about each other.
 */
import {
  useMutation,
  useQuery,
  useQueryClient,
  type QueryClient,
} from "@tanstack/react-query";
import * as ipc from "./ipc";
import type * as T from "./types";

export const qk = {
  course: ["course"] as const,
  chapters: (includeDeleted = false) => ["chapters", includeDeleted] as const,
  chapter: (id: string) => ["chapter", id] as const,
  blocks: (chapterId: string) => ["blocks", chapterId] as const,
  block: (id: string) => ["block", id] as const,
  progress: ["progress"] as const,
  trash: ["trash"] as const,
  notes: (chapterId: string) => ["notes", chapterId] as const,
  search: (q: string) => ["search", q] as const,
  system: ["system"] as const,
  integrity: ["integrity"] as const,
  asset: (id: string) => ["asset", id] as const,
};

/** Maps a `data://changed` entity onto the keys it invalidates. */
export function invalidateFor(qc: QueryClient, entity: T.DataChanged["entity"]) {
  const all = () => qc.invalidateQueries();
  switch (entity) {
    case "course":
      qc.invalidateQueries({ queryKey: qk.course });
      qc.invalidateQueries({ queryKey: ["chapters"] });
      break;
    case "chapter":
      qc.invalidateQueries({ queryKey: ["chapters"] });
      qc.invalidateQueries({ queryKey: ["chapter"] });
      qc.invalidateQueries({ queryKey: qk.trash });
      qc.invalidateQueries({ queryKey: qk.progress });
      break;
    case "block":
      qc.invalidateQueries({ queryKey: ["blocks"] });
      qc.invalidateQueries({ queryKey: ["block"] });
      qc.invalidateQueries({ queryKey: ["chapter"] });
      qc.invalidateQueries({ queryKey: ["chapters"] });
      qc.invalidateQueries({ queryKey: qk.trash });
      break;
    case "asset":
      qc.invalidateQueries({ queryKey: ["asset"] });
      qc.invalidateQueries({ queryKey: ["blocks"] });
      qc.invalidateQueries({ queryKey: ["block"] });
      qc.invalidateQueries({ queryKey: ["chapter"] });
      qc.invalidateQueries({ queryKey: ["chapters"] });
      qc.invalidateQueries({ queryKey: qk.integrity });
      qc.invalidateQueries({ queryKey: qk.system });
      break;
    case "progress":
      qc.invalidateQueries({ queryKey: qk.progress });
      qc.invalidateQueries({ queryKey: ["chapters"] });
      qc.invalidateQueries({ queryKey: ["chapter"] });
      qc.invalidateQueries({ queryKey: ["blocks"] });
      break;
    case "marker":
      qc.invalidateQueries({ queryKey: ["blocks"] });
      qc.invalidateQueries({ queryKey: ["block"] });
      qc.invalidateQueries({ queryKey: ["chapter"] });
      break;
    case "note":
      qc.invalidateQueries({ queryKey: ["notes"] });
      break;
    default:
      all();
  }
}

// --- reads ------------------------------------------------------------------

export const useCourse = () =>
  useQuery({ queryKey: qk.course, queryFn: ipc.courseGet });

export const useChapters = (includeDeleted = false) =>
  useQuery({
    queryKey: qk.chapters(includeDeleted),
    queryFn: () => ipc.chapterList(includeDeleted),
  });

export const useChapter = (id: string | undefined) =>
  useQuery({
    queryKey: qk.chapter(id ?? ""),
    queryFn: () => ipc.chapterGet(id as string),
    enabled: !!id,
  });

export const useCourseProgress = () =>
  useQuery({ queryKey: qk.progress, queryFn: ipc.progressCourse });

export const useTrash = () => useQuery({ queryKey: qk.trash, queryFn: ipc.trashList });

export const useNotes = (chapterId: string | undefined) =>
  useQuery({
    queryKey: qk.notes(chapterId ?? ""),
    queryFn: () => ipc.noteList(chapterId as string),
    enabled: !!chapterId,
  });

export const useSystemInfo = () =>
  useQuery({ queryKey: qk.system, queryFn: ipc.systemInfo });

export const useIntegrity = () =>
  useQuery({ queryKey: qk.integrity, queryFn: ipc.integrityCheck });

export const useSearch = (q: string, enabled = true) =>
  useQuery({
    queryKey: qk.search(q),
    queryFn: () => ipc.searchQuery(q),
    enabled: enabled && q.trim().length > 0,
  });

// --- writes -----------------------------------------------------------------
// Every mutation relies on the Rust side emitting `data://changed`, which the
// event bridge turns into the right invalidations. Mutations therefore do not
// invalidate anything themselves — one source of truth for cache freshness.

export const useCourseUpdate = () =>
  useMutation({ mutationFn: (patch: T.CoursePatch) => ipc.courseUpdate(patch) });

export const useChapterCreate = () =>
  useMutation({ mutationFn: (req: T.NewChapter) => ipc.chapterCreate(req) });

export const useChapterUpdate = () =>
  useMutation({
    mutationFn: ({ id, patch }: { id: string; patch: T.ChapterPatch }) =>
      ipc.chapterUpdate(id, patch),
  });

export const useChapterDelete = () =>
  useMutation({ mutationFn: (id: string) => ipc.chapterDelete(id) });

export const useChapterRestore = () =>
  useMutation({ mutationFn: (id: string) => ipc.chapterRestore(id) });

export const useChapterDuplicate = () =>
  useMutation({ mutationFn: (id: string) => ipc.chapterDuplicate(id) });

export const useChapterReorder = () =>
  useMutation({
    mutationFn: ({ id, afterId }: { id: string; afterId: string | null }) =>
      ipc.chapterReorder(id, afterId),
  });

export const useBlockCreate = () =>
  useMutation({
    mutationFn: ({
      chapterId,
      kind,
      afterBlockId,
      atStart,
      seed,
    }: {
      chapterId: string;
      kind: T.BlockKind;
      afterBlockId: string | null;
      atStart?: boolean;
      seed?: T.BlockSeed;
    }) =>
      atStart
        ? ipc.blockCreateAtStart(chapterId, kind, seed)
        : ipc.blockCreate(chapterId, kind, afterBlockId, seed),
  });

export const useBlockUpdate = () =>
  useMutation({
    mutationFn: ({ id, patch }: { id: string; patch: T.BlockPatch }) =>
      ipc.blockUpdate(id, patch),
  });

export const useBlockReorder = () =>
  useMutation({
    mutationFn: ({
      id,
      chapterId,
      afterBlockId,
    }: {
      id: string;
      chapterId: string;
      afterBlockId: string | null;
    }) => ipc.blockReorder(id, chapterId, afterBlockId),
  });

export const useBlockConvert = () =>
  useMutation({
    mutationFn: ({ id, newKind }: { id: string; newKind: T.BlockKind }) =>
      ipc.blockConvert(id, newKind),
  });

export const useBlockDelete = () =>
  useMutation({ mutationFn: (id: string) => ipc.blockDelete(id) });

export const useBlockRestore = () =>
  useMutation({ mutationFn: (id: string) => ipc.blockRestore(id) });

export const useProgressSet = () =>
  useMutation({
    mutationFn: ({ blockId, patch }: { blockId: string; patch: T.ProgressPatch }) =>
      ipc.progressSet(blockId, patch),
  });

export const useChapterComplete = () =>
  useMutation({
    mutationFn: ({ chapterId, completed }: { chapterId: string; completed: boolean }) =>
      ipc.progressSetChapter(chapterId, completed),
  });

export const useLibraryLoad = () =>
  useMutation({ mutationFn: (mode: T.LoadMode) => ipc.libraryLoadBuiltin(mode) });

/** Refetches everything — used after a bundle import replaces the world. */
export function useHardRefresh() {
  const qc = useQueryClient();
  return () => qc.invalidateQueries();
}
