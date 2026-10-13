import { useEffect, useState } from "react";
import { Code2, Workflow } from "lucide-react";
import { cx } from "@/lib/format";
import { MermaidView } from "./MermaidView";

/** The only languages this course needs. Loading more costs startup time. */
export const CODE_LANGS = ["rust", "toml", "bash", "json", "text", "mermaid"] as const;
export type CodeLang = (typeof CODE_LANGS)[number];

/** Fence tags people actually type, mapped onto the languages above. */
const ALIASES = new Map<string, CodeLang>([
  ["rs", "rust"],
  ["sh", "bash"],
  ["shell", "bash"],
  ["zsh", "bash"],
  ["jsonc", "json"],
]);

export function normalizeLang(lang: string | null | undefined): CodeLang {
  const l = (lang ?? "rust").toLowerCase();
  const alias = ALIASES.get(l);
  if (alias) return alias;
  return (CODE_LANGS as readonly string[]).includes(l) ? (l as CodeLang) : "text";
}

type Highlighter = { codeToHtml: (code: string, lang: string) => string };

/**
 * Shiki, loaded lazily and only once.
 *
 * The grammars and themes are imported by name rather than through the
 * batteries-included entry point: that entry pulls in every language Shiki
 * ships and the Oniguruma WASM binary, which is several megabytes for the four
 * grammars this app actually uses. The JavaScript regex engine handles all four.
 *
 * Until the highlighter resolves the code shows in a plain `<pre>` — no layout
 * shift, just colour arriving a moment later.
 */
let highlighterPromise: Promise<Highlighter> | null = null;

async function getHighlighter(): Promise<Highlighter> {
  if (!highlighterPromise) {
    highlighterPromise = (async () => {
      const [{ createHighlighterCore }, { createJavaScriptRegexEngine }] = await Promise.all([
        import("shiki/core"),
        import("shiki/engine/javascript"),
      ]);
      const [dark, light, rust, toml, bash, json] = await Promise.all([
        import("shiki/themes/github-dark-dimmed.mjs"),
        import("shiki/themes/github-light.mjs"),
        import("shiki/langs/rust.mjs"),
        import("shiki/langs/toml.mjs"),
        import("shiki/langs/bash.mjs"),
        import("shiki/langs/json.mjs"),
      ]);
      const core = await createHighlighterCore({
        themes: [dark.default, light.default],
        langs: [rust.default, toml.default, bash.default, json.default],
        engine: createJavaScriptRegexEngine(),
      });
      return {
        codeToHtml: (code: string, lang: string) =>
          core.codeToHtml(code, {
            lang,
            themes: { light: "github-light", dark: "github-dark-dimmed" },
            defaultColor: false,
          }),
      };
    })();
  }
  return highlighterPromise;
}

/** Highlighted code, or a diagram for `mermaid` with a switch to its source. */
export function CodeView({
  code,
  lang,
  className,
}: {
  code: string;
  lang: string | null | undefined;
  className?: string;
}) {
  if (normalizeLang(lang) === "mermaid") {
    return <DiagramView code={code} className={className} />;
  }
  return <HighlightedCode code={code} lang={lang} className={className} />;
}

function DiagramView({ code, className }: { code: string; className?: string }) {
  const [showSource, setShowSource] = useState(false);
  return (
    <div className={className}>
      {showSource ? <HighlightedCode code={code} lang="text" /> : <MermaidView code={code} />}
      <div className="mt-1 flex justify-end">
        <button
          type="button"
          className="btn btn-ghost btn-sm"
          onClick={() => setShowSource((v) => !v)}
        >
          {showSource ? <Workflow size={13} /> : <Code2 size={13} />}
          {showSource ? "Show diagram" : "Show source"}
        </button>
      </div>
    </div>
  );
}

function HighlightedCode({
  code,
  lang,
  className,
}: {
  code: string;
  lang: string | null | undefined;
  className?: string;
}) {
  const [html, setHtml] = useState<string | null>(null);
  const normalized = normalizeLang(lang);

  useEffect(() => {
    // `text` has no grammar to apply; the plain fallback is already correct.
    if (normalized === "text") {
      setHtml(null);
      return;
    }
    let live = true;
    getHighlighter()
      .then((hl) => {
        if (live) setHtml(hl.codeToHtml(code, normalized));
      })
      .catch(() => {
        // Highlighting is decoration; never let it break the page.
        if (live) setHtml(null);
      });
    return () => {
      live = false;
    };
  }, [code, normalized]);

  return (
    <div
      className={cx(
        "overflow-x-auto rounded-lg border border-line bg-surface-2 text-[13px] leading-relaxed",
        "[&_pre]:!bg-transparent [&_pre]:p-3 [&_code]:font-mono",
        // The highlighter sets two colour variables per token, one per theme,
        // and leaves choosing between them to the page.
        "[&_.shiki_span]:[color:var(--shiki-light)] dark:[&_.shiki_span]:[color:var(--shiki-dark)]",
        className,
      )}
    >
      {html ? (
        <div dangerouslySetInnerHTML={{ __html: html }} />
      ) : (
        <pre className="p-3 font-mono">
          <code>{code}</code>
        </pre>
      )}
    </div>
  );
}
