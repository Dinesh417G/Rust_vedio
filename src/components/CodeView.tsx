import { useEffect, useState } from "react";
import { cx } from "@/lib/format";

/** The only languages this course needs. Loading more costs startup time. */
export const CODE_LANGS = ["rust", "toml", "bash", "json", "text"] as const;
export type CodeLang = (typeof CODE_LANGS)[number];

export function normalizeLang(lang: string | null | undefined): CodeLang {
  const l = (lang ?? "rust").toLowerCase();
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

export function CodeView({
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
