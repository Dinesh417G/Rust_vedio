import { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import {
  BookOpen,
  Keyboard,
  Moon,
  PanelLeft,
  PanelRight,
  PenSquare,
  Search,
  Settings,
  Sun,
} from "lucide-react";
import * as ipc from "@/lib/ipc";
import { useCourse, useCourseProgress } from "@/lib/queries";
import { useUi } from "@/store/ui";
import { cx } from "@/lib/format";
import { ProgressRing } from "./ui/ProgressRing";
import { useAutosave } from "@/lib/autosave";

export function Titlebar() {
  const { data: course } = useCourse();
  const { data: progress } = useCourseProgress();
  const navigate = useNavigate();

  const mode = useUi((s) => s.mode);
  const toggleMode = useUi((s) => s.toggleMode);
  const theme = useUi((s) => s.theme);
  const toggleTheme = useUi((s) => s.toggleTheme);
  const toggleSidebar = useUi((s) => s.toggleSidebar);
  const collapsed = useUi((s) => s.sidebarCollapsed);
  const inspectorOpen = useUi((s) => s.inspectorOpen);
  const setInspectorOpen = useUi((s) => s.setInspectorOpen);
  const setPaletteOpen = useUi((s) => s.setPaletteOpen);
  const setShortcutsOpen = useUi((s) => s.setShortcutsOpen);

  const [title, setTitle] = useState(course?.title ?? "");
  const { schedule } = useAutosave<string>("", (v: string) =>
    ipc.courseUpdate({ title: v }),
  );

  useEffect(() => {
    setTitle(course?.title ?? "");
  }, [course?.title]);

  return (
    <header
      data-tauri-drag-region
      className="flex h-12 shrink-0 items-center gap-2 border-b border-line bg-surface-1 px-2"
    >
      <button
        className="btn btn-ghost btn-sm"
        onClick={toggleSidebar}
        title={collapsed ? "Show outline" : "Hide outline"}
        aria-label={collapsed ? "Show outline" : "Hide outline"}
      >
        <PanelLeft size={15} />
      </button>

      <Link to="/" className="shrink-0 rounded px-1 text-brand" title="Dashboard">
        <BookOpen size={16} />
      </Link>

      {mode === "author" ? (
        <input
          className="min-w-0 flex-1 rounded border border-transparent bg-transparent px-2 py-1 text-sm font-semibold text-ink hover:border-line focus:border-brand focus:bg-surface-2"
          value={title}
          aria-label="Course title"
          onChange={(e) => {
            setTitle(e.target.value);
            if (e.target.value.trim()) schedule(e.target.value);
          }}
        />
      ) : (
        <span className="min-w-0 flex-1 truncate px-2 text-sm font-semibold text-ink">
          {course?.title ?? "RustClass"}
        </span>
      )}

      <button
        className="btn btn-sm gap-2 text-ink-muted"
        onClick={() => setPaletteOpen(true)}
        title="Search (Ctrl+K)"
      >
        <Search size={14} />
        <span className="hidden sm:inline">Search</span>
        <kbd className="hidden rounded border border-line px-1 text-[10px] sm:inline">
          Ctrl K
        </kbd>
      </button>

      {/* Author / Learn toggle */}
      <div className="flex shrink-0 overflow-hidden rounded-lg border border-line">
        {(["author", "learn"] as const).map((m) => (
          <button
            key={m}
            onClick={() => mode !== m && toggleMode()}
            className={cx(
              "inline-flex items-center gap-1.5 px-2.5 py-1.5 text-xs font-medium transition-colors",
              mode === m ? "bg-brand text-white" : "text-ink-muted hover:bg-surface-2",
            )}
            aria-pressed={mode === m}
          >
            {m === "author" ? <PenSquare size={13} /> : <BookOpen size={13} />}
            {m === "author" ? "Author" : "Learn"}
          </button>
        ))}
      </div>

      <ProgressRing
        percent={progress?.percent ?? 0}
        label={`${progress?.completedBlocks ?? 0} of ${progress?.totalBlocks ?? 0} blocks complete`}
      />

      {mode === "author" && (
        <button
          className="btn btn-ghost btn-sm"
          onClick={() => setInspectorOpen(!inspectorOpen)}
          title={inspectorOpen ? "Hide inspector" : "Show inspector"}
        >
          <PanelRight size={15} />
        </button>
      )}

      <button
        className="btn btn-ghost btn-sm"
        onClick={() => setShortcutsOpen(true)}
        title="Keyboard shortcuts (?)"
      >
        <Keyboard size={15} />
      </button>

      <button
        className="btn btn-ghost btn-sm"
        onClick={toggleTheme}
        title={theme === "dark" ? "Switch to light theme" : "Switch to dark theme"}
      >
        {theme === "dark" ? <Sun size={15} /> : <Moon size={15} />}
      </button>

      <button
        className="btn btn-ghost btn-sm"
        onClick={() => navigate("/settings")}
        title="Settings"
      >
        <Settings size={15} />
      </button>
    </header>
  );
}
