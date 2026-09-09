import { useCallback } from "react";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import * as ipc from "@/lib/ipc";
import { useUi } from "@/store/ui";
import { useImports } from "@/store/imports";
import type { AssetKind, BlockKind, ImportMode } from "@/lib/types";

const FILTERS: Record<AssetKind, { name: string; extensions: string[] }[]> = {
  video: [
    { name: "Video", extensions: ["mp4", "m4v", "webm", "mkv", "mov", "avi", "ogv"] },
  ],
  image: [
    { name: "Image", extensions: ["png", "jpg", "jpeg", "gif", "webp", "bmp", "avif", "svg"] },
  ],
  file: [{ name: "All files", extensions: ["*"] }],
};

export function assetKindFor(kind: BlockKind): AssetKind {
  if (kind === "video") return "video";
  if (kind === "image") return "image";
  return "file";
}

/**
 * Starts an import and wires the ticket into the progress store.
 *
 * The command returns as soon as the job is queued; everything after that
 * arrives as `import://progress` / `done` / `failed`.
 */
export function useStartImport() {
  const toast = useUi((s) => s.toast);
  const defaultMode = useUi((s) => s.defaultImportMode);
  const start = useImports((s) => s.start);

  return useCallback(
    async (
      path: string,
      blockId: string | null,
      kind: AssetKind,
      mode: ImportMode = defaultMode,
    ) => {
      try {
        const ticket = await ipc.assetImport({ path, blockId, kind, mode });
        start({
          ticketId: ticket.ticketId,
          blockId: ticket.blockId,
          originalName: ticket.originalName,
          bytesTotal: ticket.bytesTotal,
        });
        return ticket;
      } catch (e) {
        toast({ tone: "error", message: ipc.asAppError(e).message });
        return null;
      }
    },
    [defaultMode, start, toast],
  );
}

/** Opens the native picker with the right filters, then imports the choice. */
export function usePickAndImport() {
  const startImport = useStartImport();
  const toast = useUi((s) => s.toast);

  return useCallback(
    async (blockId: string, blockKind: BlockKind) => {
      const kind = assetKindFor(blockKind);
      try {
        const chosen = await openDialog({
          multiple: false,
          directory: false,
          title:
            kind === "video"
              ? "Choose a video"
              : kind === "image"
                ? "Choose an image"
                : "Attach a file",
          filters: FILTERS[kind],
        });
        if (typeof chosen !== "string") return;
        await startImport(chosen, blockId, kind);
      } catch (e) {
        toast({ tone: "error", message: ipc.asAppError(e).message });
      }
    },
    [startImport, toast],
  );
}

/** The picker used by the `Missing` state's "Locate file…" button. */
export function useRelink() {
  const toast = useUi((s) => s.toast);
  return useCallback(
    async (assetId: string, originalName: string) => {
      try {
        const chosen = await openDialog({
          multiple: false,
          directory: false,
          title: `Locate ${originalName}`,
        });
        if (typeof chosen !== "string") return false;
        await ipc.assetRelink(assetId, chosen);
        toast({ tone: "success", message: `Relinked ${originalName}.` });
        return true;
      } catch (e) {
        toast({ tone: "error", message: ipc.asAppError(e).message });
        return false;
      }
    },
    [toast],
  );
}
