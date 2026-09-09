import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Search } from "lucide-react";
import { useSearch } from "@/lib/queries";
import { useUi } from "@/store/ui";

export function SearchPage() {
  const [q, setQ] = useState("");
  const { data: hits = [], isFetching } = useSearch(q);
  const navigate = useNavigate();
  const flashBlock = useUi((s) => s.flashBlock);
  const selectBlock = useUi((s) => s.selectBlock);

  return (
    <div className="h-full overflow-y-auto">
      <div className="mx-auto max-w-3xl px-8 py-10">
        <h1 className="mb-4 text-2xl font-bold text-ink">Search</h1>
        <div className="mb-6 flex items-center gap-2 rounded-lg border border-line bg-surface-1 px-3">
          <Search size={16} className="text-ink-faint" />
          <input
            autoFocus
            className="w-full bg-transparent py-2.5 text-sm outline-none placeholder:text-ink-faint"
            placeholder="Search chapters, prose and code…"
            value={q}
            onChange={(e) => setQ(e.target.value)}
          />
        </div>

        {q.trim() && hits.length === 0 && !isFetching && (
          <p className="text-sm text-ink-muted">Nothing matches &ldquo;{q}&rdquo;.</p>
        )}

        <ul className="space-y-2">
          {hits.map((hit) => (
            <li key={hit.entityId}>
              <button
                className="w-full rounded-xl border border-line bg-surface-1 p-4 text-left hover:border-brand/60"
                onClick={() => {
                  navigate(`/chapter/${hit.chapterId}`);
                  if (hit.entityType === "block") {
                    selectBlock(hit.entityId);
                    flashBlock(hit.entityId);
                  }
                }}
              >
                <p className="text-xs text-ink-faint">
                  {hit.chapterNumber ? `${hit.chapterNumber}. ` : ""}
                  {hit.chapterTitle}
                </p>
                <p className="text-sm font-medium text-ink">{hit.title}</p>
                {hit.snippet.trim() && (
                  <p className="mt-1 text-xs text-ink-muted">{hit.snippet}</p>
                )}
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
