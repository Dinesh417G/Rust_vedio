import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import {
  Copy,
  FolderOpen,
  HeartPulse,
  Package,
  PackageOpen,
  RotateCcw,
  Trash2,
} from "lucide-react";
import * as ipc from "@/lib/ipc";
import type { GcReport } from "@/lib/types";
import { useCourse, useSystemInfo, useTrash } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { useAutosave } from "@/lib/autosave";
import { cx, daysRemaining, humanBytes, relativeDate } from "@/lib/format";
import { SaveIndicator } from "@/components/SaveIndicator";
import { ConfirmDialog } from "@/components/ui/ConfirmDialog";
import { BundleExportDialog, BundleImportDialog } from "@/components/BundleDialogs";
import { KIND_LABEL } from "@/components/blocks/common";

export function SettingsPage() {
  const { data: course } = useCourse();
  const { data: info } = useSystemInfo();
  const { data: trash } = useTrash();
  const toast = useUi((s) => s.toast);
  const theme = useUi((s) => s.theme);
  const setTheme = useUi((s) => s.setTheme);
  const importMode = useUi((s) => s.defaultImportMode);
  const setImportMode = useUi((s) => s.setDefaultImportMode);

  const [subtitle, setSubtitle] = useState("");
  const [author, setAuthor] = useState("");
  const [description, setDescription] = useState("");
  const [exportOpen, setExportOpen] = useState(false);
  const [importOpen, setImportOpen] = useState(false);
  const [gc, setGc] = useState<GcReport | null>(null);
  const [gcBusy, setGcBusy] = useState(false);

  const { status, schedule } = useAutosave<Record<string, unknown>>({}, (patch) =>
    ipc.courseUpdate(patch),
  );

  useEffect(() => {
    setSubtitle(course?.subtitle ?? "");
    setAuthor(course?.author ?? "");
    setDescription(course?.descriptionMd ?? "");
  }, [course?.id, course?.subtitle, course?.author, course?.descriptionMd]);

  const runGc = async (dryRun: boolean) => {
    setGcBusy(true);
    try {
      const report = await ipc.assetGc(dryRun);
      if (dryRun) {
        setGc(report);
        if (report.bytesReclaimable === 0)
          toast({ tone: "success", message: "Nothing to clean up — storage is tidy." });
      } else {
        setGc(null);
        toast({
          tone: "success",
          message: `Reclaimed ${humanBytes(report.bytesReclaimable)}.`,
        });
      }
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    } finally {
      setGcBusy(false);
    }
  };

  return (
    <div className="h-full overflow-y-auto">
      <div className="mx-auto max-w-3xl space-y-8 px-8 py-10">
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-bold text-ink">Settings</h1>
          <SaveIndicator status={status} />
        </div>

        {/* --- course ------------------------------------------------------ */}
        <Section title="Course">
          <div className="grid gap-3 sm:grid-cols-2">
            <Field label="Subtitle">
              <input
                className="input"
                value={subtitle}
                placeholder="A course I am building"
                onChange={(e) => {
                  setSubtitle(e.target.value);
                  schedule({ subtitle: e.target.value || null });
                }}
              />
            </Field>
            <Field label="Author">
              <input
                className="input"
                value={author}
                placeholder="Your name"
                onChange={(e) => {
                  setAuthor(e.target.value);
                  schedule({ author: e.target.value || null });
                }}
              />
            </Field>
          </div>
          <Field label="Description">
            <textarea
              className="input min-h-[6rem] resize-y font-mono text-[13px]"
              value={description}
              placeholder="What this course covers, in markdown."
              onChange={(e) => {
                setDescription(e.target.value);
                schedule({ descriptionMd: e.target.value || null });
              }}
            />
          </Field>
        </Section>

        {/* --- appearance -------------------------------------------------- */}
        <Section title="Appearance">
          <Field label="Theme">
            <div className="flex gap-1">
              {(["dark", "light"] as const).map((t) => (
                <button
                  key={t}
                  className={cx("btn btn-sm capitalize", theme === t && "border-brand text-brand")}
                  onClick={() => setTheme(t)}
                >
                  {t}
                </button>
              ))}
            </div>
          </Field>
        </Section>

        {/* --- media ------------------------------------------------------- */}
        <Section title="Media">
          <Field label="How files are imported">
            <div className="space-y-2">
              {(
                [
                  {
                    id: "copy" as const,
                    label: "Copy into the app",
                    blurb:
                      "The file is streamed into the RustClass media folder. Moving or deleting your original afterwards changes nothing.",
                  },
                  {
                    id: "reference" as const,
                    label: "Reference where it is",
                    blurb:
                      "Nothing is copied. Moving the original will break the link, and the Health page will flag it.",
                  },
                ]
              ).map((m) => (
                <button
                  key={m.id}
                  onClick={() => setImportMode(m.id)}
                  className={cx(
                    "block w-full rounded-lg border p-3 text-left transition-colors",
                    importMode === m.id
                      ? "border-brand bg-brand/10"
                      : "border-line hover:bg-surface-2",
                  )}
                >
                  <span className="block text-sm font-medium text-ink">{m.label}</span>
                  <span className="block text-xs text-ink-muted">{m.blurb}</span>
                </button>
              ))}
            </div>
          </Field>
        </Section>

        {/* --- backup ------------------------------------------------------ */}
        <Section title="Backup and transfer">
          <div className="flex flex-wrap gap-2">
            <button className="btn" onClick={() => setExportOpen(true)}>
              <Package size={14} /> Export bundle
            </button>
            <button className="btn" onClick={() => setImportOpen(true)}>
              <PackageOpen size={14} /> Import bundle
            </button>
          </div>
        </Section>

        {/* --- trash ------------------------------------------------------- */}
        <Section
          title={`Trash${
            trash ? ` (${trash.chapters.length + trash.blocks.length})` : ""
          }`}
        >
          <p className="mb-3 text-xs text-ink-muted">
            Deleted chapters and blocks stay here for {trash?.retentionDays ?? 30} days.
            Deleting permanently never touches a media file — only storage cleanup does
            that, and only after it shows you the list.
          </p>

          {trash && trash.chapters.length === 0 && trash.blocks.length === 0 && (
            <p className="text-sm text-ink-muted">The Trash is empty.</p>
          )}

          <ul className="space-y-1.5">
            {trash?.chapters.map((c) => (
              <TrashRow
                key={c.id}
                title={c.title}
                subtitle={`Chapter · ${c.blockCount} block${c.blockCount === 1 ? "" : "s"}`}
                deletedAt={c.deletedAt ?? 0}
                retentionDays={trash.retentionDays}
                onRestore={() => ipc.chapterRestore(c.id)}
                onPurge={() => ipc.chapterPurge(c.id)}
                purgeConsequence={`"${c.title}" and its ${c.blockCount} block${
                  c.blockCount === 1 ? "" : "s"
                } are removed from the database for good. No media file is deleted.`}
              />
            ))}
            {trash?.blocks.map((b) => (
              <TrashRow
                key={b.id}
                title={b.heading?.trim() || `Untitled ${KIND_LABEL[b.kind].toLowerCase()}`}
                subtitle={`${KIND_LABEL[b.kind]} · in "${b.chapterTitle}"`}
                deletedAt={b.deletedAt ?? 0}
                retentionDays={trash.retentionDays}
                onRestore={() => ipc.blockRestore(b.id)}
                onPurge={() => ipc.blockPurge(b.id)}
                purgeConsequence="This block is removed from the database for good. Any file it pointed at stays in your media library."
              />
            ))}
          </ul>
        </Section>

        {/* --- storage ----------------------------------------------------- */}
        <Section title="Storage">
          {info && (
            <dl className="mb-3 space-y-1 text-sm">
              <InfoRow label="Course database" value={humanBytes(info.dbBytes)} path={info.dbPath} />
              <InfoRow label="Media folder" value={humanBytes(info.mediaBytes)} path={info.mediaDir} />
              <InfoRow label="Logs" value="" path={info.logDir} />
              <InfoRow label="App data" value="" path={info.appDataDir} />
            </dl>
          )}

          <div className="flex flex-wrap gap-2">
            <button className="btn btn-sm" disabled={gcBusy} onClick={() => void runGc(true)}>
              {gcBusy ? "Checking…" : "Check for reclaimable space"}
            </button>
            <Link to="/health" className="btn btn-sm">
              <HeartPulse size={13} /> Health report
            </Link>
          </div>

          {gc && (
            <div className="mt-3 rounded-lg border border-line bg-surface-2 p-3">
              {gc.bytesReclaimable === 0 ? (
                <p className="text-sm text-ink-muted">Nothing to clean up.</p>
              ) : (
                <>
                  <p className="mb-2 text-sm text-ink">
                    Cleaning up would free {humanBytes(gc.bytesReclaimable)}:
                  </p>
                  <GcList label="Files no asset points at" entries={gc.orphanFiles} />
                  <GcList label="Unfinished imports" entries={gc.partialFiles} />
                  <GcList label="Assets no block uses" entries={gc.unreferencedAssets} />
                  <GcConfirm onConfirm={() => void runGc(false)} report={gc} />
                </>
              )}
            </div>
          )}
        </Section>

        {info && (
          <p className="pb-6 text-center text-xs text-ink-faint">
            RustClass {info.appVersion} · schema v{info.schemaVersion} · bundle format v
            {info.bundleSchemaVersion}
          </p>
        )}
      </div>

      <BundleExportDialog open={exportOpen} onClose={() => setExportOpen(false)} />
      <BundleImportDialog open={importOpen} onClose={() => setImportOpen(false)} />
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="rounded-xl border border-line bg-surface-1 p-5">
      <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-ink-faint">
        {title}
      </h2>
      <div className="space-y-3">{children}</div>
    </section>
  );
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div>
      <span className="label">{label}</span>
      {children}
    </div>
  );
}

