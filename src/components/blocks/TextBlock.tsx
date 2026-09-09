import { useEffect, useRef, useState } from "react";
import { PenLine } from "lucide-react";
import { BlockShell, EmptyCard } from "../BlockShell";
import { Markdown } from "../Markdown";
import { SaveIndicator } from "../SaveIndicator";
import { useAutosave } from "@/lib/autosave";
import type { BlockRendererProps } from "./common";

export function TextBlock(props: BlockRendererProps) {
  const { block, authoring, onPatch, onLearnEmpty } = props;
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState(block.bodyMd ?? "");
  const area = useRef<HTMLTextAreaElement>(null);

  const { status, schedule, flush } = useAutosave(draft, (v: string) =>
    Promise.resolve(onPatch({ bodyMd: v })),
  );

  useEffect(() => {
    if (!editing) setDraft(block.bodyMd ?? "");
  }, [block.bodyMd, editing]);

  useEffect(() => {
    if (editing) area.current?.focus();
  }, [editing]);

  const openEditor = () => {
    if (!authoring) return;
    setDraft(block.bodyMd ?? "");
    setEditing(true);
  };

  const editor = (
    <div className="space-y-2">
      <textarea
        ref={area}
        className="input min-h-[10rem] resize-y font-mono text-[13px] leading-relaxed"
        value={draft}
        placeholder="Write in markdown. Headings, lists, links and tables all work."
        onChange={(e) => {
          setDraft(e.target.value);
          schedule(e.target.value);
        }}
        onKeyDown={(e) => {
          if (e.key === "Escape") {
            e.stopPropagation();
            void flush();
            setEditing(false);
          }
        }}
      />
      <div className="flex items-center justify-between">
        <SaveIndicator status={status} />
        <button
          className="btn btn-sm"
          onClick={() => {
            void flush();
            setEditing(false);
          }}
        >
          Done
        </button>
      </div>
    </div>
  );

  return (
    <BlockShell
      {...shellProps(props)}
      ready={
        editing ? (
          editor
        ) : (
          <div
            className={
              authoring
                ? "-mx-2 cursor-text rounded-lg px-2 py-1 transition-colors hover:bg-surface-1"
                : undefined
            }
            onClick={openEditor}
          >
            <Markdown>{block.bodyMd}</Markdown>
          </div>
        )
      }
      empty={
        editing ? (
          editor
        ) : (
          <EmptyCard
            authoring={authoring}
            icon={<PenLine size={26} />}
            title={authoring ? "Nothing written yet" : "Notes coming soon"}
            subline={
              authoring
                ? "Write the prose for this section."
                : "This section hasn't been written yet."
            }
            captionMd={block.captionMd}
            action={authoring ? { label: "Start writing", run: openEditor } : undefined}
            onActivate={authoring ? openEditor : onLearnEmpty}
          />
        )
      }
    />
  );
}

/** Shared plumbing from renderer props to shell props. */
export function shellProps(p: BlockRendererProps) {
  return {
    state: p.block.state,
    kind: p.block.kind,
    heading: p.block.heading,
    captionMd: p.block.captionMd,
    authoring: p.authoring,
    selected: p.selected,
    flash: p.flash,
    toolbar: p.toolbar,
  };
}
