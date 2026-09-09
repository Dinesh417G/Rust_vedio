import { useState } from "react";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import {
  ArrowDown,
  ArrowUp,
  Copy,
  GripVertical,
  Replace,
  Trash2,
} from "lucide-react";
import type { BlockKind, BlockPatch, BlockView } from "@/lib/types";
import { BLOCK_KINDS } from "@/lib/types";
import { cx } from "@/lib/format";
import { BlockRenderer, KIND_LABEL } from "./blocks";
import { ConfirmDialog } from "./ui/ConfirmDialog";

export interface BlockRowProps {
  block: BlockView;
  authoring: boolean;
  selected: boolean;
  flash: boolean;
  isDropTarget: boolean;
  canMoveUp: boolean;
  canMoveDown: boolean;
  onSelect: () => void;
  onPatch: (patch: BlockPatch) => void;
  onActivate: () => void;
  onLearnEmpty: () => void;
  onLocate: () => void;
  onDetach: () => void;
  onReveal: () => void;
  onCancelImport: () => void;
  onMove: (direction: -1 | 1) => void;
  onDuplicate: () => void;
  onConvert: (kind: BlockKind) => void;
  onDelete: () => void;
}

export function BlockRow(props: BlockRowProps) {
  const { block, authoring, selected } = props;
  const {
    attributes,
    listeners,
    setNodeRef,
    setActivatorNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: block.id, disabled: !authoring });

  const [convertOpen, setConvertOpen] = useState(false);
  const [confirmDelete, setConfirmDelete] = useState(false);

  const toolbar = (
    <div className="flex items-center gap-0.5 rounded-lg border border-line bg-surface-1 p-0.5 shadow-sm">
      <button
        ref={setActivatorNodeRef}
        {...attributes}
        {...listeners}
        className="btn btn-ghost btn-sm cursor-grab active:cursor-grabbing"
        title="Drag to reorder"
        aria-label="Drag to reorder"
      >
        <GripVertical size={14} />
      </button>
      <button
        className="btn btn-ghost btn-sm"
        title="Move up"
        disabled={!props.canMoveUp}
        onClick={() => props.onMove(-1)}
      >
        <ArrowUp size={14} />
      </button>
      <button
        className="btn btn-ghost btn-sm"
        title="Move down"
        disabled={!props.canMoveDown}
        onClick={() => props.onMove(1)}
      >
        <ArrowDown size={14} />
      </button>
      <button className="btn btn-ghost btn-sm" title="Duplicate" onClick={props.onDuplicate}>
        <Copy size={14} />
      </button>
      <div className="relative">
        <button
          className="btn btn-ghost btn-sm"
          title="Convert to another kind"
          onClick={() => setConvertOpen((o) => !o)}
        >
          <Replace size={14} />
        </button>
        {convertOpen && (
          <div
            className="card absolute right-0 top-full z-30 mt-1 w-44 p-1 shadow-2xl"
            onMouseLeave={() => setConvertOpen(false)}
          >
            <p className="px-2 py-1 text-xs text-ink-faint">Convert to</p>
            {BLOCK_KINDS.filter((k) => k !== block.kind).map((k) => (
              <button
                key={k}
                className="block w-full rounded px-2 py-1 text-left text-sm hover:bg-surface-2"
                onClick={() => {
                  setConvertOpen(false);
                  props.onConvert(k);
                }}
              >
                {KIND_LABEL[k]}
              </button>
            ))}
          </div>
        )}
      </div>
      <button
        className="btn btn-ghost btn-sm text-danger"
        title="Delete block"
        onClick={() => setConfirmDelete(true)}
      >
        <Trash2 size={14} />
      </button>
    </div>
  );

  return (
    <div
      ref={setNodeRef}
      id={`block-${block.id}`}
      style={{ transform: CSS.Translate.toString(transform), transition }}
      className={cx(
        "relative scroll-mt-24 rounded-xl px-3 py-2",
        isDragging && "z-20 opacity-60 shadow-2xl",
        authoring && "hover:bg-surface-1/40",
      )}
      onMouseDown={authoring ? props.onSelect : undefined}
    >
      <BlockRenderer
        block={block}
        authoring={authoring}
        selected={selected}
        flash={props.flash}
        toolbar={toolbar}
        onActivate={props.onActivate}
        onLearnEmpty={props.onLearnEmpty}
        onPatch={props.onPatch}
        onLocate={props.onLocate}
        onDetach={props.onDetach}
        onReveal={props.onReveal}
        onCancelImport={props.onCancelImport}
        isDropTarget={props.isDropTarget}
      />

      <ConfirmDialog
        open={confirmDelete}
        title={`Delete this ${KIND_LABEL[block.kind].toLowerCase()} block?`}
        consequence={`It moves to the Trash and can be restored for 30 days. ${
          block.asset
            ? "The file stays in your media library and is not deleted."
            : "Nothing on disk is affected."
        }`}
        confirmLabel="Move to Trash"
        onCancel={() => setConfirmDelete(false)}
        onConfirm={() => {
          setConfirmDelete(false);
          props.onDelete();
        }}
      />
    </div>
  );
}
