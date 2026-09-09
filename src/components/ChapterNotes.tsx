import { useState } from "react";
import { NotebookPen, Trash2 } from "lucide-react";
import * as ipc from "@/lib/ipc";
import { useNotes } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { relativeDate } from "@/lib/format";
import { Markdown } from "./Markdown";

/** My own notes while studying, kept per chapter. Learn mode only. */
export function ChapterNotes({ chapterId }: { chapterId: string }) {
  const { data: notes = [] } = useNotes(chapterId);
  const toast = useUi((s) => s.toast);
  const [draft, setDraft] = useState("");
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editDraft, setEditDraft] = useState("");

  const add = async () => {
    if (!draft.trim()) return;
    try {
      await ipc.noteCreate(chapterId, draft.trim());
      setDraft("");
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  return (
    <section className="mt-10 rounded-xl border border-line bg-surface-1 p-5">
      <h2 className="mb-3 inline-flex items-center gap-2 text-sm font-semibold text-ink">
        <NotebookPen size={15} className="text-brand" /> My notes
      </h2>

      {notes.length === 0 && (
        <p className="mb-3 text-sm text-ink-muted">
          Nothing noted yet for this chapter.
        </p>
      )}

      <ul className="mb-3 space-y-2">
        {notes.map((n) => (
          <li key={n.id} className="rounded-lg border border-line bg-surface-2 p-3">
            {editingId === n.id ? (
              <div className="space-y-2">
                <textarea
                  autoFocus
                  className="input min-h-[5rem] resize-y font-mono text-[13px]"
                  value={editDraft}
                  onChange={(e) => setEditDraft(e.target.value)}
                />
                <div className="flex justify-end gap-2">
                  <button className="btn btn-sm btn-ghost" onClick={() => setEditingId(null)}>
                    Cancel
                  </button>
                  <button
                    className="btn btn-sm btn-primary"
                    onClick={async () => {
                      try {
                        await ipc.noteUpdate(n.id, editDraft);
                        setEditingId(null);
                      } catch (e) {
                        toast({ tone: "error", message: ipc.asAppError(e).message });
                      }
                    }}
                  >
                    Save
                  </button>
                </div>
              </div>
            ) : (
              <>
                <div
                  className="cursor-text"
                  onClick={() => {
                    setEditingId(n.id);
                    setEditDraft(n.bodyMd);
                  }}
                >
                  <Markdown>{n.bodyMd}</Markdown>
                </div>
                <div className="mt-2 flex items-center justify-between text-xs text-ink-faint">
                  <span>{relativeDate(n.updatedAt)}</span>
                  <button
                    className="btn btn-ghost btn-sm"
                    title="Delete note"
                    onClick={() => void ipc.noteDelete(n.id).catch(() => undefined)}
                  >
                    <Trash2 size={13} />
                  </button>
                </div>
              </>
            )}
          </li>
        ))}
      </ul>

      <textarea
        className="input min-h-[4.5rem] resize-y"
        value={draft}
        placeholder="What did this chapter make click? Markdown works here."
        onChange={(e) => setDraft(e.target.value)}
        onKeyDown={(e) => {
          if ((e.ctrlKey || e.metaKey) && e.key === "Enter") void add();
        }}
      />
      <div className="mt-2 flex items-center justify-between">
        <span className="text-xs text-ink-faint">Ctrl+Enter to save</span>
        <button className="btn btn-sm btn-primary" disabled={!draft.trim()} onClick={add}>
          Add note
        </button>
      </div>
    </section>
  );
}
