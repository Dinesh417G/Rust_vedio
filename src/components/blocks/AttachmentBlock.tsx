import { Download, FolderOpen, Paperclip } from "lucide-react";
import { BlockShell, EmptyCard } from "../BlockShell";
import { humanBytes } from "@/lib/format";
import * as ipc from "@/lib/ipc";
import { useUi } from "@/store/ui";
import type { BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";
import { useAssetShellExtras } from "./assetShell";

export function AttachmentBlock(props: BlockRendererProps) {
  const { block, authoring, onActivate, onLearnEmpty, onReveal, isDropTarget } = props;
  const toast = useUi((s) => s.toast);
  const asset = block.asset;

  const open = async () => {
    if (!asset) return;
    try {
      await ipc.assetOpenExternal(asset.id);
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  return (
    <BlockShell
      {...shellProps(props)}
      {...useAssetShellExtras(props)}
      ready={
        <div className="card flex items-center gap-3 p-4">
          <div className="grid h-11 w-11 shrink-0 place-items-center rounded-lg bg-surface-2 text-ink-muted">
            <Paperclip size={18} />
          </div>
          <div className="min-w-0 flex-1">
            <p className="truncate text-sm font-medium text-ink">
              {asset?.originalName ?? "Attachment"}
            </p>
            <p className="text-xs text-ink-muted">
              {humanBytes(asset?.bytes ?? 0)}
              {asset?.mime ? ` · ${asset.mime}` : ""}
            </p>
          </div>
          <button className="btn btn-sm btn-primary" onClick={open}>
            <Download size={13} /> Open
          </button>
          <button className="btn btn-sm" onClick={onReveal} title="Show in folder">
            <FolderOpen size={13} />
          </button>
        </div>
      }
      empty={
        <EmptyCard
          authoring={authoring}
          icon={<Paperclip size={26} />}
          title={authoring ? "No file attached" : "Download coming soon"}
          subline={
            authoring
              ? "Attach the worksheet, dataset or slide deck for this section."
              : "The file for this section hasn't been uploaded yet."
          }
          captionMd={block.captionMd}
          action={authoring ? { label: "Attach file…", run: onActivate } : undefined}
          dropHint="or drop a file here"
          onActivate={authoring ? onActivate : onLearnEmpty}
          isDropTarget={isDropTarget}
        />
      }
    />
  );
}
