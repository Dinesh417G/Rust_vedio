import { useEffect, useState } from "react";
import {
  ExternalLink,
  FolderOpen,
  Link2Off,
  PanelRightClose,
  Replace,
  Upload,
} from "lucide-react";
import * as ipc from "@/lib/ipc";
import type { BlockKind, BlockView } from "@/lib/types";
import { BLOCK_KINDS } from "@/lib/types";
import { useChapter } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { useAutosave } from "@/lib/autosave";
import { usePickAndImport, useRelink } from "@/lib/useImport";
import { clockTime, cx, humanBytes, relativeDate } from "@/lib/format";
import { KIND_LABEL } from "./blocks/common";
import { SaveIndicator } from "./SaveIndicator";
import { STATE_SPEC, StateDot } from "./StateDot";
import { CODE_LANGS, normalizeLang } from "./CodeView";

/** The right-hand panel. Author mode only, per the spec. */
export function Inspector({ chapterId }: { chapterId: string | undefined }) {
  const open = useUi((s) => s.inspectorOpen);
  const setOpen = useUi((s) => s.setInspectorOpen);
  const selectedId = useUi((s) => s.selectedBlockId);
  const { data: chapter } = useChapter(chapterId);
  const block = chapter?.blocks.find((b) => b.id === selectedId) ?? null;

  if (!open) return null;

  return (
    <aside className="flex w-80 shrink-0 flex-col border-l border-line bg-surface-1">
      <div className="flex items-center justify-between border-b border-line px-3 py-2">
        <h2 className="text-xs font-semibold uppercase tracking-wide text-ink-faint">
          Inspector
        </h2>
        <button
          className="btn btn-ghost btn-sm"
          onClick={() => setOpen(false)}
          title="Hide inspector"
        >
          <PanelRightClose size={14} />
        </button>
      </div>
      <div className="min-h-0 flex-1 overflow-y-auto p-3">
        {block ? (
          <BlockInspector key={block.id} block={block} />
        ) : (
          <p className="px-1 py-6 text-center text-sm text-ink-muted">
            Select a block to edit its heading, description and settings.
          </p>
        )}
      </div>
    </aside>
  );
}

/** A placeholder that describes the block in front of you, not a video. */
const CAPTION_PLACEHOLDER: Record<BlockKind, string> = {
  video: "This video covers moves, clones and the borrow checker",
  image: "How the stack and the heap relate",
  text: "Why the borrow checker exists at all",
  code: "The example this section walks through",
  callout: "The mistake people make here",
  attachment: "The worksheet for this section",
  quiz: "A few questions to confirm the idea landed",
  divider: "A break between sections",
};

