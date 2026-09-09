import type { ReactNode } from "react";
import type {
  BlockKind,
  BlockPatch,
  BlockView,
  CalloutMeta,
  ImageMeta,
  QuizMeta,
} from "@/lib/types";

/** Everything a block renderer is handed. Identical for all eight kinds. */
export interface BlockRendererProps {
  block: BlockView;
  authoring: boolean;
  selected: boolean;
  flash: boolean;
  toolbar?: ReactNode;
  /** Author mode: open the picker or the inline editor for this block. */
  onActivate: () => void;
  /** Learn mode: explain what is coming rather than doing nothing. */
  onLearnEmpty: () => void;
  onPatch: (patch: BlockPatch) => void;
  /** Missing-state actions. */
  onLocate: () => void;
  onDetach: () => void;
  onReveal: () => void;
  onCancelImport: () => void;
  /** Highlighted because a file is hovering over it. */
  isDropTarget?: boolean;
}

/** Reads a block's `metaJson` without ever throwing on malformed input. */
export function readMeta<T extends object>(json: string, fallback: T): T {
  try {
    const parsed = JSON.parse(json);
    return parsed && typeof parsed === "object" ? { ...fallback, ...parsed } : fallback;
  } catch {
    return fallback;
  }
}

export const quizMeta = (b: BlockView): QuizMeta =>
  readMeta<QuizMeta>(b.metaJson, { questions: [] });
export const calloutMeta = (b: BlockView): CalloutMeta =>
  readMeta<CalloutMeta>(b.metaJson, { tone: "note" });
export const imageMeta = (b: BlockView): ImageMeta =>
  readMeta<ImageMeta>(b.metaJson, { fit: "contain" });

/** The label shown in palettes, menus and the inspector. */
export const KIND_LABEL: Record<BlockKind, string> = {
  video: "Video",
  text: "Text",
  image: "Image",
  code: "Code",
  callout: "Callout",
  attachment: "Attachment",
  quiz: "Quiz",
  divider: "Divider",
};

export const KIND_BLURB: Record<BlockKind, string> = {
  video: "A recorded lesson",
  text: "Prose, in markdown",
  image: "A diagram, screenshot or figure",
  code: "A highlighted code sample",
  callout: "A note, tip or warning",
  attachment: "A file to download",
  quiz: "Questions to check understanding",
  divider: "A horizontal rule",
};
