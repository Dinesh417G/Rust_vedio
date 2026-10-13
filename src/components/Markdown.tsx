import { Children, isValidElement, type ReactNode } from "react";
import ReactMarkdown, { type Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeSanitize, { defaultSchema } from "rehype-sanitize";
import { cx } from "@/lib/format";
import { CodeView } from "./CodeView";
import { MermaidView } from "./MermaidView";

/**
 * Content is local and authored by the person using the app, so this is not a
 * cross-site defence — it is a "a pasted snippet must never execute" defence.
 * The sanitiser stays on regardless.
 */
const schema = {
  ...defaultSchema,
  attributes: {
    ...defaultSchema.attributes,
    code: [...(defaultSchema.attributes?.code ?? []), ["className"]],
  },
};

/**
 * The language and text of a fenced block. react-markdown renders a fence as
 * `<pre><code class="language-x">text</code></pre>`, so the `pre` override reads
 * both back out of the `code` element it was handed.
 */
function fence(children: ReactNode): { lang: string | null; text: string } | null {
  const only = Children.toArray(children)[0];
  if (!isValidElement<{ className?: string; children?: ReactNode }>(only)) return null;
  const { className, children: inner } = only.props;
  // The info string can carry more than the language: ```rust,ignore
  const tag = /(?:^|\s)language-([^\s,{]+)/.exec(className ?? "")?.[1] ?? null;
  const text = Children.toArray(inner).join("");
  return { lang: tag?.toLowerCase() ?? null, text: text.replace(/\n$/, "") };
}

/**
 * Module-level on purpose: a `components` object (or a component inside it)
 * created during render would be a new component type every render, and React
 * would tear down every diagram on the page each time the parent re-rendered.
 */
const components: Components = {
  pre: ({ children }) => {
    const block = fence(children);
    if (!block) return <pre>{children}</pre>;
    if (block.lang === "mermaid") return <MermaidView code={block.text} />;
    // A fence with no language is plain text, not Rust.
    return <CodeView code={block.text} lang={block.lang ?? "text"} />;
  },
};

export function Markdown({
  children,
  className,
}: {
  children: string | null | undefined;
  className?: string;
}) {
  if (!children?.trim()) return null;
  return (
    <div className={cx("prose-rc", className)}>
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[[rehypeSanitize, schema]]}
        components={components}
      >
        {children}
      </ReactMarkdown>
    </div>
  );
}

/** Single-line markdown, for captions and list rows. */
export function InlineMarkdown({
  children,
  className,
}: {
  children: string | null | undefined;
  className?: string;
}) {
  if (!children?.trim()) return null;
  return (
    <span className={cx("prose-rc [&>*]:!mt-0 [&_p]:inline", className)}>
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[[rehypeSanitize, schema]]}
        components={{ p: ({ children }) => <>{children}</> }}
      >
        {children}
      </ReactMarkdown>
    </span>
  );
}
