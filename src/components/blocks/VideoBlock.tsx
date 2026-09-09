import { Film } from "lucide-react";
import { BlockShell, EmptyCard } from "../BlockShell";
import { VideoPlayer } from "../VideoPlayer";
import { useNearViewport } from "@/lib/useNearViewport";
import { thumbUrl } from "@/lib/assetSrc";
import type { BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";
import { useAssetShellExtras } from "./assetShell";

export function VideoBlock(props: BlockRendererProps) {
  const { block, authoring, onActivate, onLearnEmpty, onReveal, isDropTarget } = props;
  // A chapter can hold a hundred video blocks. Creating a hundred media
  // elements on open would fetch a hundred sets of metadata; the player mounts
  // when the block is nearly on screen and stays mounted after that.
  const [ref, near] = useNearViewport<HTMLDivElement>();
  const poster = thumbUrl(block.asset);

  return (
    <BlockShell
      {...shellProps(props)}
      {...useAssetShellExtras(props)}
      ready={
        block.asset ? (
          <div ref={ref}>
            {near ? (
              <VideoPlayer
                block={block}
                asset={block.asset}
                authoring={authoring}
                onReveal={onReveal}
              />
            ) : (
              // Same shape as the player, so nothing shifts when it mounts.
              <div className="aspect-video w-full overflow-hidden rounded-xl border border-line bg-black">
                {poster && (
                  <img
                    src={poster}
                    alt=""
                    className="h-full w-full object-cover opacity-70"
                  />
                )}
              </div>
            )}
          </div>
        ) : null
      }
      empty={
        <EmptyCard
          authoring={authoring}
          icon={<Film size={28} />}
          title={authoring ? "Video not uploaded yet" : "Video coming soon"}
          subline={
            authoring
              ? "Add the lesson video for this section."
              : "This lesson hasn't been uploaded yet."
          }
          captionMd={block.captionMd}
          action={authoring ? { label: "Choose video…", run: onActivate } : undefined}
          dropHint="or drop a file here"
          onActivate={authoring ? onActivate : onLearnEmpty}
          isDropTarget={isDropTarget}
        />
      }
    />
  );
}
