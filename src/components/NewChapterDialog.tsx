import { useState } from "react";
import { useNavigate } from "react-router-dom";
import * as ipc from "@/lib/ipc";
import type { ChapterTemplate } from "@/lib/types";
import { useUi } from "@/store/ui";
import { cx } from "@/lib/format";
import { Modal } from "./ui/Modal";

const TEMPLATES: {
  id: ChapterTemplate;
  label: string;
  blurb: string;
  shape: string;
}[] = [
  { id: "blank", label: "Blank", blurb: "Just the chapter", shape: "—" },
  {
    id: "lesson",
    label: "Lesson",
    blurb: "Teach something with a recording",
    shape: "Text → Video → Code → Text",
  },
  {
    id: "concept",
    label: "Concept",
    blurb: "Explain an idea with a picture",
    shape: "Text → Image → Callout",
  },
  {
    id: "practice",
    label: "Practice",
    blurb: "Make them do the work",
    shape: "Text → Code → Quiz",
  },
];

export function NewChapterDialog({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const navigate = useNavigate();
  const toast = useUi((s) => s.toast);
  const [title, setTitle] = useState("");
  const [number, setNumber] = useState("");
  const [template, setTemplate] = useState<ChapterTemplate>("lesson");
  const [busy, setBusy] = useState(false);

  const reset = () => {
    setTitle("");
    setNumber("");
    setTemplate("lesson");
  };

  const create = async () => {
    if (!title.trim() || busy) return;
    setBusy(true);
    try {
      const created = await ipc.chapterCreate({
        title: title.trim(),
        displayNumber: number.trim() || null,
        template,
      });
      reset();
      onClose();
      navigate(`/chapter/${created.id}`);
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
      title="New chapter"
      description="Pick a shape now; fill in the content whenever you get to it."
      footer={
        <>
          <button className="btn" onClick={onClose}>
            Cancel
          </button>
          <button
            className="btn btn-primary"
            disabled={!title.trim() || busy}
            onClick={create}
          >
            {busy ? "Creating…" : "Create chapter"}
          </button>
        </>
      }
    >
      <div className="space-y-4">
        <div className="flex gap-3">
          <div className="w-24 shrink-0">
            <label className="label" htmlFor="new-chapter-number">
              Number
            </label>
            <input
              id="new-chapter-number"
              className="input"
              value={number}
              placeholder="4"
              onChange={(e) => setNumber(e.target.value)}
            />
          </div>
          <div className="min-w-0 flex-1">
            <label className="label" htmlFor="new-chapter-title">
              Title
            </label>
            <input
              id="new-chapter-title"
              autoFocus
              className="input"
              value={title}
              placeholder="Ownership & Borrowing"
              onChange={(e) => setTitle(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") void create();
              }}
            />
          </div>
        </div>

        <fieldset>
          <legend className="label">Scaffold</legend>
          <div className="grid gap-2 sm:grid-cols-2">
            {TEMPLATES.map((t) => (
              <button
                key={t.id}
                onClick={() => setTemplate(t.id)}
                className={cx(
                  "rounded-lg border p-3 text-left transition-colors",
                  template === t.id
                    ? "border-brand bg-brand/10"
                    : "border-line hover:bg-surface-2",
                )}
              >
                <span className="block text-sm font-medium text-ink">{t.label}</span>
                <span className="block text-xs text-ink-muted">{t.blurb}</span>
                <span className="mt-1 block font-mono text-[11px] text-ink-faint">
                  {t.shape}
                </span>
              </button>
            ))}
          </div>
          <p className="mt-2 text-xs text-ink-faint">
            Every scaffolded block starts empty, with a heading and a description you can
            edit right away.
          </p>
        </fieldset>
      </div>
    </Modal>
  );
}
