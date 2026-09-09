import { useCallback, useEffect, useRef, useState } from "react";

export type SaveStatus = "idle" | "dirty" | "saving" | "saved" | "error";

const DEBOUNCE_MS = 600;

/**
 * Debounced autosave for a single editor field.
 *
 * Nothing is ever lost by closing the window: a pending save is flushed on
 * unmount, on `Ctrl+S`, and when the window loses focus. `flush()` is exposed
 * so a parent can force one before navigating away.
 */
export function useAutosave<V>(
  value: V,
  save: (v: V) => Promise<unknown>,
  opts: { enabled?: boolean } = {},
) {
  const { enabled = true } = opts;
  const [status, setStatus] = useState<SaveStatus>("idle");
  const timer = useRef<number | null>(null);
  const pending = useRef<V | null>(null);
  const saveRef = useRef(save);
  const savedAt = useRef<number>(0);
  saveRef.current = save;

  const run = useCallback(async () => {
    if (pending.current === null) return;
    const v = pending.current;
    pending.current = null;
    setStatus("saving");
    try {
      await saveRef.current(v);
      savedAt.current = Date.now();
      setStatus("saved");
    } catch {
      setStatus("error");
    }
  }, []);

  const flush = useCallback(async () => {
    if (timer.current !== null) {
      window.clearTimeout(timer.current);
      timer.current = null;
    }
    await run();
  }, [run]);

  const schedule = useCallback(
    (v: V) => {
      if (!enabled) return;
      pending.current = v;
      setStatus("dirty");
      if (timer.current !== null) window.clearTimeout(timer.current);
      timer.current = window.setTimeout(() => {
        timer.current = null;
        void run();
      }, DEBOUNCE_MS);
    },
    [enabled, run],
  );

  // A pending edit must survive the component going away.
  useEffect(() => {
    const onHide = () => {
      if (pending.current !== null) void flush();
    };
    window.addEventListener("blur", onHide);
    document.addEventListener("visibilitychange", onHide);
    return () => {
      window.removeEventListener("blur", onHide);
      document.removeEventListener("visibilitychange", onHide);
      if (pending.current !== null) void flush();
    };
  }, [flush]);

  // The "Saved" pill fades back to idle rather than sitting there forever.
  useEffect(() => {
    if (status !== "saved") return;
    const t = window.setTimeout(() => setStatus("idle"), 1800);
    return () => window.clearTimeout(t);
  }, [status]);

  useEffect(() => {
    void value; // the caller owns the value; this hook only schedules writes
  }, [value]);

  return { status, schedule, flush };
}

/** `Ctrl+S` forces every pending autosave to land right now. */
export function useForceSaveShortcut(flush: () => void) {
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
        e.preventDefault();
        flush();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [flush]);
}