function InfoRow({
  label,
  value,
  path,
}: {
  label: string;
  value: string;
  path: string;
}) {
  const toast = useUi((s) => s.toast);
  return (
    <div className="flex items-center gap-2">
      <dt className="w-36 shrink-0 text-ink-muted">{label}</dt>
      <dd className="min-w-0 flex-1 truncate font-mono text-xs text-ink-faint" title={path}>
        {path}
      </dd>
      {value && <span className="shrink-0 tabular-nums text-ink">{value}</span>}
      <button
        className="btn btn-ghost btn-sm shrink-0"
        title="Copy path"
        onClick={async () => {
          try {
            await navigator.clipboard.writeText(path);
            toast({ tone: "success", message: "Path copied." });
          } catch {
            toast({ tone: "error", message: "The clipboard is not available." });
          }
        }}
      >
        <Copy size={12} />
      </button>
      <button
        className="btn btn-ghost btn-sm shrink-0"
        title="Open folder"
        onClick={() => void ipc.revealPath(path).catch(() => undefined)}
      >
        <FolderOpen size={12} />
      </button>
    </div>
  );
}

function TrashRow({
  title,
  subtitle,
  deletedAt,
  retentionDays,
  onRestore,
  onPurge,
  purgeConsequence,
}: {
  title: string;
  subtitle: string;
  deletedAt: number;
  retentionDays: number;
  onRestore: () => Promise<unknown>;
  onPurge: () => Promise<unknown>;
  purgeConsequence: string;
}) {
  const toast = useUi((s) => s.toast);
  const [confirm, setConfirm] = useState(false);
  const left = daysRemaining(deletedAt, retentionDays);

  return (
    <li className="flex items-center gap-3 rounded-lg border border-line bg-surface-2 px-3 py-2">
      <div className="min-w-0 flex-1">
        <p className="truncate text-sm text-ink">{title}</p>
        <p className="text-xs text-ink-faint">
          {subtitle} · deleted {relativeDate(deletedAt)} · {left} day
          {left === 1 ? "" : "s"} left
        </p>
      </div>
      <button
        className="btn btn-sm"
        onClick={async () => {
          try {
            await onRestore();
            toast({ tone: "success", message: `Restored "${title}".` });
          } catch (e) {
            toast({ tone: "error", message: ipc.asAppError(e).message });
          }
        }}
      >
        <RotateCcw size={12} /> Restore
      </button>
      <button className="btn btn-sm btn-ghost text-danger" onClick={() => setConfirm(true)}>
        <Trash2 size={12} />
      </button>

      <ConfirmDialog
        open={confirm}
        title={`Permanently delete "${title}"?`}
        consequence={purgeConsequence}
        confirmLabel="Delete permanently"
        onCancel={() => setConfirm(false)}
        onConfirm={async () => {
          setConfirm(false);
          try {
            await onPurge();
          } catch (e) {
            toast({ tone: "error", message: ipc.asAppError(e).message });
          }
        }}
      />
    </li>
  );
}

