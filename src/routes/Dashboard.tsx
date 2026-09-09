import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import {
  BookOpen,
  FileInput,
  Package,
  PlayCircle,
  Plus,
  Sparkles,
} from "lucide-react";
import { useChapters, useCourse, useCourseProgress } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { chapterLabel, cx, minutesLabel } from "@/lib/format";
import { Markdown } from "@/components/Markdown";
import { ProgressBar, ProgressRing } from "@/components/ui/ProgressRing";
import { StateDot } from "@/components/StateDot";
import { NewChapterDialog } from "@/components/NewChapterDialog";
import { OutlineImportDialog } from "@/components/OutlineImportDialog";
import { BundleImportDialog } from "@/components/BundleDialogs";

export function Dashboard() {
  const { data: course } = useCourse();
  const { data: chapters = [], isLoading } = useChapters();
  const { data: progress } = useCourseProgress();
  const navigate = useNavigate();
  const mode = useUi((s) => s.mode);
  const authoring = mode === "author";

  const [newOpen, setNewOpen] = useState(false);
  const [outlineOpen, setOutlineOpen] = useState(false);
  const [bundleOpen, setBundleOpen] = useState(false);

  const resume = progress?.lastChapterId
    ? chapters.find((c) => c.id === progress.lastChapterId)
    : null;

  const emptyBlocks = chapters.reduce((n, c) => n + c.emptyCount, 0);
  const missingBlocks = chapters.reduce((n, c) => n + c.missingCount, 0);

  return (
    <div className="h-full overflow-y-auto">
      <div className="mx-auto max-w-5xl px-8 py-10">
        <header className="mb-8 flex items-start gap-6">
          <div className="min-w-0 flex-1">
            <h1 className="text-3xl font-bold leading-tight text-ink">
              {course?.title ?? "RustClass"}
            </h1>
            {course?.subtitle && (
              <p className="mt-1 text-sm text-ink-muted">{course.subtitle}</p>
            )}
            {course?.author && (
              <p className="mt-1 text-xs text-ink-faint">by {course.author}</p>
            )}
            {course?.descriptionMd?.trim() && (
              <div className="mt-4 max-w-prose">
                <Markdown>{course.descriptionMd}</Markdown>
              </div>
            )}
          </div>
          <ProgressRing percent={progress?.percent ?? 0} size={64} stroke={5} />
        </header>

        {chapters.length === 0 && !isLoading ? (
          <FirstRun
            onNewChapter={() => setNewOpen(true)}
            onImportOutline={() => setOutlineOpen(true)}
            onImportBundle={() => setBundleOpen(true)}
          />
        ) : (
          <>
            <div className="mb-8 grid gap-3 sm:grid-cols-4">
              <Stat label="Chapters" value={String(chapters.length)} />
              <Stat
                label="Blocks complete"
                value={`${progress?.completedBlocks ?? 0} / ${progress?.totalBlocks ?? 0}`}
              />
              <Stat
                label="Still to fill in"
                value={String(emptyBlocks)}
                hint={emptyBlocks > 0 ? "planned, not yet written" : undefined}
              />
              <Stat
                label="Missing files"
                value={String(missingBlocks)}
                tone={missingBlocks > 0 ? "danger" : undefined}
                hint={missingBlocks > 0 ? "see Health" : undefined}
              />
            </div>

            {resume && (
              <button
                onClick={() => navigate(`/chapter/${resume.id}`)}
                className="mb-8 flex w-full items-center gap-4 rounded-xl border border-brand/40 bg-brand/10 p-4 text-left transition-colors hover:bg-brand/15"
              >
                <PlayCircle size={28} className="shrink-0 text-brand" />
                <span className="min-w-0 flex-1">
                  <span className="block text-xs font-semibold uppercase tracking-wide text-brand">
                    Continue where you left off
                  </span>
                  <span className="block truncate text-base font-medium text-ink">
                    {chapterLabel(resume.displayNumber, chapters.indexOf(resume))}.{" "}
                    {resume.title}
                  </span>
                </span>
              </button>
            )}

            <div className="mb-3 flex items-center justify-between">
              <h2 className="text-sm font-semibold uppercase tracking-wide text-ink-faint">
                Chapters
              </h2>
              {authoring && (
                <div className="flex gap-2">
                  <button className="btn btn-sm" onClick={() => setOutlineOpen(true)}>
                    <FileInput size={13} /> Import outline
                  </button>
                  <button className="btn btn-sm btn-primary" onClick={() => setNewOpen(true)}>
                    <Plus size={13} /> New chapter
                  </button>
                </div>
              )}
            </div>

            <ul className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
              {chapters.map((c, i) => {
                const percent =
                  c.blockCount > 0 ? (c.completedCount / c.blockCount) * 100 : 0;
                const time = minutesLabel(c.estimatedMinutes);
                return (
                  <li key={c.id}>
                    <Link
                      to={`/chapter/${c.id}`}
                      className="flex h-full flex-col rounded-xl border border-line bg-surface-1 p-4 transition-colors hover:border-brand/60"
                    >
                      <span className="mb-1 font-mono text-xs text-brand">
                        {chapterLabel(c.displayNumber, i)}
                      </span>
                      <span className="mb-1 line-clamp-2 text-base font-medium leading-snug text-ink">
                        {c.title}
                      </span>
                      {c.summaryMd?.trim() && (
                        <span className="mb-3 line-clamp-2 text-xs text-ink-muted">
                          {c.summaryMd}
                        </span>
                      )}
                      <span className="mt-auto space-y-2">
                        <ProgressBar percent={percent} />
                        <span className="flex flex-wrap items-center gap-2 text-[11px] text-ink-faint">
                          <span className="inline-flex items-center gap-1">
                            <StateDot state="ready" /> {c.readyCount}
                          </span>
                          <span className="inline-flex items-center gap-1">
                            <StateDot state="empty" /> {c.emptyCount}
                          </span>
                          {c.missingCount > 0 && (
                            <span className="inline-flex items-center gap-1 text-danger">
                              <StateDot state="missing" /> {c.missingCount}
                            </span>
                          )}
                          {time && <span className="ml-auto">{time}</span>}
                        </span>
                      </span>
                    </Link>
                  </li>
                );
              })}
            </ul>
          </>
        )}
      </div>

      <NewChapterDialog open={newOpen} onClose={() => setNewOpen(false)} />
      <OutlineImportDialog open={outlineOpen} onClose={() => setOutlineOpen(false)} />
      <BundleImportDialog open={bundleOpen} onClose={() => setBundleOpen(false)} />
    </div>
  );
}

