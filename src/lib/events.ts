/**
 * Bridges Tauri events into TanStack Query and the import store.
 *
 * Mounted once, at the app root. Everything that changes data on the Rust side
 * emits `data://changed`, so no component needs to know what else to refresh.
 */
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useQueryClient } from "@tanstack/react-query";
import { invalidateFor } from "./queries";
import { useImports } from "@/store/imports";
import type * as T from "./types";

export function useTauriEvents() {
  const qc = useQueryClient();
  const onProgress = useImports((s) => s.onProgress);
  const onDone = useImports((s) => s.onDone);
  const onFailed = useImports((s) => s.onFailed);

  useEffect(() => {
    const unlisteners: Array<() => void> = [];
    let cancelled = false;

    const register = <P,>(name: string, handler: (payload: P) => void) => {
      listen<P>(name, (e) => handler(e.payload)).then((un) => {
        if (cancelled) un();
        else unlisteners.push(un);
      });
    };

    register<T.DataChanged>("data://changed", (p) => invalidateFor(qc, p.entity));
    register<T.ImportProgress>("import://progress", onProgress);
    register<T.ImportDone>("import://done", onDone);
    register<T.ImportFailed>("import://failed", onFailed);

    return () => {
      cancelled = true;
      unlisteners.forEach((un) => un());
    };
  }, [qc, onProgress, onDone, onFailed]);
}
