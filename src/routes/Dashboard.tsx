import { useMemo, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import {
  ArrowRight,
  BookOpen,
  FileInput,
  Library,
  Package,
  PlayCircle,
  Plus,
  Sparkles,
} from "lucide-react";
import * as ipc from "@/lib/ipc";
import type { ChapterSummary } from "@/lib/types";
import { useChapters, useCourse, useCourseProgress, useLibraryLoad } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { chapterLabel, cx, minutesLabel } from "@/lib/format";
import { Markdown } from "@/components/Markdown";
import { ProgressBar, ProgressRing } from "@/components/ui/ProgressRing";
import { ConfirmDialog } from "@/components/ui/ConfirmDialog";
import { StateDot } from "@/components/StateDot";
import { NewChapterDialog } from "@/components/NewChapterDialog";
import { OutlineImportDialog } from "@/components/OutlineImportDialog";
import { BundleImportDialog } from "@/components/BundleDialogs";

// --- grouping: Course (part) > Module (section) > Topic (chapter) ------------

interface Tally {
  chapters: number;
  blocks: number;
  done: number;
}

interface ModuleSummary {
  /** `null` for the topics of a course that sit under no module. */
  title: string | null;
  chapters: ChapterSummary[];
  tally: Tally;
}

interface CourseSummary {
  title: string;
  chapters: ChapterSummary[];
  modules: ModuleSummary[];
  tally: Tally;
}

const noTally = (): Tally => ({ chapters: 0, blocks: 0, done: 0 });

function add(t: Tally, c: ChapterSummary) {
  t.chapters += 1;
  t.blocks += c.blockCount;
  t.done += c.completedCount;
}

const percentOf = (t: Tally) => (t.blocks > 0 ? (t.done / t.blocks) * 100 : 0);
const topics = (n: number) => `${n} topic${n === 1 ? "" : "s"}`;
const count = (n: number, noun: string) => `${n} ${noun}${n === 1 ? "" : "s"}`;

/**
 * One summary per course title, in the order each first appears, whatever else
 * sits between its chapters. Chapters with no course stay loose, with their place
 * in the whole list so an unnumbered one is still labelled by its position.
 */
function summarise(chapters: ChapterSummary[]) {
  const courses = new Map<string, CourseSummary>();
  const loose: Array<{ chapter: ChapterSummary; index: number }> = [];

  chapters.forEach((chapter, index) => {
    const title = chapter.part?.trim();
    if (!title) {
      loose.push({ chapter, index });
      return;
    }
    let course = courses.get(title);
    if (!course) {
      course = { title, chapters: [], modules: [], tally: noTally() };
      courses.set(title, course);
    }
    const moduleTitle = chapter.section?.trim() || null;
    let module = course.modules.find((m) => m.title === moduleTitle);
    if (!module) {
      module = { title: moduleTitle, chapters: [], tally: noTally() };
      course.modules.push(module);
    }
    course.chapters.push(chapter);
    module.chapters.push(chapter);
    add(course.tally, chapter);
    add(module.tally, chapter);
  });

  return { courses: [...courses.values()], loose };
}

/**
 * Where "Continue" leads: the first chapter that has blocks and is not finished.
 * A chapter with no blocks cannot be completed, so stopping on one would strand
 * the link there.
 */
function nextUp(chapters: ChapterSummary[]): { chapter: ChapterSummary; label: string } | null {
  const open = chapters.find((c) => c.blockCount > 0 && c.completedCount < c.blockCount);
  if (open) return { chapter: open, label: "Continue" };
  const first = chapters[0];
  if (!first) return null;
  return { chapter: first, label: chapters.some((c) => c.blockCount > 0) ? "Review" : "Open" };
}

export function Dashboard() {
  const { data: course } = useCourse();
  const { data: chapters = [], isLoading } = useChapters();
  const { data: progress } = useCourseProgress();
  const navigate = useNavigate();
  const mode = useUi((s) => s.mode);
  const toast = useUi((s) => s.toast);
  const authoring = mode === "author";
  const loadLibrary = useLibraryLoad();

  const [newOpen, setNewOpen] = useState(false);
  const [outlineOpen, setOutlineOpen] = useState(false);
  const [bundleOpen, setBundleOpen] = useState(false);
  const [loadOpen, setLoadOpen] = useState(false);

  const resume = progress?.lastChapterId
    ? chapters.find((c) => c.id === progress.lastChapterId)
    : null;

  const emptyBlocks = chapters.reduce((n, c) => n + c.emptyCount, 0);
  const missingBlocks = chapters.reduce((n, c) => n + c.missingCount, 0);
  const { courses, loose } = useMemo(() => summarise(chapters), [chapters]);
  const grouped = courses.length > 0;

  const runLoad = async () => {
    setLoadOpen(false);
    if (loadLibrary.isPending) return;
    try {
      const r = await loadLibrary.mutateAsync("replace");
      toast({
        tone: "success",
        message: `Loaded the Ultimate Rust library: ${count(r.chaptersCreated, "chapter")} and ${count(
          r.blocksCreated,
          "block",
        )} from ${count(r.files, "file")}.${
          r.chaptersRemoved > 0
            ? ` ${count(r.chaptersRemoved, "previous chapter")} moved to the Trash.`
            : ""
        }${r.warnings.length > 0 ? ` ${count(r.warnings.length, "warning")}.` : ""}`,
      });
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

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
            busy={loadLibrary.isPending}
            onNewChapter={() => setNewOpen(true)}
            onImportOutline={() => setOutlineOpen(true)}
            onImportBundle={() => setBundleOpen(true)}
            onLoadLibrary={() => setLoadOpen(true)}
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
                {grouped ? "Courses" : "Chapters"}
              </h2>
              {authoring && (
                <div className="flex gap-2">
                  <button className="btn btn-sm" onClick={() => setOutlineOpen(true)}>
                    <FileInput size={13} /> Import outline
                  </button>
                  <button
                    className="btn btn-sm"
                    disabled={loadLibrary.isPending}
                    onClick={() => setLoadOpen(true)}
                  >
                    <Library size={13} /> {loadLibrary.isPending ? "Loading…" : "Load library…"}
                  </button>
                  <button className="btn btn-sm btn-primary" onClick={() => setNewOpen(true)}>
                    <Plus size={13} /> New chapter
                  </button>
                </div>
              )}
            </div>

            {grouped && (
              <div className="mb-8 space-y-4">
                {courses.map((c) => (
                  <CourseCard key={c.title} course={c} />
                ))}
              </div>
            )}

            {grouped && loose.length > 0 && (
              <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-ink-faint">
                Other chapters
              </h2>
            )}
            {loose.length > 0 && (
              <ul className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                {loose.map(({ chapter, index }) => (
                  <ChapterCard key={chapter.id} chapter={chapter} index={index} />
                ))}
              </ul>
            )}
          </>
        )}
      </div>

      <NewChapterDialog open={newOpen} onClose={() => setNewOpen(false)} />
      <OutlineImportDialog open={outlineOpen} onClose={() => setOutlineOpen(false)} />
      <BundleImportDialog open={bundleOpen} onClose={() => setBundleOpen(false)} />
      <ConfirmDialog
        open={loadOpen}
        title="Load the Ultimate Rust library?"
        consequence="Every current chapter moves to the Trash for 30 days; attached videos stay on disk."
        confirmLabel="Load the library"
        destructive={chapters.length > 0}
        onCancel={() => setLoadOpen(false)}
        onConfirm={() => void runLoad()}
      >
        <p className="text-sm text-ink-muted">
          The library arrives as chapters grouped Course → Module → Topic, ready to read
          and to tick off as you go.
        </p>
      </ConfirmDialog>
    </div>
  );
}