function Stat({
  label,
  value,
  hint,
  tone,
}: {
  label: string;
  value: string;
  hint?: string;
  tone?: "danger";
}) {
  return (
    <div className="rounded-xl border border-line bg-surface-1 p-4">
      <p className="text-xs uppercase tracking-wide text-ink-faint">{label}</p>
      <p
        className={cx(
          "mt-1 text-2xl font-semibold tabular-nums",
          tone === "danger" ? "text-danger" : "text-ink",
        )}
      >
        {value}
      </p>
      {hint && <p className="text-[11px] text-ink-faint">{hint}</p>}
    </div>
  );
}

function FirstRun({
  onNewChapter,
  onImportOutline,
  onImportBundle,
}: {
  onNewChapter: () => void;
  onImportOutline: () => void;
  onImportBundle: () => void;
}) {
  return (
    <div className="rounded-xl border-2 border-dashed border-line px-8 py-14 text-center">
      <Sparkles size={32} className="mx-auto mb-3 text-brand" />
      <h2 className="text-lg font-semibold text-ink">Start your course</h2>
      <p className="mx-auto mt-2 max-w-md text-sm text-ink-muted">
        Plan the whole thing first. Chapters, headings and descriptions can all exist
        long before a single video is recorded.
      </p>
      <div className="mt-6 flex flex-wrap justify-center gap-2">
        <button className="btn btn-primary" onClick={onNewChapter}>
          <Plus size={14} /> Create first chapter
        </button>
        <button className="btn" onClick={onImportOutline}>
          <FileInput size={14} /> Import outline
        </button>
        <button className="btn" onClick={onImportBundle}>
          <Package size={14} /> Import bundle
        </button>
      </div>
      <p className="mt-6 inline-flex items-center gap-1.5 text-xs text-ink-faint">
        <BookOpen size={12} /> Everything is stored locally. Nothing leaves this machine.
      </p>
    </div>
  );
}