function GcList({
  label,
  entries,
}: {
  label: string;
  entries: GcReport["orphanFiles"];
}) {
  if (entries.length === 0) return null;
  return (
    <div className="mb-2">
      <p className="text-xs font-medium text-ink-muted">
        {label} ({entries.length})
      </p>
      <ul className="mt-1 max-h-32 space-y-0.5 overflow-y-auto">
        {entries.map((e) => (
          <li key={e.id} className="flex justify-between gap-2 text-xs">
            <span className="truncate font-mono text-ink-faint" title={e.path}>
              {e.label || e.path}
            </span>
            <span className="shrink-0 tabular-nums text-ink-muted">
              {humanBytes(e.bytes)}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function GcConfirm({
  report,
  onConfirm,
}: {
  report: GcReport;
  onConfirm: () => void;
}) {
  const [open, setOpen] = useState(false);
  const total =
    report.orphanFiles.length + report.partialFiles.length + report.unreferencedAssets.length;
  return (
    <>
      <button className="btn btn-sm btn-danger mt-2" onClick={() => setOpen(true)}>
        <Trash2 size={13} /> Delete these {total} item{total === 1 ? "" : "s"}
      </button>
      <ConfirmDialog
        open={open}
        title="Delete these files for good?"
        consequence={`${total} item${
          total === 1 ? "" : "s"
        } listed above will be removed from disk, freeing ${humanBytes(
          report.bytesReclaimable,
        )}. Files you imported by reference are never deleted — only their database rows.`}
        confirmLabel="Delete permanently"
        onCancel={() => setOpen(false)}
        onConfirm={() => {
          setOpen(false);
          onConfirm();
        }}
      />
    </>
  );
}
