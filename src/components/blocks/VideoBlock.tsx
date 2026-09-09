import { Film } from "lucide-react";
import { BlockShell, EmptyCard } from "../BlockShell";
import { VideoPlayer } from "../VideoPlayer";
import type { BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";
import { useAssetShellExtras } from "./assetShell";

export function VideoBlock(props: BlockRendererProps) {
  const { block, authoring, onActivate, onLearnEmpty, onReveal, isDropTarget } = props;

  return (
    <BlockShell
      {...shellProps(props)}
      {...useAssetShellExtras(props)}
      ready={
        block.asset ? (
          <VideoPlayer
            block={block}
            asset={block.asset}
            authoring={authoring}
            onReveal={onReveal}
          />
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
