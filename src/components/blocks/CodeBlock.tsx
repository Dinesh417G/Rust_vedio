import { useEffect, useRef, useState } from "react";
import { Code2, Copy } from "lucide-react";
import { useInlineEditor } from "@/lib/useInlineEditor";
import { BlockShell, EmptyCard } from "../BlockShell";
import { CodeView, CODE_LANGS, normalizeLang } from "../CodeView";
import { SaveIndicator } from "../SaveIndicator";
import { useAutosave } from "@/lib/autosave";
import { useUi } from "@/store/ui";
import { cx } from "@/lib/format";
import type { BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";

export function CodeBlock(props: BlockRendererProps) {
  const { block, authoring, onPatch, onLearnEmpty } = props;
  const toast = useUi((s) => s.toast);
  const [editing, setEditing] = useInlineEditor(authoring);
  const [draft, setDraft] = useState(block.codeText ?? "");
  const area = useRef<HTMLTextAreaElement>(null);
  const lang = normalizeLang(block.codeLang);

  const { status, schedule, flush } = useAutosave(draft, (v: string) =>
    Promise.resolve(onPatch({ codeText: v })),
  );

  useEffect(() => {
    if (!editing) setDraft(block.codeText ?? "");
  }, [block.codeText, editing]);
  useEffect(() => {
    if (editing) area.current?.focus();
  }, [editing]);

  const open = () => {
    if (!authoring) return;
    setDraft(block.codeText ?? "");
    setEditing(true);
  };

  const copy = async () => {
    try {
      await navigator.clipboard.writeText(block.codeText ?? "");
      toast({ tone: "success", message: "Code copied." });
    } catch {
      toast({ tone: "error", message: "The clipboard is not available." });
    }
  };

  const editor = (
    <div className="space-y-2">
      <div className="flex flex-wrap items-center gap-1">
        {CODE_LANGS.map((l) => (
          <button
            key={l}
            className={cx("btn btn-sm font-mono", lang === l && "border-brand text-brand")}
            onClick={() => onPatch({ codeLang: l })}
          >
            {l}
          </button>
        ))}
      </div>
      <textarea
        ref={area}
        spellCheck={false}
        className="input min-h-[12rem] resize-y whitespace-pre font-mono text-[13px] leading-relaxed"
        value={draft}
        placeholder={"fn main() {\n    println!(\"Hello, RustClass\");\n}"}
        onChange={(e) => {
          setDraft(e.target.value);
          schedule(e.target.value);
        }}
        onKeyDown={(e) => {
          // Tab indents rather than escaping the field — this is a code editor.
          if (e.key === "Tab") {
            e.preventDefault();
            const el = e.currentTarget;
            const { selectionStart: a, selectionEnd: b } = el;
            const next = `${draft.slice(0, a)}    ${draft.slice(b)}`;
            setDraft(next);
            schedule(next);
            requestAnimationFrame(() => el.setSelectionRange(a + 4, a + 4));
          }
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
          <div className="group/code relative">
            <div className="absolute right-2 top-2 z-10 flex gap-1 opacity-0 transition-opacity group-hover/code:opacity-100">
              <button className="btn btn-sm" onClick={copy} title="Copy code">
                <Copy size={13} />
              </button>
              {authoring && (
                <button className="btn btn-sm" onClick={open}>
                  Edit
                </button>
              )}
            </div>
            <CodeView code={block.codeText ?? ""} lang={block.codeLang} />
          </div>
        )
      }
      empty={
        editing ? (
          editor
        ) : (
          <EmptyCard
            authoring={authoring}
            icon={<Code2 size={26} />}
            title={authoring ? "No code sample yet" : "Code sample coming soon"}
            subline={
              authoring
                ? "Paste the code this section walks through."
                : "This code sample hasn't been written yet."
            }
            captionMd={block.captionMd}
            action={authoring ? { label: "Add code", run: open } : undefined}
            onActivate={authoring ? open : onLearnEmpty}
          />
        )
      }
    />
  );
}