/** One course: its progress, a way back in, and one row per module. */
function CourseCard({ course }: { course: CourseSummary }) {
  const next = nextUp(course.chapters);
  return (
    <section
      aria-label={course.title}
      className="overflow-hidden rounded-xl border border-line bg-surface-1"
    >
      <div className="flex items-start gap-4 p-4">
        <div className="min-w-0 flex-1">
          <h3 className="text-base font-semibold leading-snug text-ink">{course.title}</h3>
          <p className="mt-0.5 text-xs tabular-nums text-ink-faint">
            {topics(course.tally.chapters)} · {course.tally.done}/{course.tally.blocks} blocks
            complete
          </p>
          <ProgressBar percent={percentOf(course.tally)} className="mt-3" />
        </div>
        {next && (
          <Link
            to={`/chapter/${next.chapter.id}`}
            aria-label={`${next.label}: ${next.chapter.title}`}
            title={next.chapter.title}
            className="btn btn-primary shrink-0"
          >
            {next.label} <ArrowRight size={14} />
          </Link>
        )}
      </div>

      <ul className="divide-y divide-line border-t border-line">
        {course.modules.map((m, i) => (
          <li key={`${m.title ?? ""}#${i}`}>
            <Link
              to={`/chapter/${m.chapters[0].id}`}
              className="flex items-center gap-4 px-4 py-2.5 transition-colors hover:bg-surface-2"
            >
              <span className="min-w-0 flex-1 truncate text-sm text-ink">
                {m.title ?? "Other topics"}
              </span>
              <span className="shrink-0 text-xs text-ink-faint">{topics(m.tally.chapters)}</span>
              <div className="w-24 shrink-0">
                <ProgressBar percent={percentOf(m.tally)} />
              </div>
              <span className="w-14 shrink-0 text-right text-xs tabular-nums text-ink-faint">
                {m.tally.done}/{m.tally.blocks}
              </span>
            </Link>
          </li>
        ))}
      </ul>
    </section>
  );
}

function ChapterCard({ chapter: c, index }: { chapter: ChapterSummary; index: number }) {
  const percent = c.blockCount > 0 ? (c.completedCount / c.blockCount) * 100 : 0;
  const time = minutesLabel(c.estimatedMinutes);
  return (
    <li>
      <Link
        to={`/chapter/${c.id}`}
        className="flex h-full flex-col rounded-xl border border-line bg-surface-1 p-4 transition-colors hover:border-brand/60"
      >
        <span className="mb-1 font-mono text-xs text-brand">
          {chapterLabel(c.displayNumber, index)}
        </span>
        <span className="mb-1 line-clamp-2 text-base font-medium leading-snug text-ink">
          {c.title}
        </span>
        {c.summaryMd?.trim() && (
          <span className="mb-3 line-clamp-2 text-xs text-ink-muted">{c.summaryMd}</span>
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
  busy,
  onNewChapter,
  onImportOutline,
  onImportBundle,
  onLoadLibrary,
}: {
  busy: boolean;
  onNewChapter: () => void;
  onImportOutline: () => void;
  onImportBundle: () => void;
  onLoadLibrary: () => void;
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
        <button className="btn btn-primary" disabled={busy} onClick={onLoadLibrary}>
          <Library size={14} /> {busy ? "Loading the library…" : "Load the Ultimate Rust library"}
        </button>
        <button className="btn" onClick={onNewChapter}>
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
