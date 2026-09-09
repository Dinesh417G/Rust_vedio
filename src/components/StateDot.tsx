import type { BlockState } from "@/lib/types";
import { cx } from "@/lib/format";

const SPEC: Record<BlockState, { glyph: string; className: string; label: string }> = {
  ready: { glyph: "●", className: "text-ok", label: "Ready" },
  empty: { glyph: "○", className: "text-ink-faint", label: "Not filled in yet" },
  missing: { glyph: "▲", className: "text-danger", label: "File is missing" },
  importing: { glyph: "◐", className: "text-brand animate-pulse", label: "Importing" },
};

export function StateDot({ state, className }: { state: BlockState; className?: string }) {
  const s = SPEC[state];
  return (
    <span
      className={cx("select-none text-[10px] leading-none", s.className, className)}
      title={s.label}
      aria-label={s.label}
      role="img"
    >
      {s.glyph}
    </span>
  );
}

export { SPEC as STATE_SPEC };