function BlockInspector({ block }: { block: BlockView }) {
  const toast = useUi((s) => s.toast);
  const pickAndImport = usePickAndImport();
  const relink = useRelink();

  const [heading, setHeading] = useState(block.heading ?? "");
  const [caption, setCaption] = useState(block.captionMd ?? "");

  const { status, schedule } = useAutosave<{ heading?: string; captionMd?: string }>(
    {},
    (patch) => ipc.blockUpdate(block.id, patch),
  );

  useEffect(() => {
    setHeading(block.heading ?? "");
    setCaption(block.captionMd ?? "");
  }, [block.id, block.heading, block.captionMd]);

  const assetBacked = ["video", "image", "attachment"].includes(block.kind);
  const lang = normalizeLang(block.codeLang);

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <span className="chip">
          <StateDot state={block.state} />
          {KIND_LABEL[block.kind]} · {STATE_SPEC[block.state].label}
        </span>
        <SaveIndicator status={status} />
      </div>

      <div>
        <label className="label" htmlFor="insp-heading">
          Heading
        </label>
        <input
          id="insp-heading"
          className="input"
          value={heading}
          placeholder="Move semantics"
          onChange={(e) => {
            setHeading(e.target.value);
            schedule({ heading: e.target.value.trim() ? e.target.value : "" });
          }}
        />
      </div>

      <div>
        <label className="label" htmlFor="insp-caption">
          What this is about
        </label>
        <textarea
          id="insp-caption"
          className="input min-h-[5rem] resize-y text-[13px]"
          value={caption}
          placeholder={CAPTION_PLACEHOLDER[block.kind]}
          onChange={(e) => {
            setCaption(e.target.value);
            schedule({ captionMd: e.target.value });
          }}
        />
        <p className="mt-1 text-xs text-ink-faint">
          Shown whether or not the content exists — this is what makes a placeholder
          useful.
        </p>
      </div>

      {block.kind === "code" && (
        <div>
          <span className="label">Language</span>
          <div className="flex flex-wrap gap-1">
            {CODE_LANGS.map((l) => (
              <button
                key={l}
                className={cx("btn btn-sm font-mono", lang === l && "border-brand text-brand")}
                onClick={() => void ipc.blockUpdate(block.id, { codeLang: l })}
              >
                {l}
              </button>
            ))}
          </div>
        </div>
      )}

      {assetBacked && (
        <div>
          <span className="label">File</span>
          {block.asset ? (
            <div className="space-y-2 rounded-lg border border-line bg-surface-2 p-3">
              <p className="break-words text-sm font-medium text-ink">
                {block.asset.originalName}
              </p>
              <dl className="space-y-0.5 text-xs text-ink-muted">
                <Row label="Size" value={humanBytes(block.asset.bytes)} />
                {block.asset.durationSecs != null && (
                  <Row label="Duration" value={clockTime(block.asset.durationSecs)} />
                )}
                {block.asset.width != null && block.asset.height != null && (
                  <Row
                    label="Dimensions"
                    value={`${block.asset.width} × ${block.asset.height}`}
                  />
                )}
                <Row
                  label="Stored"
                  value={block.asset.importMode === "copy" ? "Copied in" : "Referenced"}
                />
                <Row label="Imported" value={relativeDate(block.asset.importedAt)} />
              </dl>
              <p className="break-all font-mono text-[10px] text-ink-faint">
                {block.asset.resolvedPath}
              </p>
              <div className="flex flex-wrap gap-1">
                <button
                  className="btn btn-sm"
                  onClick={() => void pickAndImport(block.id, block.kind)}
                >
                  <Replace size={12} /> Replace
                </button>
                <button
                  className="btn btn-sm"
                  onClick={() => void ipc.assetReveal(block.asset!.id).catch(() => undefined)}
                >
                  <FolderOpen size={12} /> Show
                </button>
                <button
                  className="btn btn-sm"
                  onClick={() =>
                    void ipc.assetOpenExternal(block.asset!.id).catch(() => undefined)
                  }
                >
                  <ExternalLink size={12} /> Open
                </button>
                <button
                  className="btn btn-sm btn-ghost"
                  onClick={async () => {
                    try {
                      await ipc.assetDetach(block.id);
                      toast({
                        tone: "info",
                        message: "Detached. The file is still in your media library.",
                      });
                    } catch (e) {
                      toast({ tone: "error", message: ipc.asAppError(e).message });
                    }
                  }}
                >
                  <Link2Off size={12} /> Detach
                </button>
                {!block.asset.fileExists && (
                  <button
                    className="btn btn-sm btn-primary"
                    onClick={() => void relink(block.asset!.id, block.asset!.originalName)}
                  >
                    Locate file…
                  </button>
                )}
              </div>
            </div>
          ) : (
            <button
              className="btn w-full"
              onClick={() => void pickAndImport(block.id, block.kind)}
            >
              <Upload size={13} /> Choose a file…
            </button>
          )}
        </div>
      )}

      <div>
        <span className="label">Convert to</span>
        <div className="flex flex-wrap gap-1">
          {BLOCK_KINDS.filter((k) => k !== block.kind).map((k) => (
            <ConvertButton key={k} blockId={block.id} kind={k} />
          ))}
        </div>
      </div>
    </div>
  );
}

function ConvertButton({ blockId, kind }: { blockId: string; kind: BlockKind }) {
  const toast = useUi((s) => s.toast);
  return (
    <button
      className="btn btn-sm"
      onClick={async () => {
        try {
          const losses = await ipc.blockConvertPreview(blockId, kind);
          if (
            losses.length > 0 &&
            !window.confirm(
              `Converting to ${KIND_LABEL[kind]} will discard ${losses.join(" and ")}.\n\nContinue?`,
            )
          )
            return;
          await ipc.blockConvert(blockId, kind);
        } catch (e) {
          toast({ tone: "error", message: ipc.asAppError(e).message });
        }
      }}
    >
      {KIND_LABEL[kind]}
    </button>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between gap-2">
      <dt>{label}</dt>
      <dd className="text-ink">{value}</dd>
    </div>
  );
}
