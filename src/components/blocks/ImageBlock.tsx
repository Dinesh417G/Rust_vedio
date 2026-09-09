import { ImageIcon } from "lucide-react";
import { BlockShell, EmptyCard } from "../BlockShell";
import { mediaUrl } from "@/lib/assetSrc";
import { cx } from "@/lib/format";
import { imageMeta, type BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";
import { useAssetShellExtras } from "./assetShell";

export function ImageBlock(props: BlockRendererProps) {
  const { block, authoring, onActivate, onLearnEmpty, onPatch, isDropTarget } = props;
  const url = mediaUrl(block.asset);
  const fit = imageMeta(block).fit ?? "contain";

  return (
    <BlockShell
      {...shellProps(props)}
      {...useAssetShellExtras(props)}
      ready={
        <figure className="space-y-2">
          <div
            className={cx(
              "overflow-hidden rounded-xl border border-line bg-surface-2",
              fit === "contain" ? "p-2" : "",
            )}
          >
            {url ? (
              <img
                src={url}
                alt={block.heading ?? block.asset?.originalName ?? "Course image"}
                className={cx(
                  "mx-auto max-h-[70vh] w-full rounded-lg",
                  fit === "cover" ? "object-cover" : "object-contain",
                )}
              />
            ) : (
              <p className="p-6 text-center text-sm text-ink-muted">
                This image could not be loaded.
              </p>
            )}
          </div>
          {authoring && (
            <div className="flex gap-1">
              {(["contain", "cover"] as const).map((f) => (
                <button
                  key={f}
                  className={cx("btn btn-sm", fit === f && "border-brand text-brand")}
                  onClick={() =>
                    onPatch({ metaJson: JSON.stringify({ ...imageMeta(block), fit: f }) })
                  }
                >
                  {f === "contain" ? "Fit" : "Fill"}
                </button>
              ))}
            </div>
          )}
        </figure>
      }
      empty={
        <EmptyCard
          authoring={authoring}
          icon={<ImageIcon size={26} />}
          title={authoring ? "No image yet" : "Image coming soon"}
          subline={
            authoring
              ? "Add a diagram, screenshot or figure."
              : "This figure hasn't been uploaded yet."
          }
          captionMd={block.captionMd}
          action={authoring ? { label: "Choose image…", run: onActivate } : undefined}
          dropHint="or drop a file here"
          onActivate={authoring ? onActivate : onLearnEmpty}
          isDropTarget={isDropTarget}
        />
      }
    />
  );
}
