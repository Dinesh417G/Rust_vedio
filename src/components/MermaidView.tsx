import { useEffect, useState } from "react";
import type { Mermaid } from "mermaid";
import { useUi } from "@/store/ui";
import { cx } from "@/lib/format";

/**
 * Mermaid, fetched on first use and then kept.
 *
 * The library and its diagram types come to several megabytes and most chapters
 * have no diagram, so it is only ever reached through this dynamic import (the
 * `import type` above is erased). Vite then gives it chunks of its own and the
 * entry chunk stays free of it.
 */
let mermaidPromise: Promise<Mermaid> | null = null;

function loadMermaid(): Promise<Mermaid> {
  if (!mermaidPromise) {
    const loading = import("mermaid").then((m) => m.default);
    // A failed fetch must not be cached; the next diagram gets to try again.
    loading.catch(() => {
      if (mermaidPromise === loading) mermaidPromise = null;
    });
    mermaidPromise = loading;
  }
  return mermaidPromise;
}

let chain: Promise<unknown> = Promise.resolve();
let nextId = 0;

/**
 * `initialize` sets global state that `render` then reads. A theme change
 * redraws every diagram on the page, so the pair runs as one step and the steps
 * run one at a time: each diagram is drawn with the theme it asked for.
 */
function draw(source: string, dark: boolean): Promise<string> {
  // Fresh id per draw: Mermaid scopes its CSS and arrow markers to it, and the
  // previous SVG may still be on screen while the new one is being made.
  const id = `rc-mermaid-${++nextId}`;
  const step = async () => {
    const mermaid = await loadMermaid();
    mermaid.initialize({
      startOnLoad: false,
      securityLevel: "strict",
      theme: dark ? "dark" : "default",
      // Throw on a parse error instead of drawing Mermaid's own error graphic.
      suppressErrorRendering: true,
    });
    try {
      const { svg } = await mermaid.render(id, source);
      return svg;
    } finally {
      // Mermaid measures inside a scratch element in <body>; make sure it is gone.
      document.getElementById(`d${id}`)?.remove();
    }
  };
  const result = chain.then(step);
  chain = result.catch(() => undefined);
  return result;
}

/** Mermaid's messages run to several lines with a caret diagram; one line is enough here. */
function oneLine(e: unknown): string {
  const text = e instanceof Error ? e.message : String(e);
  const first = text.split("\n").find((l) => l.trim()) ?? "unknown error";
  return first.length > 160 ? `${first.slice(0, 157)}…` : first;
}

/**
 * Draws a Mermaid diagram and follows the app theme.
 *
 * It never throws and is never blank: while the library loads there is a
 * placeholder, and if the source does not parse the source itself is shown with
 * the reason on one line.
 */
export function MermaidView({ code, className }: { code: string; className?: string }) {
  const dark = useUi((s) => s.theme) === "dark";
  const [svg, setSvg] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const source = code.trim();

  useEffect(() => {
    if (!source) {
      setSvg(null);
      setError(null);
      return;
    }
    let live = true;
    // The old SVG stays up until the new one is ready, so a theme change does not flash.
    draw(source, dark).then(
      (out) => {
        if (!live) return;
        setSvg(out);
        setError(null);
      },
      (e: unknown) => {
        if (!live) return;
        setSvg(null);
        setError(oneLine(e));
      },
    );
    return () => {
      live = false;
    };
  }, [source, dark]);

  if (!source) {
    return (
      <p className={cx("text-sm italic text-ink-faint", className)}>This diagram is empty.</p>
    );
  }

  if (error) {
    return (
      <div className={cx("overflow-hidden rounded-lg border border-warn/40 bg-surface-2", className)}>
        <p className="border-b border-warn/40 bg-warn/10 px-3 py-1.5 text-xs text-ink">
          This diagram could not be drawn: {error}
        </p>
        <pre className="overflow-x-auto p-3 font-mono text-[13px] leading-relaxed">
          <code>{source}</code>
        </pre>
      </div>
    );
  }

  return (
    <div
      role="img"
      aria-label="Diagram"
      aria-busy={svg === null}
      className={cx(
        "overflow-x-auto rounded-lg border border-line bg-surface-2 p-3",
        svg === null && "min-h-[4rem]",
        className,
      )}
    >
      {svg === null ? (
        <span className="block py-3 text-center text-xs text-ink-faint">Drawing diagram…</span>
      ) : (
        // Mermaid sizes its SVG as `width: 100%` capped by `max-width`, so the
        // parent has to be a full-width block for the percentage to mean anything.
        <div
          className="[&_svg]:mx-auto [&_svg]:block [&_svg]:h-auto [&_svg]:max-w-full"
          dangerouslySetInnerHTML={{ __html: svg }}
        />
      )}
    </div>
  );
}
