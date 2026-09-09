import { useUi } from "@/store/ui";
import { Modal } from "./ui/Modal";

const GROUPS: { title: string; items: [string, string][] }[] = [
  {
    title: "Anywhere",
    items: [
      ["Ctrl K", "Search the course"],
      ["Ctrl S", "Save everything pending"],
      ["Ctrl E", "Switch between Author and Learn"],
      ["Ctrl B", "Show or hide the outline"],
      ["?", "This sheet"],
      ["Esc", "Close whatever is open"],
    ],
  },
  {
    title: "Reading a chapter",
    items: [
      ["Alt ←", "Previous chapter"],
      ["Alt →", "Next chapter"],
    ],
  },
  {
    title: "Video player",
    items: [
      ["Space", "Play or pause"],
      ["K", "Pause"],
      ["← / →", "Back or forward 5 seconds"],
      ["J / L", "Back or forward 10 seconds"],
      [", / .", "Step one frame"],
      ["Shift , / Shift .", "Slower or faster"],
      ["↑ / ↓", "Volume"],
      ["M", "Mute"],
      ["F", "Fullscreen"],
      ["0 – 9", "Jump to that percent"],
    ],
  },
];

export function ShortcutSheet() {
  const open = useUi((s) => s.shortcutsOpen);
  const setOpen = useUi((s) => s.setShortcutsOpen);

  return (
    <Modal
      open={open}
      onClose={() => setOpen(false)}
      title="Keyboard shortcuts"
      width="lg"
      footer={
        <button className="btn" onClick={() => setOpen(false)}>
          Close
        </button>
      }
    >
      <div className="grid gap-6 sm:grid-cols-2">
        {GROUPS.map((g) => (
          <section key={g.title}>
            <h3 className="mb-2 text-xs font-semibold uppercase tracking-wide text-ink-faint">
              {g.title}
            </h3>
            <dl className="space-y-1.5">
              {g.items.map(([keys, what]) => (
                <div key={keys} className="flex items-baseline justify-between gap-3">
                  <dt className="text-sm text-ink-muted">{what}</dt>
                  <dd className="shrink-0">
                    {keys.split(" ").map((k) => (
                      <kbd
                        key={k}
                        className="ml-1 rounded border border-line bg-surface-2 px-1.5 py-0.5 font-mono text-[11px] text-ink"
                      >
                        {k}
                      </kbd>
                    ))}
                  </dd>
                </div>
              ))}
            </dl>
          </section>
        ))}
      </div>
    </Modal>
  );
}
