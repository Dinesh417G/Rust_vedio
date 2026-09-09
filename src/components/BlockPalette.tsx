import { useEffect, useRef } from "react";
import {
  Code2,
  Film,
  ImageIcon,
  Info,
  Minus,
  Paperclip,
  PenLine,
  CircleHelp,
} from "lucide-react";
import type { BlockKind } from "@/lib/types";
import { BLOCK_KINDS } from "@/lib/types";
import { KIND_BLURB, KIND_LABEL } from "./blocks/common";

const ICON: Record<BlockKind, typeof PenLine> = {
  text: PenLine,
  video: Film,
  image: ImageIcon,
  code: Code2,
  callout: Info,
  attachment: Paperclip,
  quiz: CircleHelp,
  divider: Minus,
};

/** The popover behind every "+" insert bar. */
export function BlockPalette({
  onPick,
  onClose,
}: {
  onPick: (kind: BlockKind) => void;
  onClose: () => void;
}) {
  const root = useRef<HTMLDivElement>(null);

  useEffect(() => {
    root.current?.querySelector<HTMLButtonElement>("button")?.focus();
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.stopPropagation();
        onClose();
      }
    };
    const onClickAway = (e: MouseEvent) => {
      if (root.current && !root.current.contains(e.target as Node)) onClose();
    };
    document.addEventListener("keydown", onKey, true);
    document.addEventListener("mousedown", onClickAway);
    return () => {
      document.removeEventListener("keydown", onKey, true);
      document.removeEventListener("mousedown", onClickAway);
    };
  }, [onClose]);

  return (
    <div
      ref={root}
      role="menu"
      aria-label="Add a block"
      className="card absolute left-1/2 top-full z-30 mt-1 w-80 -translate-x-1/2 p-1.5 shadow-2xl"
    >
      {BLOCK_KINDS.map((kind) => {
        const Icon = ICON[kind];
        return (
          <button
            key={kind}
            role="menuitem"
            onClick={() => onPick(kind)}
            className="flex w-full items-center gap-3 rounded-lg px-2.5 py-2 text-left transition-colors hover:bg-surface-2"
          >
            <span className="grid h-8 w-8 shrink-0 place-items-center rounded-lg bg-surface-2 text-ink-muted">
              <Icon size={15} />
            </span>
            <span className="min-w-0">
              <span className="block text-sm font-medium text-ink">{KIND_LABEL[kind]}</span>
              <span className="block truncate text-xs text-ink-muted">
                {KIND_BLURB[kind]}
              </span>
            </span>
          </button>
        );
      })}
    </div>
  );
}
