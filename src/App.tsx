import { useEffect } from "react";
import { Navigate, Route, Routes, useNavigate, useParams } from "react-router-dom";
import * as ipc from "@/lib/ipc";
import { useTauriEvents } from "@/lib/events";
import { useChapters } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { useImports } from "@/store/imports";
import { Titlebar } from "@/components/Titlebar";
import { Sidebar } from "@/components/Sidebar";
import { Inspector } from "@/components/Inspector";
import { CommandPalette } from "@/components/CommandPalette";
import { ShortcutSheet } from "@/components/ShortcutSheet";
import { MissingBanner } from "@/components/MissingBanner";
import { Toasts } from "@/components/ui/Toasts";
import { OutlineImportDialog } from "@/components/OutlineImportDialog";
import { Dashboard } from "@/routes/Dashboard";
import { ChapterPage } from "@/routes/ChapterPage";
import { SearchPage } from "@/routes/SearchPage";
import { SettingsPage } from "@/routes/SettingsPage";
import { HealthPage } from "@/routes/HealthPage";
import { useState } from "react";

export default function App() {
  useTauriEvents();
  const hydrate = useUi((s) => s.hydrate);
  const toast = useUi((s) => s.toast);
  const lastFailure = useImports((s) => s.lastFailure);
  const clearFailure = useImports((s) => s.clearFailure);
  const [outlineOpen, setOutlineOpen] = useState(false);

  useEffect(() => {
    void hydrate();
    // Expired Trash is emptied once per launch, quietly.
    void ipc.trashSweep().catch(() => undefined);
  }, [hydrate]);

  // A failed import surfaces as a toast rather than a silent no-op.
  useEffect(() => {
    if (!lastFailure) return;
    toast({
      tone: lastFailure.code === "cancelled" ? "info" : "error",
      message:
        lastFailure.code === "cancelled"
          ? "Import cancelled. Nothing was saved."
          : lastFailure.message,
    });
    clearFailure();
  }, [lastFailure, toast, clearFailure]);

  useGlobalShortcuts();

  return (
    <div className="flex h-full flex-col">
      <Titlebar />
      <MissingBanner />
      <div className="flex min-h-0 flex-1">
        <Sidebar onImportOutline={() => setOutlineOpen(true)} />
        <main className="min-w-0 flex-1 bg-surface-0">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/chapter/:id" element={<ChapterPage />} />
            <Route path="/search" element={<SearchPage />} />
            <Route path="/settings" element={<SettingsPage />} />
            <Route path="/health" element={<HealthPage />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </main>
        <InspectorSlot />
      </div>

      <CommandPalette />
      <ShortcutSheet />
      <Toasts />
      <OutlineImportDialog open={outlineOpen} onClose={() => setOutlineOpen(false)} />
    </div>
  );
}

/** The inspector only exists in Author mode, and only on a chapter page. */
function InspectorSlot() {
  const mode = useUi((s) => s.mode);
  const { id } = useParams<{ id: string }>();
  // `useParams` is empty outside the matched route, so read the path directly.
  const chapterId = id ?? window.location.hash.match(/chapter\/([^/?#]+)/)?.[1];
  if (mode !== "author") return null;
  return <Inspector chapterId={chapterId} />;
}

function useGlobalShortcuts() {
  const setPaletteOpen = useUi((s) => s.setPaletteOpen);
  const setShortcutsOpen = useUi((s) => s.setShortcutsOpen);
  const toggleMode = useUi((s) => s.toggleMode);
  const toggleSidebar = useUi((s) => s.toggleSidebar);
  const { data: chapters = [] } = useChapters();
  const navigate = useNavigate();

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      const typing = !!target && /input|textarea|select/i.test(target.tagName);
      const mod = e.ctrlKey || e.metaKey;

      if (mod && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setPaletteOpen(true);
        return;
      }
      if (mod && e.key.toLowerCase() === "e") {
        e.preventDefault();
        toggleMode();
        return;
      }
      if (mod && e.key.toLowerCase() === "b") {
        e.preventDefault();
        toggleSidebar();
        return;
      }
      if (e.key === "?" && !typing) {
        e.preventDefault();
        setShortcutsOpen(true);
        return;
      }
      if (e.altKey && (e.key === "ArrowLeft" || e.key === "ArrowRight")) {
        const current = window.location.hash.match(/chapter\/([^/?#]+)/)?.[1];
        const i = chapters.findIndex((c) => c.id === current);
        if (i < 0) return;
        const next = e.key === "ArrowLeft" ? chapters[i - 1] : chapters[i + 1];
        if (next) {
          e.preventDefault();
          navigate(`/chapter/${next.id}`);
        }
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [chapters, navigate, setPaletteOpen, setShortcutsOpen, toggleMode, toggleSidebar]);
}
