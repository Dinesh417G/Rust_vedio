import { useEffect, useMemo, useRef, useState } from "react";
import { Clock, Target } from "lucide-react";
import type { ChapterPatch, ChapterView } from "@/lib/types";
import { chapterLabel, cx, minutesLabel } from "@/lib/format";
import { useChapters } from "@/lib/queries";
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

/** Trimmed, non-empty values in first-seen order — the suggestions for a field. */
function distinct(values: Array<string | null | undefined>): string[] {
  const seen = new Set<string>();
  for (const v of values) {
    const t = v?.trim();
    if (t) seen.add(t);
  }
  return [...seen];
}

export function ChapterHeader({ chapter, index, authoring, percent, onPatch }: Props) {
  const [title, setTitle] = useState(chapter.title);
  const [number, setNumber] = useState(chapter.displayNumber ?? "");
  const [summary, setSummary] = useState(chapter.summaryMd ?? "");
  const [objectives, setObjectives] = useState(chapter.objectivesMd ?? "");
  const [minutes, setMinutes] = useState(chapter.estimatedMinutes?.toString() ?? "");
  const [part, setPart] = useState(chapter.part ?? "");
  const [section, setSection] = useState(chapter.section ?? "");
  const [editingSummary, setEditingSummary] = useState(false);
  const [editingObjectives, setEditingObjectives] = useState(false);

  // `schedule` keeps only the latest patch it was given, so two fields edited
  // inside one debounce window (Course, then Module) would lose the first.
  // Edits are folded together here and cleared once a save actually starts.
  const edits = useRef<ChapterPatch>({});
  const { status, schedule } = useAutosave<ChapterPatch>({}, (patch: ChapterPatch) => {
    edits.current = {};
    return onPatch(patch);
  });
  const queue = (patch: ChapterPatch) => {
    edits.current = { ...edits.current, ...patch };
    schedule(edits.current);
  };

  useEffect(() => {
    setTitle(chapter.title);
    setNumber(chapter.displayNumber ?? "");
    if (!editingSummary) setSummary(chapter.summaryMd ?? "");
    if (!editingObjectives) setObjectives(chapter.objectivesMd ?? "");
    setMinutes(chapter.estimatedMinutes?.toString() ?? "");
    // Only re-seed when the chapter itself changes.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [chapter.id, chapter.title, chapter.displayNumber, chapter.estimatedMinutes]);

  // Course and Module are re-seeded on a chapter change only. Following every
  // saved value would overwrite a character typed while the previous save was
  // still on its way back.
  useEffect(() => {
    setPart(chapter.part ?? "");
    setSection(chapter.section ?? "");
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [chapter.id]);

  // Moving a chapter to another course or module is done by typing its name
  // here, so offer the names that already exist rather than ask for them to be
  // retyped exactly.
  const { data: all = [] } = useChapters();
  const partOptions = useMemo(() => distinct(all.map((c) => c.part)), [all]);
  const sectionOptions = useMemo(() => {
    const here = part.trim();
    const inPart = here ? all.filter((c) => c.part?.trim() === here) : [];
    const pool = inPart.some((c) => c.section?.trim()) ? inPart : all;
    return distinct(pool.map((c) => c.section));
  }, [all, part]);

  const label = chapterLabel(chapter.displayNumber, index);
  const time = minutesLabel(chapter.estimatedMinutes);

  if (!authoring) {
    const crumbs = [chapter.part, chapter.section].map((c) => c?.trim()).filter(Boolean);
    return (
      <header className="mb-8 border-b border-line pb-6">
        {crumbs.length > 0 && (
          <nav aria-label="Course and module" className="mb-1.5">
            <ol className="flex flex-wrap items-center gap-x-1.5 text-xs text-ink-faint">
              {crumbs.map((crumb, i) => (
                <li key={i} className="flex items-center gap-1.5">
                  {i > 0 && <span aria-hidden="true">›</span>}
                  <span>{crumb}</span>
                </li>
              ))}
            </ol>
          </nav>
        )}
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
              queue({ displayNumber: e.target.value.trim() || null });
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
              if (e.target.value.trim()) queue({ title: e.target.value });
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
              queue({ estimatedMinutes: v ? Number(v) : null });
            }}
          />
        </div>
      </div>

      {/* Where the chapter sits in the sidebar: chapters sharing a Course and a
          Module are grouped there. Blank means none. */}
      <div className="mt-4 grid gap-4 md:grid-cols-2">
        <div>
          <label className="label" htmlFor="chapter-part">
            Course (part)
          </label>
          <input
            id="chapter-part"
            className="input"
            list="chapter-part-options"
            autoComplete="off"
            value={part}
            placeholder="1 · Ultimate Rust Crash Course"
            onChange={(e) => {
              setPart(e.target.value);
              queue({ part: e.target.value.trim() || null });
            }}
          />
        </div>
        <div>
          <label className="label" htmlFor="chapter-section">
            Module (section)
          </label>
          <input
            id="chapter-section"
            className="input"
            list="chapter-section-options"
            autoComplete="off"
            value={section}
            placeholder="Module 2: Fundamentals"
            onChange={(e) => {
              setSection(e.target.value);
              queue({ section: e.target.value.trim() || null });
            }}
          />
        </div>
        <datalist id="chapter-part-options">
          {partOptions.map((p) => (
            <option key={p} value={p} />
          ))}
        </datalist>
        <datalist id="chapter-section-options">
          {sectionOptions.map((s) => (
            <option key={s} value={s} />
          ))}
        </datalist>
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
                queue({ summaryMd: e.target.value });
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
                queue({ objectivesMd: e.target.value });
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
