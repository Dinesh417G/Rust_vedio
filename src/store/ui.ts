/**
 * Pure UI state. Nothing here is course data — that lives in SQLite, always.
 * The few things persisted (theme, sidebar width, import mode) go through the
 * Tauri store plugin, not browser storage.
 */
import { create } from "zustand";
import { load, type Store } from "@tauri-apps/plugin-store";
import type { BlockKind, ImportMode } from "@/lib/types";

export type Mode = "author" | "learn";
export type Theme = "dark" | "light";

interface Toast {
  id: string;
  tone: "info" | "success" | "error";
  message: string;
  action?: { label: string; run: () => void };
}

interface UiState {
  mode: Mode;
  theme: Theme;
  sidebarWidth: number;
  sidebarCollapsed: boolean;
  inspectorOpen: boolean;
  selectedBlockId: string | null;
  expandedChapters: Set<string>;
  paletteOpen: boolean;
  shortcutsOpen: boolean;
  flashBlockId: string | null;
  defaultImportMode: ImportMode;
  lastBlockKind: BlockKind;
  toasts: Toast[];
  dismissedMissingBanner: boolean;

  setMode: (m: Mode) => void;
  toggleMode: () => void;
  setTheme: (t: Theme) => void;
  toggleTheme: () => void;
  setSidebarWidth: (w: number) => void;
  toggleSidebar: () => void;
  setInspectorOpen: (open: boolean) => void;
  selectBlock: (id: string | null) => void;
  toggleChapter: (id: string) => void;
  expandChapter: (id: string) => void;
  setPaletteOpen: (open: boolean) => void;
  setShortcutsOpen: (open: boolean) => void;
  flashBlock: (id: string | null) => void;
  setDefaultImportMode: (m: ImportMode) => void;
  setLastBlockKind: (k: BlockKind) => void;
  toast: (t: Omit<Toast, "id">) => void;
  dismissToast: (id: string) => void;
  dismissMissingBanner: () => void;
  hydrate: () => Promise<void>;
}

const SETTINGS_FILE = "settings.json";
let settings: Store | null = null;

async function settingsStore(): Promise<Store | null> {
  if (settings) return settings;
  try {
    settings = await load(SETTINGS_FILE, { autoSave: true });
    return settings;
  } catch {
    // Running outside Tauri (a plain `vite dev`); defaults are fine.
    return null;
  }
}

async function persist(key: string, value: unknown) {
  const s = await settingsStore();
  await s?.set(key, value);
}

export const useUi = create<UiState>((set, get) => ({
  mode: "author",
  theme: "dark",
  sidebarWidth: 300,
  sidebarCollapsed: false,
  inspectorOpen: true,
  selectedBlockId: null,
  expandedChapters: new Set<string>(),
  paletteOpen: false,
  shortcutsOpen: false,
  flashBlockId: null,
  defaultImportMode: "copy",
  lastBlockKind: "text",
  toasts: [],
  dismissedMissingBanner: false,

  setMode: (mode) => {
    set({ mode, selectedBlockId: mode === "learn" ? null : get().selectedBlockId });
    void persist("mode", mode);
  },
  toggleMode: () => get().setMode(get().mode === "author" ? "learn" : "author"),

  setTheme: (theme) => {
    set({ theme });
    document.documentElement.classList.toggle("dark", theme === "dark");
    void persist("theme", theme);
  },
  toggleTheme: () => get().setTheme(get().theme === "dark" ? "light" : "dark"),

  setSidebarWidth: (w) => {
    const width = Math.min(520, Math.max(220, Math.round(w)));
    set({ sidebarWidth: width });
    void persist("sidebarWidth", width);
  },
  toggleSidebar: () => {
    const next = !get().sidebarCollapsed;
    set({ sidebarCollapsed: next });
    void persist("sidebarCollapsed", next);
  },
  setInspectorOpen: (inspectorOpen) => {
    set({ inspectorOpen });
    void persist("inspectorOpen", inspectorOpen);
  },

  selectBlock: (selectedBlockId) => set({ selectedBlockId }),
  toggleChapter: (id) =>
    set((s) => {
      const next = new Set(s.expandedChapters);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return { expandedChapters: next };
    }),
  expandChapter: (id) =>
    set((s) => {
      if (s.expandedChapters.has(id)) return s;
      const next = new Set(s.expandedChapters);
      next.add(id);
      return { expandedChapters: next };
    }),

  setPaletteOpen: (paletteOpen) => set({ paletteOpen }),
  setShortcutsOpen: (shortcutsOpen) => set({ shortcutsOpen }),
  flashBlock: (flashBlockId) => {
    set({ flashBlockId });
    if (flashBlockId) {
      window.setTimeout(() => {
        if (useUi.getState().flashBlockId === flashBlockId) set({ flashBlockId: null });
      }, 1400);
    }
  },

  setDefaultImportMode: (m) => {
    set({ defaultImportMode: m });
    void persist("defaultImportMode", m);
  },
  setLastBlockKind: (k) => set({ lastBlockKind: k }),

  toast: (t) => {
    const id = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    set((s) => ({ toasts: [...s.toasts, { ...t, id }] }));
    window.setTimeout(() => get().dismissToast(id), t.tone === "error" ? 9000 : 4500);
  },
  dismissToast: (id) => set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })),
  dismissMissingBanner: () => set({ dismissedMissingBanner: true }),

  hydrate: async () => {
    const s = await settingsStore();
    if (!s) {
      document.documentElement.classList.add("dark");
      return;
    }
    const theme = ((await s.get<Theme>("theme")) ?? "dark") as Theme;
    const mode = ((await s.get<Mode>("mode")) ?? "author") as Mode;
    const width = (await s.get<number>("sidebarWidth")) ?? 300;
    const collapsed = (await s.get<boolean>("sidebarCollapsed")) ?? false;
    const inspector = (await s.get<boolean>("inspectorOpen")) ?? true;
    const importMode = ((await s.get<ImportMode>("defaultImportMode")) ??
      "copy") as ImportMode;
    document.documentElement.classList.toggle("dark", theme === "dark");
    set({
      theme,
      mode,
      sidebarWidth: width,
      sidebarCollapsed: collapsed,
      inspectorOpen: inspector,
      defaultImportMode: importMode,
    });
  },
}));
