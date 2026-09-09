import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeSanitize, { defaultSchema } from "rehype-sanitize";
import { cx } from "@/lib/format";

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
      <ReactMarkdown remarkPlugins={[remarkGfm]} rehypePlugins={[[rehypeSanitize, schema]]}>
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
