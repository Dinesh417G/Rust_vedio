/**
 * In-flight imports, keyed by block so a block renderer can show its own
 * progress bar without prop drilling. Fed entirely by Tauri events.
 */
import { create } from "zustand";
import type { ImportDone, ImportFailed, ImportProgress } from "@/lib/types";

export interface ActiveImport {
  ticketId: string;
  blockId: string | null;
  originalName: string;
  bytesDone: number;
  bytesTotal: number;
  pct: number;
  startedAt: number;
}

interface ImportsState {
  byTicket: Record<string, ActiveImport>;
  lastFailure: ImportFailed | null;
  start: (t: {
    ticketId: string;
    blockId: string | null;
    originalName: string;
    bytesTotal: number;
  }) => void;
  onProgress: (p: ImportProgress) => void;
  onDone: (d: ImportDone) => void;
  onFailed: (f: ImportFailed) => void;
  forBlock: (blockId: string) => ActiveImport | undefined;
  clearFailure: () => void;
}

export const useImports = create<ImportsState>((set, get) => ({
  byTicket: {},
  lastFailure: null,

  start: (t) =>
    set((s) => ({
      byTicket: {
        ...s.byTicket,
        [t.ticketId]: {
          ...t,
          bytesDone: 0,
          pct: 0,
          startedAt: Date.now(),
        },
      },
    })),

  onProgress: (p) =>
    set((s) => {
      const existing = s.byTicket[p.ticketId];
      // A progress event can arrive before the invoke() promise resolves.
      const base: ActiveImport = existing ?? {
        ticketId: p.ticketId,
        blockId: p.blockId,
        originalName: "",
        bytesDone: 0,
        bytesTotal: p.bytesTotal,
        pct: 0,
        startedAt: Date.now(),
      };
      return {
        byTicket: {
          ...s.byTicket,
          [p.ticketId]: {
            ...base,
            blockId: p.blockId ?? base.blockId,
            bytesDone: p.bytesDone,
            bytesTotal: p.bytesTotal || base.bytesTotal,
            pct: p.pct,
          },
        },
      };
    }),

  onDone: (d) =>
    set((s) => {
      const { [d.ticketId]: _gone, ...rest } = s.byTicket;
      return { byTicket: rest };
    }),

  onFailed: (f) =>
    set((s) => {
      const { [f.ticketId]: _gone, ...rest } = s.byTicket;
      return { byTicket: rest, lastFailure: f };
    }),

  forBlock: (blockId) => Object.values(get().byTicket).find((i) => i.blockId === blockId),
  clearFailure: () => set({ lastFailure: null }),
}));
