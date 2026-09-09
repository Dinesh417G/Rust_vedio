import type { BlockKind } from "@/lib/types";
import type { BlockRendererProps } from "./common";
import { AttachmentBlock } from "./AttachmentBlock";
import { CalloutBlock } from "./CalloutBlock";
import { CodeBlock } from "./CodeBlock";
import { DividerBlock } from "./DividerBlock";
import { ImageBlock } from "./ImageBlock";
import { QuizBlock } from "./QuizBlock";
import { TextBlock } from "./TextBlock";
import { VideoBlock } from "./VideoBlock";

/**
 * The registry is exhaustive by type: adding a `BlockKind` without adding a
 * renderer here is a compile error, and every renderer goes through
 * `BlockShell`, which in turn demands all four visual states.
 */
export const RENDERERS: Record<BlockKind, (p: BlockRendererProps) => JSX.Element> = {
  video: VideoBlock,
  text: TextBlock,
  image: ImageBlock,
  code: CodeBlock,
  callout: CalloutBlock,
  attachment: AttachmentBlock,
  quiz: QuizBlock,
  divider: DividerBlock,
};

export function BlockRenderer(props: BlockRendererProps) {
  const Component = RENDERERS[props.block.kind];
  return <Component {...props} />;
}

export * from "./common";
