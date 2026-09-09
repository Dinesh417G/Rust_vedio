import { useCallback, useEffect, useMemo, useRef, useState } from "react";
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
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { ArrowLeft, ArrowRight, Check, CircleCheck } from "lucide-react";
import * as ipc from "@/lib/ipc";
import type { BlockKind, BlockPatch, BlockView, ChapterPatch } from "@/lib/types";
import {
  useChapter,
  useChapterComplete,
  useChapters,
  useChapterUpdate,
} from "@/lib/queries";
import { useUi } from "@/store/ui";
import { useImports } from "@/store/imports";
import { assetKindFor, usePickAndImport, useRelink, useStartImport } from "@/lib/useImport";
import { blockIdAtPoint, useNativeDrop } from "@/lib/useNativeDrop";
import { cx } from "@/lib/format";
import { BlockRow } from "@/components/BlockRow";
import { InsertBar } from "@/components/InsertBar";
import { ChapterHeader } from "@/components/ChapterHeader";
import { ChapterNotes } from "@/components/ChapterNotes";
import { Modal } from "@/components/ui/Modal";
import { InlineMarkdown } from "@/components/Markdown";

export function ChapterPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data: chapter, isLoading, error } = useChapter(id);
  const { data: chapters = [] } = useChapters();

  const mode = useUi((s) => s.mode);
  const authoring = mode === "author";
  const setMode = useUi((s) => s.setMode);
  const selectedBlockId = useUi((s) => s.selectedBlockId);
  const selectBlock = useUi((s) => s.selectBlock);
  const flashBlockId = useUi((s) => s.flashBlockId);
  const expandChapter = useUi((s) => s.expandChapter);
  const toast = useUi((s) => s.toast);

  const updateChapter = useChapterUpdate();
  const completeChapter = useChapterComplete();
  const pickAndImport = usePickAndImport();
  const startImport = useStartImport();
  const relink = useRelink();
  const cancelTicketFor = useImports((s) => s.forBlock);

  const [dropTargetId, setDropTargetId] = useState<string | null>(null);
  const [emptyModal, setEmptyModal] = useState<BlockView | null>(null);
  const scroller = useRef<HTMLDivElement>(null);

  const blocks = useMemo(() => chapter?.blocks ?? [], [chapter]);
  const index = chapters.findIndex((c) => c.id === id);
  const prev = index > 0 ? chapters[index - 1] : null;
  const next = index >= 0 && index < chapters.length - 1 ? chapters[index + 1] : null;
  const summary = chapters.find((c) => c.id === id);
  const percent =
    summary && summary.blockCount > 0
      ? (summary.completedCount / summary.blockCount) * 100
      : 0;

  useEffect(() => {
    if (id) expandChapter(id);
  }, [id, expandChapter]);

  // Scroll the flashed block into view after a search jump.
  useEffect(() => {
    if (!flashBlockId) return;
    document
      .getElementById(`block-${flashBlockId}`)
      ?.scrollIntoView({ behavior: "smooth", block: "center" });
  }, [flashBlockId, blocks.length]);

  // --- native file drop ----------------------------------------------------

  const onDropFiles = useCallback(
    async (paths: string[], position: { x: number; y: number }) => {
      setDropTargetId(null);
      const targetId = blockIdAtPoint(position);
      const target = blocks.find((b) => b.id === targetId);
      const path = paths[0];
      if (!path) return;

      if (target && ["video", "image", "attachment"].includes(target.kind)) {
        await startImport(path, target.id, assetKindFor(target.kind));
        return;
      }
      toast({
        tone: "info",
        message:
          "Drop a file onto a video, image or attachment block to fill it in.",
      });
    },
    [blocks, startImport, toast],
  );

  useNativeDrop({
    onOver: (position) => {
      const overId = blockIdAtPoint(position);
      const target = blocks.find((b) => b.id === overId);
      setDropTargetId(
        authoring && target && ["video", "image", "attachment"].includes(target.kind)
          ? target.id
          : null,
      );
    },
    onDrop: ({ paths, position }) => void onDropFiles(paths, position),
    onLeave: () => setDropTargetId(null),
  });

  // --- drag reorder --------------------------------------------------------

  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 4 } }),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates }),
  );

  const onDragEnd = async (e: DragEndEvent) => {
    const { active, over } = e;
    if (!over || active.id === over.id || !chapter) return;
    const from = blocks.findIndex((b) => b.id === active.id);
    const to = blocks.findIndex((b) => b.id === over.id);
    if (from < 0 || to < 0) return;
    // Whatever ends up directly above the dragged block is its new anchor.
    const without = blocks.filter((b) => b.id !== active.id);
    const anchor = to === 0 ? null : (without[to - 1]?.id ?? null);
    try {
      await ipc.blockReorder(String(active.id), chapter.id, anchor);
    } catch (err) {
      toast({ tone: "error", message: ipc.asAppError(err).message });
    }
  };

  // --- block actions -------------------------------------------------------

  const patchBlock = useCallback(
    async (blockId: string, patch: BlockPatch) => {
      try {
        await ipc.blockUpdate(blockId, patch);
      } catch (e) {
        toast({ tone: "error", message: ipc.asAppError(e).message });
      }
    },
    [toast],
  );

  const insert = async (kind: BlockKind, afterBlockId: string | null, atStart = false) => {
    if (!chapter) return;
    try {
      const created = atStart
        ? await ipc.blockCreateAtStart(chapter.id, kind)
        : await ipc.blockCreate(chapter.id, kind, afterBlockId);
      selectBlock(created.id);
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  const move = async (block: BlockView, direction: -1 | 1) => {
    if (!chapter) return;
    const i = blocks.findIndex((b) => b.id === block.id);
    const target = i + direction;
    if (target < 0 || target >= blocks.length) return;
    const without = blocks.filter((b) => b.id !== block.id);
    const anchor = target === 0 ? null : (without[target - 1]?.id ?? null);
    try {
      await ipc.blockReorder(block.id, chapter.id, anchor);
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  const duplicate = async (block: BlockView) => {
    if (!chapter) return;
    try {
      const created = await ipc.blockCreate(chapter.id, block.kind, block.id, {
        heading: block.heading,
        captionMd: block.captionMd,
        bodyMd: block.bodyMd,
        codeText: block.codeText,
        codeLang: block.codeLang,
        metaJson: block.metaJson,
      });
      if (block.assetId) await ipc.assetAttach(created.id, block.assetId);
      selectBlock(created.id);
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  const convert = async (block: BlockView, kind: BlockKind) => {
    try {
      const losses = await ipc.blockConvertPreview(block.id, kind);
      if (losses.length > 0) {
        const ok = window.confirm(
          `Converting this block to ${kind} will discard ${losses.join(" and ")}.\n\nContinue?`,
        );
        if (!ok) return;
      }
      await ipc.blockConvert(block.id, kind);
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  const remove = async (block: BlockView) => {
    try {
      await ipc.blockDelete(block.id);
      toast({
        tone: "info",
        message: "Block moved to the Trash.",
        action: {
          label: "Undo",
          run: () => void ipc.blockRestore(block.id).catch(() => undefined),
        },
      });
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  // --- render --------------------------------------------------------------

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center text-sm text-ink-muted">
        Opening chapter…
      </div>
    );
  }

  if (error || !chapter) {
    return (
      <div className="flex h-full flex-col items-center justify-center gap-3 text-center">
        <p className="text-sm text-ink-muted">
          That chapter is not here. It may have been moved to the Trash.
        </p>
        <button className="btn" onClick={() => navigate("/")}>
          Back to the dashboard
        </button>
      </div>
    );
  }

  return (
    <div ref={scroller} className="h-full overflow-y-auto">
      <div className="mx-auto max-w-3xl px-8 py-8">
        <ChapterHeader
          chapter={chapter}
          index={index < 0 ? 0 : index}
          authoring={authoring}
          percent={percent}
          onPatch={(patch: ChapterPatch) =>
            updateChapter.mutateAsync({ id: chapter.id, patch })
          }
        />

        {blocks.length === 0 ? (
          <EmptyChapter
            authoring={authoring}
            onAdd={(kind) => void insert(kind, null)}
          />
        ) : (
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            modifiers={[restrictToVerticalAxis]}
            onDragEnd={(e) => void onDragEnd(e)}
          >
            <SortableContext
              items={blocks.map((b) => b.id)}
              strategy={verticalListSortingStrategy}
            >
              <div className="space-y-1">
                {authoring && (
                  <InsertBar
                    label="Add a block at the top"
                    onInsert={(kind) => void insert(kind, null, true)}
                  />
                )}
                {blocks.map((block, i) => (
                  <div key={block.id}>
                    <BlockRow
                      block={block}
                      authoring={authoring}
                      selected={selectedBlockId === block.id}
                      flash={flashBlockId === block.id}
                      isDropTarget={dropTargetId === block.id}
                      canMoveUp={i > 0}
                      canMoveDown={i < blocks.length - 1}
                      onSelect={() => selectBlock(block.id)}
                      onPatch={(patch) => void patchBlock(block.id, patch)}
                      onActivate={() => {
                        if (["video", "image", "attachment"].includes(block.kind)) {
                          void pickAndImport(block.id, block.kind);
                        }
                      }}
                      onLearnEmpty={() => setEmptyModal(block)}
                      onLocate={() => {
                        if (block.asset)
                          void relink(block.asset.id, block.asset.originalName);
                      }}
                      onDetach={() => void ipc.assetDetach(block.id).catch(() => undefined)}
                      onReveal={() => {
                        if (block.asset)
                          void ipc.assetReveal(block.asset.id).catch(() => undefined);
                      }}
                      onCancelImport={() => {
                        const t = cancelTicketFor(block.id);
                        if (t) void ipc.assetImportCancel(t.ticketId).catch(() => undefined);
                      }}
                      onMove={(d) => void move(block, d)}
                      onDuplicate={() => void duplicate(block)}
                      onConvert={(k) => void convert(block, k)}
                      onDelete={() => void remove(block)}
                    />
                    {authoring && (
                      <InsertBar onInsert={(kind) => void insert(kind, block.id)} />
                    )}
                  </div>
                ))}
              </div>
            </SortableContext>
          </DndContext>
        )}

        {!authoring && <ChapterNotes chapterId={chapter.id} />}

        <footer className="mt-10 flex items-center justify-between gap-3 border-t border-line pt-6">
          <button
            className={cx("btn", !prev && "invisible")}
            onClick={() => prev && navigate(`/chapter/${prev.id}`)}
          >
            <ArrowLeft size={14} />
            <span className="max-w-[12rem] truncate">{prev?.title ?? ""}</span>
          </button>

          <button
            className="btn"
            onClick={() =>
              completeChapter.mutate(
                { chapterId: chapter.id, completed: percent < 100 },
                {
                  onSuccess: () =>
                    toast({
                      tone: "success",
                      message:
                        percent < 100
                          ? "Chapter marked complete."
                          : "Chapter marked as not started.",
                    }),
                },
              )
            }
          >
            {percent >= 100 ? <CircleCheck size={14} className="text-ok" /> : <Check size={14} />}
            {percent >= 100 ? "Chapter complete" : "Mark chapter complete"}
          </button>

          <button
            className={cx("btn", !next && "invisible")}
            onClick={() => next && navigate(`/chapter/${next.id}`)}
          >
            <span className="max-w-[12rem] truncate">{next?.title ?? ""}</span>
            <ArrowRight size={14} />
          </button>
        </footer>
      </div>

      {/* Learn mode: clicking an empty block explains what is coming. */}
      <Modal
        open={!!emptyModal}
        onClose={() => setEmptyModal(null)}
        title="Not uploaded yet"
        width="sm"
        footer={
          <>
            <button className="btn" onClick={() => setEmptyModal(null)}>
              OK
            </button>
            <button
              className="btn btn-primary"
              onClick={() => {
                const block = emptyModal;
                setEmptyModal(null);
                setMode("author");
                if (block) selectBlock(block.id);
              }}
            >
              Switch to Author mode to upload
            </button>
          </>
        }
      >
        <p className="text-sm italic text-ink-muted">
          {emptyModal?.captionMd?.trim() ? (
            <>
              {`${summary?.displayNumber ? `Chapter ${summary.displayNumber}` : chapter.title} · `}
              <InlineMarkdown>{emptyModal.captionMd}</InlineMarkdown>
            </>
          ) : (
            (emptyModal?.heading ??
            "This part of the chapter has not been filled in yet.")
          )}
        </p>
      </Modal>
    </div>
  );
}

function EmptyChapter({
  authoring,
  onAdd,
}: {
  authoring: boolean;
  onAdd: (kind: BlockKind) => void;
}) {
  return (
    <div className="rounded-xl border-2 border-dashed border-line px-6 py-14 text-center">
      <p className="text-sm font-semibold text-ink">
        {authoring ? "This chapter has no blocks yet" : "This chapter is still being written"}
      </p>
      <p className="mx-auto mt-1 max-w-sm text-sm text-ink-muted">
        {authoring
          ? "Add the pieces you know it needs — a video, some prose, a code sample. You can fill them in later."
          : "Check back once the author has added the first section."}
      </p>
      {authoring && (
        <div className="mt-4 flex justify-center">
          <InsertBar label="Add the first block" onInsert={onAdd} />
        </div>
      )}
    </div>
  );
}
