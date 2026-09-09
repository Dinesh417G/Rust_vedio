import { BlockShell } from "../BlockShell";
import type { BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";

/**
 * A divider has no payload, so it is always `Ready`. The shell still demands an
 * `empty` node; the honest one is a rule that looks the same either way.
 */
export function DividerBlock(props: BlockRendererProps) {
  const rule = <hr className="my-2 border-t border-line" />;
  return <BlockShell {...shellProps(props)} ready={rule} empty={rule} />;
}
