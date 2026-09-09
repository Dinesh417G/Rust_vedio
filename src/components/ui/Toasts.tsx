import { CheckCircle2, Info, TriangleAlert, X } from "lucide-react";
import { useUi } from "@/store/ui";
import { cx } from "@/lib/format";

export function Toasts() {
  const toasts = useUi((s) => s.toasts);
  const dismiss = useUi((s) => s.dismissToast);
  if (toasts.length === 0) return null;

  return (
    <div className="pointer-events-none fixed bottom-4 right-4 z-[60] flex w-80 flex-col gap-2">
      {toasts.map((t) => {
        const Icon =
          t.tone === "error" ? TriangleAlert : t.tone === "success" ? CheckCircle2 : Info;
        return (
          <div
            key={t.id}
            role="status"
            className={cx(
              "card pointer-events-auto flex items-start gap-3 px-3 py-2.5 shadow-xl",
              t.tone === "error" && "border-danger/50",
              t.tone === "success" && "border-ok/50",
            )}
          >
            <Icon
              size={16}
              className={cx(
                "mt-0.5 shrink-0",
                t.tone === "error" && "text-danger",
                t.tone === "success" && "text-ok",
                t.tone === "info" && "text-ink-muted",
              )}
            />
            <div className="min-w-0 flex-1 text-sm text-ink">
              <p className="break-words">{t.message}</p>
              {t.action && (
                <button
                  className="btn btn-ghost btn-sm mt-1 -ml-2"
                  onClick={() => {
                    t.action?.run();
                    dismiss(t.id);
                  }}
                >
                  {t.action.label}
                </button>
              )}
            </div>
            <button
              className="btn btn-ghost btn-sm -mr-1 -mt-1"
              onClick={() => dismiss(t.id)}
              aria-label="Dismiss"
            >
              <X size={14} />
            </button>
          </div>
        );
      })}
    </div>
  );
}
