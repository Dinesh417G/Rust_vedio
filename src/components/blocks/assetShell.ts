import { useImports } from "@/store/imports";
import type { BlockRendererProps } from "./common";
import type { ImportingInfo, MissingInfo } from "../BlockShell";

/**
 * The `importing` and `missing` nodes every asset-backed block needs, built
 * once so video, image and attachment cannot drift apart.
 *
 * This is a hook by React's rules (it reads a store), and is only ever called
 * unconditionally at the top of a renderer.
 */
export function useAssetShellExtras(p: BlockRendererProps): {
  importing?: ImportingInfo;
  missing?: MissingInfo;
} {
  const active = useImports((s) =>
    Object.values(s.byTicket).find((i) => i.blockId === p.block.id),
  );

  const importing: ImportingInfo | undefined =
    p.block.state === "importing"
      ? {
          originalName: active?.originalName ?? "",
          bytesDone: active?.bytesDone ?? 0,
          bytesTotal: active?.bytesTotal ?? 0,
          pct: active?.pct ?? 0,
          onCancel: p.onCancelImport,
        }
      : undefined;

  const missing: MissingInfo | undefined =
    p.block.state === "missing"
      ? {
          expectedPath: p.block.asset?.resolvedPath ?? "(unknown path)",
          onLocate: p.onLocate,
          onRemove: p.onDetach,
          onReveal: p.onReveal,
        }
      : undefined;

  return { importing, missing };
}
