import type { ReactNode } from "react";
import {
  FileQuestion,
  FolderOpen,
  Loader2,
  Trash2,
  TriangleAlert,
  X,
} from "lucide-react";
import type { BlockKind, BlockState } from "@/lib/types";
import { cx, humanBytes } from "@/lib/format";
import { InlineMarkdown } from "./Markdown";
import { ProgressBar } from "./ui/ProgressRing";

/**
 * The architectural expression of the core requirement.
 *
 * Every block renderer goes through this wrapper, and the wrapper demands a
 * node for all four states. A new block kind cannot be added without deciding
 * what `Empty`, `Ready`, `Missing` and `Importing` look like, which is exactly
 * the mistake this app exists to prevent.
 */
export interface BlockShellProps {
  state: BlockState;
  kind: BlockKind;
  heading: string | null;
  captionMd: string | null;
  /** Author mode shows editing chrome; Learn mode shows none. */
  authoring: boolean;
  selected?: boolean;
  flash?: boolean;

  /** Payload present and usable. */
  ready: ReactNode;
  /** Declared but not filled in — the placeholder case. */
  empty: ReactNode;
  /** An import for this block is in flight. */
  importing?: ImportingInfo;
  /** Asset row exists but the file is gone. */
  missing?: MissingInfo;

  toolbar?: ReactNode;
  onClick?: () => void;
  children?: never;
}

export interface ImportingInfo {
  originalName: string;
  bytesDone: number;
  bytesTotal: number;
  pct: number;
  onCancel: () => void;
}

export interface MissingInfo {
  expectedPath: string;
  onLocate: () => void;
  onRemove: () => void;
  onReveal: () => void;
}

export function BlockShell(props: BlockShellProps) {
  const {
    state,
    heading,
    captionMd,
    authoring,
    selected,
    flash,
    ready,
    empty,
    importing,
    missing,
    toolbar,
    onClick,
  } = props;

  return (
    <section
      className={cx(
        "group relative rounded-xl transition-colors",
        selected && authoring && "ring-2 ring-brand/60",
        flash && "animate-flash",
      )}
      data-block-state={state}
      aria-busy={state === "importing"}
    >
      {(heading || (authoring && toolbar)) && (
        <header className="mb-2 flex items-start justify-between gap-3">
          {heading ? (
            <h3 className="text-lg font-semibold leading-snug text-ink">{heading}</h3>
          ) : (
            <span />
          )}
          {authoring && toolbar && (
            <div className="shrink-0 opacity-0 transition-opacity focus-within:opacity-100 group-hover:opacity-100">
              {toolbar}
            </div>
          )}
        </header>
      )}

      {/* The caption is shown in every state — it is what the block is about,
          and the whole point of planning before recording. */}
      {captionMd?.trim() && state !== "empty" && (
        <div className="mb-2 text-sm text-ink-muted">
          <InlineMarkdown>{captionMd}</InlineMarkdown>
        </div>
      )}

      <div onClick={onClick} className={onClick ? "cursor-pointer" : undefined}>
        {state === "ready" && ready}
        {state === "empty" && empty}
        {state === "importing" && importing && <ImportingCard {...importing} />}
        {state === "missing" && missing && <MissingCard {...missing} />}
        {/* A renderer that somehow reaches a state it did not describe still
            says something honest rather than rendering nothing. */}
        {state === "importing" && !importing && <FallbackCard label="Importing…" />}
        {state === "missing" && !missing && <FallbackCard label="File is missing" />}
      </div>
    </section>
  );
}

/**
 * The dashed placeholder every `Empty` block shares. Kinds supply their own
 * icon, title, sub-line and action; the frame and behaviour stay identical so
 * a page of placeholders reads as one designed thing.
 */
