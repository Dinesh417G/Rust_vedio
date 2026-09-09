import { useEffect, useRef, useState } from "react";
import { Info, Lightbulb, OctagonAlert, TriangleAlert } from "lucide-react";
import { useInlineEditor } from "@/lib/useInlineEditor";
import { BlockShell, EmptyCard } from "../BlockShell";
import { Markdown } from "../Markdown";
import { SaveIndicator } from "../SaveIndicator";
import { useAutosave } from "@/lib/autosave";
import { cx } from "@/lib/format";
import type { CalloutTone } from "@/lib/types";
import { calloutMeta, type BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";

const TONES: Record<
  CalloutTone,
  { label: string; icon: typeof Info; wrap: string; accent: string }
> = {
  note: {
    label: "Note",
    icon: Info,
    wrap: "border-line bg-surface-2/60",
    accent: "text-ink-muted",
  },
  tip: {
    label: "Tip",
    icon: Lightbulb,
    wrap: "border-ok/40 bg-ok/10",
    accent: "text-ok",
  },
  warning: {
    label: "Watch out",
    icon: TriangleAlert,
    wrap: "border-warn/40 bg-warn/10",
    accent: "text-warn",
  },
  danger: {
    label: "Careful",
    icon: OctagonAlert,
    wrap: "border-danger/40 bg-danger/10",
    accent: "text-danger",
  },
};

export function CalloutBlock(props: BlockRendererProps) {
  const { block, authoring, onPatch, onLearnEmpty } = props;
  const tone = calloutMeta(block).tone ?? "note";
  const spec = TONES[tone];
  const Icon = spec.icon;

  const [editing, setEditing] = useInlineEditor(authoring);
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

  const open = () => {
    if (!authoring) return;
    setDraft(block.bodyMd ?? "");
    setEditing(true);
  };

  const toneRow = (
    <div className="flex flex-wrap gap-1">
      {(Object.keys(TONES) as CalloutTone[]).map((t) => (
        <button
          key={t}
          className={cx("btn btn-sm", tone === t && "border-brand text-brand")}
          onClick={() => onPatch({ metaJson: JSON.stringify({ ...calloutMeta(block), tone: t }) })}
        >
          {TONES[t].label}
        </button>
      ))}
    </div>
  );

  const editor = (
    <div className={cx("space-y-2 rounded-xl border p-4", spec.wrap)}>
      {toneRow}
      <textarea
        ref={area}
        className="input min-h-[6rem] resize-y font-mono text-[13px]"
        value={draft}
        placeholder="The thing that trips people up."
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
            className={cx(
              "flex gap-3 rounded-xl border p-4",
              spec.wrap,
              authoring && "cursor-text",
            )}
            onClick={open}
          >
            <Icon size={18} className={cx("mt-0.5 shrink-0", spec.accent)} />
            <div className="min-w-0 flex-1">
              <p className={cx("mb-1 text-xs font-semibold uppercase tracking-wide", spec.accent)}>
                {spec.label}
              </p>
              <Markdown>{block.bodyMd}</Markdown>
            </div>
          </div>
        )
      }
      empty={
        editing ? (
          editor
        ) : (
          <EmptyCard
            authoring={authoring}
            icon={<Icon size={26} />}
            title={authoring ? "Nothing written yet" : "Notes coming soon"}
            subline={
              authoring
                ? "Add the note, tip or warning for this section."
                : "This section hasn't been written yet."
            }
            captionMd={block.captionMd}
            action={authoring ? { label: "Start writing", run: open } : undefined}
            onActivate={authoring ? open : onLearnEmpty}
          />
        )
      }
    />
  );
}
