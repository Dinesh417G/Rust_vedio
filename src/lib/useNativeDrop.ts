import { useEffect, useState } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

export interface DropPayload {
  paths: string[];
  position: { x: number; y: number };
}

/**
 * Native window drag-and-drop.
 *
 * HTML5 `ondrop` does not hand a webview real filesystem paths, so the only
 * way to accept a dropped file is Tauri's own drag-drop event. `position` is
 * in physical pixels; callers convert with `devicePixelRatio` to find the
 * element under the cursor.
 */
export function useNativeDrop(handlers: {
  onOver?: (p: { x: number; y: number }) => void;
  onDrop?: (p: DropPayload) => void;
  onLeave?: () => void;
}) {
  const [available, setAvailable] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | null = null;
    let cancelled = false;

    getCurrentWebview()
      .onDragDropEvent((event) => {
        const p = event.payload;
        if (p.type === "over") handlers.onOver?.(p.position);
        else if (p.type === "drop")
          handlers.onDrop?.({ paths: p.paths, position: p.position });
        else handlers.onLeave?.();
      })
      .then((un) => {
        if (cancelled) un();
        else {
          unlisten = un;
          setAvailable(true);
        }
      })
      .catch(() => {
        // Running outside a Tauri window; the file picker still works.
        setAvailable(false);
      });

    return () => {
      cancelled = true;
      unlisten?.();
    };
    // The handler object is recreated each render on purpose — the listener is
    // registered once and reads through the latest closure via `handlers`.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return available;
}

/** The block element under a physical-pixel cursor position, if any. */
export function blockIdAtPoint(position: { x: number; y: number }): string | null {
  const ratio = window.devicePixelRatio || 1;
  const el = document.elementFromPoint(position.x / ratio, position.y / ratio);
  const host = el?.closest<HTMLElement>("[id^='block-']");
  return host?.id.replace(/^block-/, "") ?? null;
}