export function EmptyCard({
  icon,
  title,
  subline,
  captionMd,
  action,
  dropHint,
  authoring,
  onActivate,
  isDropTarget,
}: {
  icon: ReactNode;
  title: string;
  subline: string;
  captionMd?: string | null;
  action?: { label: string; run: () => void };
  dropHint?: string;
  authoring: boolean;
  onActivate?: () => void;
  isDropTarget?: boolean;
}) {
  // Clickable in BOTH modes: Author mode opens the picker or the inline
  // editor, Learn mode opens the "not uploaded yet" modal. Gating this on
  // `authoring` would silently swallow the click a reader makes on a
  // placeholder, which is the one interaction the empty state exists for.
  const interactive = !!onActivate;
  return (
    <div
      role={interactive ? "button" : undefined}
      tabIndex={interactive ? 0 : undefined}
      onClick={interactive ? onActivate : undefined}
      onKeyDown={
        interactive
          ? (e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                onActivate?.();
              }
            }
          : undefined
      }
      className={cx(
        "flex flex-col items-center justify-center gap-2 rounded-xl border-2 border-dashed px-6 py-10 text-center transition-colors",
        authoring
          ? "border-line bg-surface-1/60 hover:border-brand/60 hover:bg-surface-2"
          : "border-line/70 bg-surface-1/30 opacity-80",
        interactive && "cursor-pointer",
        isDropTarget && "border-brand bg-brand/10",
      )}
    >
      <div className={cx("mb-1", authoring ? "text-ink-faint" : "text-ink-faint/70")}>
        {icon}
      </div>
      <p className="text-sm font-semibold text-ink">{title}</p>
      {/* The block's own caption wins over the generic sub-line: if the author
          wrote what this video is about, that is what should be read. */}
      {captionMd?.trim() ? (
        <div className="max-w-prose text-sm text-ink-muted">
          <InlineMarkdown>{captionMd}</InlineMarkdown>
        </div>
      ) : (
        <p className="max-w-prose text-sm text-ink-muted">{subline}</p>
      )}
      {authoring && action && (
        <button
          className="btn btn-primary mt-2"
          onClick={(e) => {
            e.stopPropagation();
            action.run();
          }}
        >
          {action.label}
        </button>
      )}
      {authoring && dropHint && (
        <p className="text-xs text-ink-faint">{dropHint}</p>
      )}
    </div>
  );
}

function ImportingCard({
  originalName,
  bytesDone,
  bytesTotal,
  pct,
  onCancel,
}: ImportingInfo) {
  return (
    // Same outer height as the Empty card, so the block does not jump when it
    // flips to Ready.
    <div className="flex flex-col justify-center gap-3 rounded-xl border-2 border-dashed border-brand/60 bg-brand/5 px-6 py-10">
      <div className="flex items-center gap-2 text-sm font-semibold text-ink">
        <Loader2 size={16} className="animate-spin text-brand" />
        <span className="truncate">Importing {originalName || "file"}…</span>
      </div>
      <ProgressBar percent={pct} className="h-1.5" />
      <div className="flex items-center justify-between text-xs text-ink-muted">
        <span className="tabular-nums">
          {humanBytes(bytesDone)} of {humanBytes(bytesTotal)} · {Math.round(pct)}%
        </span>
        <button className="btn btn-ghost btn-sm" onClick={onCancel}>
          <X size={13} /> Cancel
        </button>
      </div>
    </div>
  );
}

function MissingCard({ expectedPath, onLocate, onRemove, onReveal }: MissingInfo) {
  return (
    <div className="rounded-xl border-2 border-danger/50 bg-danger/10 px-5 py-6">
      <div className="flex items-center gap-2 text-sm font-semibold text-danger">
        <TriangleAlert size={16} />
        File is missing
      </div>
      <p className="mt-2 break-all rounded-lg bg-surface-2/70 px-2 py-1.5 font-mono text-xs text-ink-muted">
        {expectedPath}
      </p>
      <div className="mt-3 flex flex-wrap gap-2">
        <button className="btn btn-sm btn-primary" onClick={onLocate}>
          <FolderOpen size={13} /> Locate file…
        </button>
        <button className="btn btn-sm" onClick={onReveal}>
          Show in folder
        </button>
        <button className="btn btn-sm btn-ghost" onClick={onRemove}>
          <Trash2 size={13} /> Remove from block
        </button>
      </div>
    </div>
  );
}

function FallbackCard({ label }: { label: string }) {
  return (
    <div className="flex items-center gap-2 rounded-xl border border-line bg-surface-1 px-5 py-6 text-sm text-ink-muted">
      <FileQuestion size={16} /> {label}
    </div>
  );
}
