import { useState } from "react";
import { Plus } from "lucide-react";
import type { BlockKind } from "@/lib/types";
import { BlockPalette } from "./BlockPalette";

/** The thin "+" strip that lives between every pair of blocks. */
export function InsertBar({
  onInsert,
  label = "Add a block here",
}: {
  onInsert: (kind: BlockKind) => void;
  label?: string;
}) {
  const [open, setOpen] = useState(false);
  return (
    <div className="relative flex h-6 items-center justify-center">
      <button
        aria-label={label}
        title={label}
        onClick={() => setOpen((o) => !o)}
        className="group/insert flex w-full items-center gap-2 rounded px-2 py-1 opacity-0 transition-opacity hover:opacity-100 focus-visible:opacity-100"
      >
        <span className="h-px flex-1 bg-brand/40" />
        <span className="inline-flex items-center gap-1 rounded-full border border-brand/50 bg-surface-1 px-2 py-0.5 text-xs text-brand">
          <Plus size={11} /> Add block
        </span>
        <span className="h-px flex-1 bg-brand/40" />
      </button>
      {open && (
        <BlockPalette
          onClose={() => setOpen(false)}
          onPick={(kind) => {
            setOpen(false);
            onInsert(kind);
          }}
        />
      )}
    </div>
  );
}
