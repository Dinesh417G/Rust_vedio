import { Fragment, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import {
  DndContext,
  KeyboardSensor,
  PointerSensor,
  closestCenter,
  useSensor,
  useSensors,
  type DragEndEvent,
} from "@dnd-kit/core";
import { restrictToVerticalAxis } from "@dnd-kit/modifiers";
import {
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import {
  ChevronDown,
  ChevronRight,
  Copy,
  FileInput,
  GripVertical,
  Plus,
  Trash2,
} from "lucide-react";
import * as ipc from "@/lib/ipc";
import type { BlockView, ChapterSummary } from "@/lib/types";
import { useChapter, useChapters } from "@/lib/queries";
import { partKey, sectionKey, useUi } from "@/store/ui";
import { chapterLabel, cx } from "@/lib/format";
import { StateDot } from "./StateDot";
import { ProgressBar } from "./ui/ProgressRing";
import { NewChapterDialog } from "./NewChapterDialog";
import { ConfirmDialog } from "./ui/ConfirmDialog";
import { KIND_LABEL } from "./blocks/common";

// --- the outline: Course (part) > Module (section) > Lesson (chapter) -------

/** A chapter and its place in the whole list, which names it when it has no number. */
interface Lesson {
  chapter: ChapterSummary;
  index: number;
}

interface Tally {
  chapters: number;
  blocks: number;
  done: number;
}

/** Consecutive lessons of one module — or of a course that has no module. */
interface SectionGroup {
  title: string | null;
  /** `null` when the lessons hang straight off the course. */
  key: string | null;
  lessons: Lesson[];
  tally: Tally;
}

interface PartGroup {
  kind: "part";
  title: string;
  key: string;
  sections: SectionGroup[];
  tally: Tally;
}

/** Chapters with no course render as a plain list, exactly as before grouping existed. */
interface FlatRun {
  kind: "flat";
  lessons: Lesson[];
}

type OutlineNode = PartGroup | FlatRun;

const trimmed = (s: string | null | undefined) => s?.trim() || null;

/**
 * Groups *consecutive* chapters. The list is ordered by `sortIndex` alone and
 * nothing keeps a course contiguous, so a course that is split by another shows
 * up as two groups — which is also where its chapters really are.
 */
function buildOutline(chapters: ChapterSummary[]): OutlineNode[] {
  const nodes: OutlineNode[] = [];
  let part: PartGroup | null = null;
  let section: SectionGroup | null = null;

  for (let index = 0; index < chapters.length; index++) {
    const chapter = chapters[index];
    const lesson: Lesson = { chapter, index };
    const partTitle = trimmed(chapter.part);

    if (!partTitle) {
      const last = nodes[nodes.length - 1];
      if (last?.kind === "flat") last.lessons.push(lesson);
      else nodes.push({ kind: "flat", lessons: [lesson] });
      part = null;
      section = null;
      continue;
    }

    if (!part || part.title !== partTitle) {
      part = {
        kind: "part",
        title: partTitle,
        key: partKey(partTitle),
        sections: [],
        tally: { chapters: 0, blocks: 0, done: 0 },
      };
      nodes.push(part);
      section = null;
    }

    const sectionTitle = trimmed(chapter.section);
    if (!section || section.title !== sectionTitle) {
      section = {
        title: sectionTitle,
        key: sectionTitle ? sectionKey(partTitle, sectionTitle) : null,
        lessons: [],
        tally: { chapters: 0, blocks: 0, done: 0 },
      };
      part.sections.push(section);
    }

    section.lessons.push(lesson);
    for (const t of [section.tally, part.tally]) {
      t.chapters += 1;
      t.blocks += chapter.blockCount;
      t.done += chapter.completedCount;
    }
  }
  return nodes;
}

/**
 * Where a chapter sits, for deciding whether a drag stays inside its group. A
 * module only means something under a course, so it is ignored without one.
 */
function groupOf(c: ChapterSummary): string {
  const part = trimmed(c.part);
  return part ? `${part}\u0001${trimmed(c.section) ?? ""}` : "";
}

const hasChapter = (lessons: Lesson[], id: string | undefined) =>
  !!id && lessons.some((l) => l.chapter.id === id);

/** The left gutter for each depth; whole strings so Tailwind can see them. */
const INDENT = ["px-1", "pl-4 pr-1", "pl-7 pr-1"] as const;
type Indent = 0 | 1 | 2;

export function Sidebar({ onImportOutline }: { onImportOutline: () => void }) {
  const { data: chapters = [], isLoading } = useChapters();
  const navigate = useNavigate();
  const { id: activeId } = useParams<{ id: string }>();

  const width = useUi((s) => s.sidebarWidth);
  const setWidth = useUi((s) => s.setSidebarWidth);
  const collapsed = useUi((s) => s.sidebarCollapsed);
  const mode = useUi((s) => s.mode);
  const authoring = mode === "author";
  const toast = useUi((s) => s.toast);
  const openGroups = useUi((s) => s.openGroups);
  const toggleGroup = useUi((s) => s.toggleGroup);

  const outline = useMemo(() => buildOutline(chapters), [chapters]);
  const byId = useMemo(() => new Map(chapters.map((c) => [c.id, c])), [chapters]);

  const [newOpen, setNewOpen] = useState(false);
  const [pendingDelete, setPendingDelete] = useState<ChapterSummary | null>(null);
  const resizing = useRef(false);

  // --- resize handle -------------------------------------------------------

  useEffect(() => {
    const onMove = (e: MouseEvent) => {
      if (resizing.current) setWidth(e.clientX);
    };
    const onUp = () => {
      resizing.current = false;
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
    };
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
    return () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
  }, [setWidth]);

  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 4 } }),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates }),
  );

  const onDragEnd = useCallback(
    async (e: DragEndEvent) => {
      const { active, over } = e;
      if (!over || active.id === over.id) return;
      const dragged = byId.get(String(active.id));
      const target = byId.get(String(over.id));
      if (!dragged || !target) return;

      // Order is only meaningful inside a module. Moving a chapter to another
      // course or module is a change to what it *is*, and that lives in its header.
      if (groupOf(dragged) !== groupOf(target)) {
        toast({
          tone: "info",
          message: `"${dragged.title}" can only be reordered inside its own course and module. To move it elsewhere, change its Course or Module in the chapter header.`,
          action: { label: "Open chapter", run: () => navigate(`/chapter/${dragged.id}`) },
        });
        return;
      }

      const to = chapters.findIndex((c) => c.id === over.id);
      const without = chapters.filter((c) => c.id !== active.id);
      const anchor = to === 0 ? null : (without[to - 1]?.id ?? null);
      try {
        await ipc.chapterReorder(String(active.id), anchor);
      } catch (err) {
        toast({ tone: "error", message: ipc.asAppError(err).message });
      }
    },
    [byId, chapters, navigate, toast],
  );

  if (collapsed) return null;

  /** One sortable list per run, so a drag shuffles only the rows it can land among. */
  const renderLessons = (lessons: Lesson[], indent: Indent) => (
    <SortableContext
      items={lessons.map((l) => l.chapter.id)}
      strategy={verticalListSortingStrategy}
    >
      {lessons.map(({ chapter, index }) => (
        <ChapterRow
          key={chapter.id}
          chapter={chapter}
          index={index}
          indent={indent}
          active={chapter.id === activeId}
          authoring={authoring}
          onOpen={() => navigate(`/chapter/${chapter.id}`)}
          onDelete={() => setPendingDelete(chapter)}
        />
      ))}
    </SortableContext>
  );

  // Only the rows of open groups are rendered, so ~300 chapters cost a few dozen rows.
  const renderPart = (part: PartGroup, at: number) => {
    const partOpen = openGroups.has(part.key);
    return (
      <div key={`${part.key}#${at}`} className={cx(at > 0 && "mt-1 border-t border-line pt-1")}>
        <GroupHeader
          level={0}
          title={part.title}
          tally={part.tally}
          open={partOpen}
          current={part.sections.some((s) => hasChapter(s.lessons, activeId))}
          onToggle={() => toggleGroup(part.key)}
        />
        {partOpen && (
          <div role="group" aria-label={part.title}>
            {part.sections.map((section, i) => {
              if (section.key === null) {
                return (
                  <Fragment key={`none#${i}`}>{renderLessons(section.lessons, 1)}</Fragment>
                );
              }
              const sectionOpen = openGroups.has(section.key);
              return (
                <div key={`${section.key}#${i}`}>
                  <GroupHeader
                    level={1}
                    title={section.title ?? ""}
                    tally={section.tally}
                    open={sectionOpen}
                    current={hasChapter(section.lessons, activeId)}
                    onToggle={() => section.key && toggleGroup(section.key)}
                  />
                  {sectionOpen && (
                    <div role="group" aria-label={section.title ?? undefined}>
                      {renderLessons(section.lessons, 2)}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>
    );
  };

  return (
    <aside
      className="relative flex shrink-0 flex-col border-r border-line bg-surface-1"
      style={{ width }}
    >
      <div className="flex items-center justify-between gap-2 border-b border-line px-3 py-2">
        <h2 className="text-xs font-semibold uppercase tracking-wide text-ink-faint">
          Course outline
        </h2>
        {authoring && (
          <div className="flex gap-1">
            <button
              className="btn btn-ghost btn-sm"
              title="Import an outline from markdown"
              onClick={onImportOutline}
            >
              <FileInput size={14} />
            </button>
            <button
              className="btn btn-ghost btn-sm"
              title="New chapter"
              onClick={() => setNewOpen(true)}
            >
              <Plus size={14} />
            </button>
          </div>
        )}
      </div>

      <nav className="min-h-0 flex-1 overflow-y-auto py-1">
        {isLoading && <p className="px-3 py-2 text-sm text-ink-muted">Loading…</p>}
        {!isLoading && chapters.length === 0 && (
          <div className="px-3 py-6 text-center">
            <p className="text-sm text-ink-muted">No chapters yet.</p>
            {authoring && (
              <button className="btn btn-sm mt-3" onClick={() => setNewOpen(true)}>
                <Plus size={13} /> New chapter
              </button>
            )}
          </div>
        )}

        <DndContext
          sensors={sensors}
          collisionDetection={closestCenter}
          modifiers={[restrictToVerticalAxis]}
          onDragEnd={(e) => void onDragEnd(e)}
        >
          {outline.map((node, at) =>
            node.kind === "flat" ? (
              <Fragment key={`flat#${at}`}>{renderLessons(node.lessons, 0)}</Fragment>
            ) : (
              renderPart(node, at)
            ),
          )}
        </DndContext>
      </nav>

      {/* Resize handle */}
      <div
        role="separator"
        aria-orientation="vertical"
        aria-label="Resize sidebar"
        tabIndex={0}
        onMouseDown={() => {
          resizing.current = true;
          document.body.style.cursor = "col-resize";
          document.body.style.userSelect = "none";
        }}
        onKeyDown={(e) => {
          if (e.key === "ArrowLeft") setWidth(width - 16);
          if (e.key === "ArrowRight") setWidth(width + 16);
        }}
        className="absolute inset-y-0 -right-1 w-2 cursor-col-resize hover:bg-brand/30"
      />

      <NewChapterDialog open={newOpen} onClose={() => setNewOpen(false)} />

      <ConfirmDialog
        open={!!pendingDelete}
        title={`Delete "${pendingDelete?.title ?? ""}"?`}
        consequence={`The chapter and its ${pendingDelete?.blockCount ?? 0} block${
          pendingDelete?.blockCount === 1 ? "" : "s"
        } move to the Trash for 30 days. No video, image or attached file is deleted from disk.`}
        confirmLabel="Move to Trash"
        onCancel={() => setPendingDelete(null)}
        onConfirm={async () => {
          const target = pendingDelete;
          setPendingDelete(null);
          if (!target) return;
          try {
            await ipc.chapterDelete(target.id);
            if (target.id === activeId) navigate("/");
            toast({
              tone: "info",
              message: `"${target.title}" moved to the Trash.`,
              action: {
                label: "Undo",
                run: () => void ipc.chapterRestore(target.id).catch(() => undefined),
              },
            });
          } catch (e) {
            toast({ tone: "error", message: ipc.asAppError(e).message });
          }
        }}
      />
    </aside>
  );
}

/**
 * A course or module header: a button that opens and closes its group, with
 * the group's progress (blocks complete over blocks, summed across its chapters)
 * and its chapter count underneath.
 */
function GroupHeader({
  level,
  title,
  tally,
  open,
  current,
  onToggle,
}: {
  level: 0 | 1;
  title: string;
  tally: Tally;
  open: boolean;
  /** The chapter on screen is somewhere inside this group. */
  current: boolean;
  onToggle: () => void;
}) {
  const percent = tally.blocks > 0 ? (tally.done / tally.blocks) * 100 : 0;
  const count = `${tally.chapters} chapter${tally.chapters === 1 ? "" : "s"}`;
  return (
    <button
      type="button"
      aria-expanded={open}
      aria-label={`${title}: ${tally.done} of ${tally.blocks} blocks complete, ${count}`}
      title={title}
      onClick={onToggle}
      className={cx(
        "flex w-full items-start gap-1.5 rounded-lg py-1.5 pr-2 text-left transition-colors hover:bg-surface-2",
        level === 0 ? "pl-2" : "pl-5",
      )}
    >
      <span className="mt-0.5 shrink-0 text-ink-faint">
        {open ? <ChevronDown size={13} /> : <ChevronRight size={13} />}
      </span>
      <span className="min-w-0 flex-1">
        <span
          className={cx(
            "block truncate",
            level === 0 ? "text-[13px] font-semibold" : "text-xs font-medium",
            current ? "text-brand" : level === 0 ? "text-ink" : "text-ink-muted",
          )}
        >
          {title}
        </span>
        <span className="mt-1 flex items-center gap-2">
          <ProgressBar percent={percent} className="h-0.5 flex-1" />
          <span className="shrink-0 text-[10px] tabular-nums text-ink-faint">
            {tally.done}/{tally.blocks}
          </span>
          <span
            className="shrink-0 rounded-full bg-surface-3 px-1.5 text-[10px] tabular-nums text-ink-muted"
            title={count}
          >
            {tally.chapters}
          </span>
        </span>
      </span>
    </button>
  );
}

function ChapterRow({
  chapter,
  index,
  indent,
  active,
  authoring,
  onOpen,
  onDelete,
}: {
  chapter: ChapterSummary;
  index: number;
  indent: Indent;
  active: boolean;
  authoring: boolean;
  onOpen: () => void;
  onDelete: () => void;
}) {
  const expanded = useUi((s) => s.expandedChapters.has(chapter.id));
  const toggle = useUi((s) => s.toggleChapter);
  const toast = useUi((s) => s.toast);
  const {
    attributes,
    listeners,
    setNodeRef,
    setActivatorNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: chapter.id, disabled: !authoring });

  const percent =
    chapter.blockCount > 0 ? (chapter.completedCount / chapter.blockCount) * 100 : 0;

  // Opening a chapter unfolds its groups, which can leave its row below the
  // fold of a long module; bring it into view once it exists.
  const row = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (active) row.current?.scrollIntoView({ block: "nearest" });
  }, [active]);

  return (
    <div
      ref={setNodeRef}
      style={{ transform: CSS.Translate.toString(transform), transition }}
      className={cx(INDENT[indent], isDragging && "opacity-60")}
    >
      <div
        ref={row}
        className={cx(
          "group flex items-center gap-1 rounded-lg px-1.5 py-1.5 transition-colors",
          active ? "bg-brand/15" : "hover:bg-surface-2",
        )}
      >
        <button
          className="shrink-0 rounded p-0.5 text-ink-faint hover:text-ink"
          onClick={() => toggle(chapter.id)}
          aria-label={expanded ? "Collapse chapter" : "Expand chapter"}
          aria-expanded={expanded}
        >
          {expanded ? <ChevronDown size={13} /> : <ChevronRight size={13} />}
        </button>

        <button className="min-w-0 flex-1 text-left" onClick={onOpen}>
          <span className="flex items-baseline gap-1.5">
            <span
              className={cx(
                "shrink-0 font-mono text-[11px]",
                active ? "text-brand" : "text-ink-faint",
              )}
            >
              {chapterLabel(chapter.displayNumber, index)}
            </span>
            <span
              className={cx(
                "truncate text-sm",
                active ? "font-medium text-ink" : "text-ink",
              )}
            >
              {chapter.title}
            </span>
          </span>
          <span className="mt-1 flex items-center gap-2">
            <ProgressBar percent={percent} className="h-0.5 flex-1" />
            <span className="shrink-0 text-[10px] tabular-nums text-ink-faint">
              {chapter.completedCount}/{chapter.blockCount}
            </span>
            {chapter.missingCount > 0 && (
              <StateDot state="missing" className="shrink-0" />
            )}
          </span>
        </button>

        {authoring && (
          <div className="flex shrink-0 opacity-0 transition-opacity focus-within:opacity-100 group-hover:opacity-100">
            <button
              ref={setActivatorNodeRef}
              {...attributes}
              {...listeners}
              className="cursor-grab rounded p-1 text-ink-faint hover:text-ink active:cursor-grabbing"
              title="Drag to reorder"
              aria-label="Drag to reorder chapter"
            >
              <GripVertical size={13} />
            </button>
            <button
              className="rounded p-1 text-ink-faint hover:text-ink"
              title="Duplicate chapter (structure only)"
              onClick={async () => {
                try {
                  await ipc.chapterDuplicate(chapter.id);
                } catch (e) {
                  toast({ tone: "error", message: ipc.asAppError(e).message });
                }
              }}
            >
              <Copy size={13} />
            </button>
            <button
              className="rounded p-1 text-ink-faint hover:text-danger"
              title="Delete chapter"
              onClick={onDelete}
            >
              <Trash2 size={13} />
            </button>
          </div>
        )}
      </div>

      {expanded && <BlockChips chapterId={chapter.id} />}
    </div>
  );
}

/** The per-block state dots under an expanded chapter. */
function BlockChips({ chapterId }: { chapterId: string }) {
  const { data: chapter, isLoading } = useChapter(chapterId);
  const navigate = useNavigate();
  const flashBlock = useUi((s) => s.flashBlock);
  const selectBlock = useUi((s) => s.selectBlock);

  if (isLoading) {
    return <p className="py-1 pl-7 text-xs text-ink-faint">Loading…</p>;
  }
  const blocks: BlockView[] = chapter?.blocks ?? [];
  if (blocks.length === 0) {
    return <p className="py-1 pl-7 text-xs text-ink-faint">No blocks yet</p>;
  }

  return (
    <ul className="mb-1 space-y-0.5 pl-7">
      {blocks.map((b) => (
        <li key={b.id}>
          <button
            className="flex w-full items-center gap-1.5 rounded px-1.5 py-1 text-left hover:bg-surface-2"
            onClick={() => {
              navigate(`/chapter/${chapterId}`);
              selectBlock(b.id);
              flashBlock(b.id);
            }}
          >
            <StateDot state={b.state} />
            <span className="truncate text-xs text-ink-muted">
              {b.heading?.trim() || KIND_LABEL[b.kind]}
            </span>
          </button>
        </li>
      ))}
    </ul>
  );
}
