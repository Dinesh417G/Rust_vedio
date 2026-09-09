import { useCallback, useEffect, useRef, useState } from "react";
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
import { useUi } from "@/store/ui";
import { chapterLabel, cx } from "@/lib/format";
import { StateDot } from "./StateDot";
import { ProgressBar } from "./ui/ProgressRing";
import { NewChapterDialog } from "./NewChapterDialog";
import { ConfirmDialog } from "./ui/ConfirmDialog";
import { KIND_LABEL } from "./blocks/common";

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
      const to = chapters.findIndex((c) => c.id === over.id);
      const without = chapters.filter((c) => c.id !== active.id);
      const anchor = to === 0 ? null : (without[to - 1]?.id ?? null);
      try {
        await ipc.chapterReorder(String(active.id), anchor);
      } catch (err) {
        toast({ tone: "error", message: ipc.asAppError(err).message });
      }
    },
    [chapters, toast],
  );

  if (collapsed) return null;

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
          <SortableContext
            items={chapters.map((c) => c.id)}
            strategy={verticalListSortingStrategy}
          >
            {chapters.map((chapter, i) => (
              <ChapterRow
                key={chapter.id}
                chapter={chapter}
                index={i}
                active={chapter.id === activeId}
                authoring={authoring}
                onOpen={() => navigate(`/chapter/${chapter.id}`)}
                onDelete={() => setPendingDelete(chapter)}
              />
            ))}
          </SortableContext>
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

function ChapterRow({
  chapter,
  index,
  active,
  authoring,
  onOpen,
  onDelete,
}: {
  chapter: ChapterSummary;
  index: number;
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

  return (
    <div
      ref={setNodeRef}
      style={{ transform: CSS.Translate.toString(transform), transition }}
      className={cx("px-1", isDragging && "opacity-60")}
    >
      <div
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
