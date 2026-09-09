import { Check, CloudOff, Loader2 } from "lucide-react";
import type { SaveStatus } from "@/lib/autosave";

/** Deliberately quiet: it should be noticed only when it says "Not saved". */
export function SaveIndicator({ status }: { status: SaveStatus }) {
  if (status === "idle") return null;
  if (status === "dirty")
    return <span className="text-xs text-ink-faint">Unsaved changes</span>;
  if (status === "saving")
    return (
      <span className="inline-flex items-center gap-1 text-xs text-ink-faint">
        <Loader2 size={12} className="animate-spin" /> Saving
      </span>
    );
  if (status === "saved")
    return (
      <span className="inline-flex items-center gap-1 text-xs text-ok">
        <Check size={12} /> Saved
      </span>
    );
  return (
    <span className="inline-flex items-center gap-1 text-xs text-danger">
      <CloudOff size={12} /> Not saved
    </span>
  );
}
