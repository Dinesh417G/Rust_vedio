import { useState } from "react";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
import { Package, PackageOpen } from "lucide-react";
import * as ipc from "@/lib/ipc";
import type { BundleMode } from "@/lib/types";
import { useUi } from "@/store/ui";
import { useHardRefresh } from "@/lib/queries";
import { cx, humanBytes } from "@/lib/format";
import { Modal } from "./ui/Modal";

const FILTERS = [{ name: "RustClass bundle", extensions: ["rcbundle"] }];

export function BundleExportDialog({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const [includeMedia, setIncludeMedia] = useState(true);
  const [busy, setBusy] = useState(false);
  const toast = useUi((s) => s.toast);

  const run = async () => {
    if (busy) return;
    try {
      const dest = await saveDialog({
        title: "Export course bundle",
        defaultPath: "rustclass-course.rcbundle",
        filters: FILTERS,
      });
      if (!dest) return;
      setBusy(true);
      const report = await ipc.bundleExport(dest, includeMedia);
      toast({
        tone: "success",
        message: `Exported ${report.chapters} chapters and ${report.mediaFiles} media file${
          report.mediaFiles === 1 ? "" : "s"
        } (${humanBytes(report.bytes)}).`,
        action: {
          label: "Show in folder",
          run: () => void ipc.revealPath(report.path).catch(() => undefined),
        },
      });
      if (report.mediaSkipped.length > 0) {
        toast({
          tone: "error",
          message: `${report.mediaSkipped.length} file${
            report.mediaSkipped.length === 1 ? " was" : "s were"
          } missing from disk and could not be included.`,
        });
      }
      onClose();
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title="Export a bundle"
      description="A .rcbundle is a ZIP holding the whole course plus, optionally, its media. It is your backup and your way of handing the course to someone else."
      footer={
        <>
          <button className="btn" onClick={onClose}>
            Cancel
          </button>
          <button className="btn btn-primary" disabled={busy} onClick={run}>
            <Package size={14} /> {busy ? "Exporting…" : "Choose location and export"}
          </button>
        </>
      }
    >
      <label className="flex cursor-pointer items-start gap-3 rounded-lg border border-line p-3">
        <input
          type="checkbox"
          className="mt-1"
          checked={includeMedia}
          onChange={(e) => setIncludeMedia(e.target.checked)}
        />
        <span>
          <span className="block text-sm font-medium text-ink">Include media</span>
          <span className="block text-xs text-ink-muted">
            Videos, images and attachments travel with the bundle. Without them the
            structure still imports, and every media block shows as missing until you
            relink it.
          </span>
        </span>
      </label>
    </Modal>
  );
}

export function BundleImportDialog({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const [mode, setMode] = useState<BundleMode>("merge");
  const [busy, setBusy] = useState(false);
  const toast = useUi((s) => s.toast);
  const refresh = useHardRefresh();

  const run = async () => {
    if (busy) return;
    try {
      const src = await openDialog({
        multiple: false,
        directory: false,
        title: "Import course bundle",
        filters: FILTERS,
      });
      if (typeof src !== "string") return;
      setBusy(true);
      const report = await ipc.bundleImport(src, mode);
      refresh();
      toast({
        tone: "success",
        message: `Imported ${report.chapters} chapters, ${report.blocks} blocks and ${report.mediaFiles} media file${
          report.mediaFiles === 1 ? "" : "s"
        }.`,
      });
      report.warnings.forEach((w) => toast({ tone: "error", message: w }));
      onClose();
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title="Import a bundle"
      description="Bring in a .rcbundle exported from RustClass."
      footer={
        <>
          <button className="btn" onClick={onClose}>
            Cancel
          </button>
          <button className="btn btn-primary" disabled={busy} onClick={run}>
            <PackageOpen size={14} /> {busy ? "Importing…" : "Choose a bundle"}
          </button>
        </>
      }
    >
      <div className="space-y-2">
        {(
          [
            {
              id: "merge" as BundleMode,
              label: "Merge",
              blurb:
                "Add the bundle's chapters after the ones already here, with fresh IDs. Nothing existing is touched.",
            },
            {
              id: "replace" as BundleMode,
              label: "Replace",
              blurb:
                "Clear the current chapters and blocks first. Media files on disk are never deleted — only asset_gc does that, and only after you confirm.",
            },
          ] as const
        ).map((m) => (
          <button
            key={m.id}
            onClick={() => setMode(m.id)}
            className={cx(
              "block w-full rounded-lg border p-3 text-left transition-colors",
              mode === m.id ? "border-brand bg-brand/10" : "border-line hover:bg-surface-2",
            )}
          >
            <span className="block text-sm font-medium text-ink">{m.label}</span>
            <span className="block text-xs text-ink-muted">{m.blurb}</span>
          </button>
        ))}
      </div>
    </Modal>
  );
}
