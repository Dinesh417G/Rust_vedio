import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import * as ipc from "@/lib/ipc";
import type { OutlineMode, ParsedOutline } from "@/lib/types";
import { useUi } from "@/store/ui";
import { cx } from "@/lib/format";
import { Modal } from "./ui/Modal";
import { StateDot } from "./StateDot";
import { KIND_LABEL } from "./blocks/common";

const SAMPLE = `# Rust from the ground up

## 1. Getting set up
Everything you need before writing a line of Rust.
- [text] Installing the toolchain
- [code] Hello, world

## 4. Ownership & Borrowing
- [video] Move semantics — This video covers moves, clones and the borrow checker
- [text] Why the borrow checker exists
- [image] Stack vs heap diagram
- [quiz] Check yourself`;

export function OutlineImportDialog({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const [md, setMd] = useState("");
  const [mode, setMode] = useState<OutlineMode>("append");
  const [preview, setPreview] = useState<ParsedOutline | null>(null);
  const [busy, setBusy] = useState(false);
  const toast = useUi((s) => s.toast);
  const navigate = useNavigate();

  // The preview is the parser's own output, so what you see is what commits.
  useEffect(() => {
    if (!open || !md.trim()) {
      setPreview(null);
      return;
    }
    let live = true;
    const t = window.setTimeout(() => {
      ipc
        .outlinePreview(md)
        .then((p) => live && setPreview(p))
        .catch(() => live && setPreview(null));
    }, 200);
    return () => {
      live = false;
      window.clearTimeout(t);
    };
  }, [md, open]);

  const counts = useMemo(() => {
    if (!preview) return [];
    const map = new Map<string, number>();
    for (const c of preview.chapters)
      for (const b of c.blocks) map.set(b.kind, (map.get(b.kind) ?? 0) + 1);
    return [...map.entries()];
  }, [preview]);

  const blockTotal = counts.reduce((n, [, v]) => n + v, 0);

  const commit = async () => {
    if (!preview || busy) return;
    setBusy(true);
    try {
      const report = await ipc.outlineImportMarkdown(md, mode);
      toast({
        tone: "success",
        message: `Created ${report.chaptersCreated} chapter${
          report.chaptersCreated === 1 ? "" : "s"
        } and ${report.blocksCreated} block${report.blocksCreated === 1 ? "" : "s"}.${
          report.chaptersRemoved > 0
            ? ` ${report.chaptersRemoved} previous chapter${
                report.chaptersRemoved === 1 ? "" : "s"
              } moved to the Trash.`
            : ""
        }`,
      });
      setMd("");
      onClose();
      navigate("/");
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
      width="lg"
      title="Import an outline"
      description="Paste a markdown outline and RustClass builds the whole structure — every block empty and ready to fill in."
      footer={
        <>
          <button className="btn" onClick={onClose}>
            Cancel
          </button>
          <button
            className="btn btn-primary"
            disabled={!preview || preview.chapters.length === 0 || busy}
            onClick={commit}
          >
            {busy
              ? "Creating…"
              : `Create ${preview?.chapters.length ?? 0} chapter${
                  preview?.chapters.length === 1 ? "" : "s"
                }`}
          </button>
        </>
      }
    >
      <div className="space-y-4">
        <div className="flex items-center gap-2">
          <span className="label mb-0">Mode</span>
          {(["append", "replace"] as OutlineMode[]).map((m) => (
            <button
              key={m}
              className={cx("btn btn-sm", mode === m && "border-brand text-brand")}
              onClick={() => setMode(m)}
            >
              {m === "append" ? "Add to the course" : "Replace the course"}
            </button>
          ))}
        </div>
        {mode === "replace" && (
          <p className="rounded-lg border border-warn/40 bg-warn/10 px-3 py-2 text-xs text-ink">
            Every existing chapter moves to the Trash, where it can be restored for 30
            days. No media file is deleted.
          </p>
        )}

        <div>
          <div className="mb-1 flex items-center justify-between">
            <label className="label mb-0" htmlFor="outline-md">
              Outline
            </label>
            <button className="btn btn-ghost btn-sm" onClick={() => setMd(SAMPLE)}>
              Paste an example
            </button>
          </div>
          <textarea
            id="outline-md"
            className="input min-h-[14rem] resize-y font-mono text-[13px]"
            value={md}
            placeholder={"# Course title\n## Chapter\n- [video] A lesson\n- [text] Some prose"}
            onChange={(e) => setMd(e.target.value)}
          />
          <p className="mt-1 text-xs text-ink-faint">
            <code>#</code> course title · <code>##</code> chapter · bullets become blocks.
            Tag a bullet with <code>[video]</code>, <code>[text]</code>, <code>[code]</code>,{" "}
            <code>[image]</code>, <code>[quiz]</code>, <code>[callout]</code>,{" "}
            <code>[attachment]</code>. Untagged bullets become text. Add a description
            after <code>—</code>.
          </p>
        </div>

        {preview && (
          <div className="rounded-xl border border-line bg-surface-2 p-3">
            <p className="mb-2 text-sm font-medium text-ink">
              This will create {preview.chapters.length} chapter
              {preview.chapters.length === 1 ? "" : "s"} and {blockTotal} block
              {blockTotal === 1 ? "" : "s"}
              {preview.courseTitle ? `, and rename the course to "${preview.courseTitle}"` : ""}.
            </p>
            <div className="mb-3 flex flex-wrap gap-1.5">
              {counts.map(([kind, n]) => (
                <span key={kind} className="chip">
                  <StateDot state="empty" />
                  {n} × {KIND_LABEL[kind as keyof typeof KIND_LABEL]}
                </span>
              ))}
            </div>

            <ul className="max-h-52 space-y-1.5 overflow-y-auto">
              {preview.chapters.map((c, i) => (
                <li key={i} className="rounded-lg bg-surface-1 px-2.5 py-1.5">
                  <p className="text-sm text-ink">
                    <span className="mr-1.5 font-mono text-xs text-brand">
                      {c.displayNumber ?? i + 1}
                    </span>
                    {c.title}
                  </p>
                  {c.blocks.length > 0 && (
                    <p className="mt-0.5 flex flex-wrap gap-1 text-[11px] text-ink-faint">
                      {c.blocks.map((b, j) => (
                        <span key={j} className="inline-flex items-center gap-1">
                          <StateDot state="empty" />
                          {b.heading}
                        </span>
                      ))}
                    </p>
                  )}
                </li>
              ))}
            </ul>

            {preview.warnings.length > 0 && (
              <ul className="mt-3 space-y-1 border-t border-line pt-2">
                {preview.warnings.map((w, i) => (
                  <li key={i} className="text-xs text-warn">
                    {w}
                  </li>
                ))}
              </ul>
            )}
          </div>
        )}
      </div>
    </Modal>
  );
}
