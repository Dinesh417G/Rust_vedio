import { useEffect, useMemo, useRef, useState } from "react";
import { useNavigate } from "react-router-dom";
import { CornerDownLeft, Search } from "lucide-react";
import { useSearch } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { cx } from "@/lib/format";
import type { SearchHit } from "@/lib/types";

/** Ctrl+K. Results grouped by chapter; Enter jumps and flashes the block. */
export function CommandPalette() {
  const open = useUi((s) => s.paletteOpen);
  const setOpen = useUi((s) => s.setPaletteOpen);
  const flashBlock = useUi((s) => s.flashBlock);
  const selectBlock = useUi((s) => s.selectBlock);
  const navigate = useNavigate();

  const [q, setQ] = useState("");
  const [cursor, setCursor] = useState(0);
  const input = useRef<HTMLInputElement>(null);
  const { data: hits = [], isFetching } = useSearch(q, open);

  const grouped = useMemo(() => {
    const map = new Map<string, { title: string; number: string | null; hits: SearchHit[] }>();
    for (const h of hits) {
      const g = map.get(h.chapterId) ?? {
        title: h.chapterTitle,
        number: h.chapterNumber,
        hits: [],
      };
      g.hits.push(h);
      map.set(h.chapterId, g);
    }
    return [...map.entries()];
  }, [hits]);

  const flat = useMemo(() => grouped.flatMap(([, g]) => g.hits), [grouped]);

  useEffect(() => {
    if (open) {
      setCursor(0);
      requestAnimationFrame(() => input.current?.focus());
    }
  }, [open]);

  useEffect(() => setCursor(0), [q]);

  if (!open) return null;

  const go = (hit: SearchHit) => {
    setOpen(false);
    navigate(`/chapter/${hit.chapterId}`);
    if (hit.entityType === "block") {
      selectBlock(hit.entityId);
      flashBlock(hit.entityId);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center bg-black/60 p-6 pt-[12vh]"
      onMouseDown={(e) => {
        if (e.target === e.currentTarget) setOpen(false);
      }}
    >
      <div className="card w-full max-w-2xl overflow-hidden shadow-2xl" role="dialog" aria-label="Search">
        <div className="flex items-center gap-2 border-b border-line px-4">
          <Search size={16} className="shrink-0 text-ink-faint" />
          <input
            ref={input}
            className="w-full bg-transparent py-3 text-sm text-ink outline-none placeholder:text-ink-faint"
            placeholder="Search chapters, prose and code…"
            value={q}
            onChange={(e) => setQ(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Escape") setOpen(false);
              if (e.key === "ArrowDown") {
                e.preventDefault();
                setCursor((c) => Math.min(flat.length - 1, c + 1));
              }
              if (e.key === "ArrowUp") {
                e.preventDefault();
                setCursor((c) => Math.max(0, c - 1));
              }
              if (e.key === "Enter" && flat[cursor]) {
                e.preventDefault();
                go(flat[cursor]);
              }
            }}
          />
          <kbd className="shrink-0 rounded border border-line px-1.5 py-0.5 text-[10px] text-ink-faint">
            Esc
          </kbd>
        </div>

        <div className="max-h-[55vh] overflow-y-auto p-2">
          {q.trim().length === 0 && (
            <p className="px-2 py-6 text-center text-sm text-ink-muted">
              Type to search chapter titles, summaries, prose and code.
            </p>
          )}
          {q.trim().length > 0 && flat.length === 0 && !isFetching && (
            <p className="px-2 py-6 text-center text-sm text-ink-muted">
              Nothing matches &ldquo;{q}&rdquo;.
            </p>
          )}
          {grouped.map(([chapterId, group]) => (
            <div key={chapterId} className="mb-2">
              <p className="px-2 py-1 text-xs font-semibold text-ink-faint">
                {group.number ? `${group.number}. ` : ""}
                {group.title}
              </p>
              {group.hits.map((hit) => {
                const i = flat.indexOf(hit);
                return (
                  <button
                    key={hit.entityId}
                    onMouseEnter={() => setCursor(i)}
                    onClick={() => go(hit)}
                    className={cx(
                      "flex w-full items-start gap-2 rounded-lg px-2.5 py-2 text-left",
                      i === cursor ? "bg-brand/15" : "hover:bg-surface-2",
                    )}
                  >
                    <span className="min-w-0 flex-1">
                      <span className="block truncate text-sm text-ink">{hit.title}</span>
                      {hit.snippet.trim() && (
                        <span className="block truncate text-xs text-ink-muted">
                          {hit.snippet}
                        </span>
                      )}
                    </span>
                    {i === cursor && (
                      <CornerDownLeft size={13} className="mt-1 shrink-0 text-ink-faint" />
                    )}
                  </button>
                );
              })}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
