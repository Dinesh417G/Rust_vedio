import { useEffect, useState } from "react";
import { Clock, Target } from "lucide-react";
import type { ChapterPatch, ChapterView } from "@/lib/types";
import { chapterLabel, cx, minutesLabel } from "@/lib/format";
import { Markdown } from "./Markdown";
import { SaveIndicator } from "./SaveIndicator";
import { useAutosave } from "@/lib/autosave";
import { ProgressBar } from "./ui/ProgressRing";

interface Props {
  chapter: ChapterView;
  index: number;
  authoring: boolean;
  percent: number;
  onPatch: (patch: ChapterPatch) => Promise<unknown>;
}

export function ChapterHeader({ chapter, index, authoring, percent, onPatch }: Props) {
  const [title, setTitle] = useState(chapter.title);
  const [number, setNumber] = useState(chapter.displayNumber ?? "");
  const [summary, setSummary] = useState(chapter.summaryMd ?? "");
  const [objectives, setObjectives] = useState(chapter.objectivesMd ?? "");
  const [minutes, setMinutes] = useState(chapter.estimatedMinutes?.toString() ?? "");
  const [editingSummary, setEditingSummary] = useState(false);
  const [editingObjectives, setEditingObjectives] = useState(false);

  const { status, schedule } = useAutosave<ChapterPatch>(
    {},
    (patch: ChapterPatch) => onPatch(patch),
  );

  useEffect(() => {
    setTitle(chapter.title);
    setNumber(chapter.displayNumber ?? "");
    if (!editingSummary) setSummary(chapter.summaryMd ?? "");
    if (!editingObjectives) setObjectives(chapter.objectivesMd ?? "");
    setMinutes(chapter.estimatedMinutes?.toString() ?? "");
    // Only re-seed when the chapter itself changes.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [chapter.id, chapter.title, chapter.displayNumber, chapter.estimatedMinutes]);

  const label = chapterLabel(chapter.displayNumber, index);
  const time = minutesLabel(chapter.estimatedMinutes);

  if (!authoring) {
    return (
      <header className="mb-8 border-b border-line pb-6">
        <p className="mb-1 text-xs font-semibold uppercase tracking-widest text-brand">
          Chapter {label}
        </p>
        <h1 className="text-3xl font-bold leading-tight text-ink">{chapter.title}</h1>
        <div className="mt-2 flex flex-wrap items-center gap-3 text-sm text-ink-muted">
          {time && (
            <span className="inline-flex items-center gap-1">
              <Clock size={13} /> {time}
            </span>
          )}
          <span className="tabular-nums">{Math.round(percent)}% complete</span>
        </div>
        <ProgressBar percent={percent} className="mt-3 max-w-md" />
        {chapter.summaryMd?.trim() && (
          <div className="mt-4 max-w-prose">
            <Markdown>{chapter.summaryMd}</Markdown>
          </div>
        )}
        {chapter.objectivesMd?.trim() && (
          <div className="mt-4 max-w-prose rounded-xl border border-line bg-surface-1 p-4">
            <p className="mb-2 inline-flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-ink-faint">
              <Target size={12} /> What you will be able to do
            </p>
            <Markdown>{chapter.objectivesMd}</Markdown>
          </div>
        )}
      </header>
    );
  }

  return (
    <header className="mb-8 border-b border-line pb-6">
      <div className="flex items-start gap-3">
        <div className="w-24 shrink-0">
          <label className="label" htmlFor="chapter-number">
            Number
          </label>
          <input
            id="chapter-number"
            className="input"
            value={number}
            placeholder="4.2"
            onChange={(e) => {
              setNumber(e.target.value);
              schedule({ displayNumber: e.target.value.trim() || null });
            }}
          />
        </div>
        <div className="min-w-0 flex-1">
          <label className="label" htmlFor="chapter-title">
            Title
          </label>
          <input
            id="chapter-title"
            className={cx("input text-xl font-semibold", !title.trim() && "border-danger")}
            value={title}
            placeholder="Ownership & Borrowing"
            onChange={(e) => {
              setTitle(e.target.value);
              if (e.target.value.trim()) schedule({ title: e.target.value });
            }}
          />
        </div>
        <div className="w-28 shrink-0">
          <label className="label" htmlFor="chapter-minutes">
            Minutes
          </label>
          <input
            id="chapter-minutes"
            className="input"
            inputMode="numeric"
            value={minutes}
            placeholder="35"
            onChange={(e) => {
              const v = e.target.value.replace(/[^0-9]/g, "");
              setMinutes(v);
              schedule({ estimatedMinutes: v ? Number(v) : null });
            }}
          />
        </div>
      </div>

      <div className="mt-4 grid gap-4 md:grid-cols-2">
        <div>
          <label className="label">What this chapter is about</label>
          {editingSummary ? (
            <textarea
              autoFocus
              className="input min-h-[6rem] resize-y font-mono text-[13px]"
              value={summary}
              placeholder="A sentence or two, in markdown."
              onChange={(e) => {
                setSummary(e.target.value);
                schedule({ summaryMd: e.target.value });
              }}
              onBlur={() => setEditingSummary(false)}
            />
          ) : (
            <button
              className="min-h-[6rem] w-full rounded-lg border border-line bg-surface-2 p-3 text-left hover:border-brand/50"
              onClick={() => setEditingSummary(true)}
            >
              {summary.trim() ? (
                <Markdown>{summary}</Markdown>
              ) : (
                <span className="text-sm text-ink-faint">
                  Describe this chapter — you can do this long before recording anything.
                </span>
              )}
            </button>
          )}
        </div>
        <div>
          <label className="label">Learning outcomes</label>
          {editingObjectives ? (
            <textarea
              autoFocus
              className="input min-h-[6rem] resize-y font-mono text-[13px]"
              value={objectives}
              placeholder={"- Explain what a move is\n- Read a borrow-checker error"}
              onChange={(e) => {
                setObjectives(e.target.value);
                schedule({ objectivesMd: e.target.value });
              }}
              onBlur={() => setEditingObjectives(false)}
            />
          ) : (
            <button
              className="min-h-[6rem] w-full rounded-lg border border-line bg-surface-2 p-3 text-left hover:border-brand/50"
              onClick={() => setEditingObjectives(true)}
            >
              {objectives.trim() ? (
                <Markdown>{objectives}</Markdown>
              ) : (
                <span className="text-sm text-ink-faint">
                  A bullet list of what the reader will be able to do.
                </span>
              )}
            </button>
          )}
        </div>
      </div>

      <div className="mt-3 flex items-center justify-between">
        <span className="text-xs text-ink-faint">
          Chapter {label} · {Math.round(percent)}% complete
        </span>
        <SaveIndicator status={status} />
      </div>
    </header>
  );
}
