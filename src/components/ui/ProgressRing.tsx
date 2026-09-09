import { cx } from "@/lib/format";

interface Props {
  percent: number;
  size?: number;
  stroke?: number;
  className?: string;
  label?: string;
}

/** The overall-progress ring in the titlebar. */
export function ProgressRing({
  percent,
  size = 30,
  stroke = 3,
  className,
  label,
}: Props) {
  const pct = Math.max(0, Math.min(100, percent));
  const r = (size - stroke) / 2;
  const c = 2 * Math.PI * r;
  return (
    <div
      className={cx("relative shrink-0", className)}
      title={label ?? `${Math.round(pct)}% complete`}
      role="img"
      aria-label={label ?? `${Math.round(pct)} percent complete`}
    >
      <svg width={size} height={size} className="-rotate-90">
        <circle
          cx={size / 2}
          cy={size / 2}
          r={r}
          fill="none"
          strokeWidth={stroke}
          className="stroke-surface-3"
        />
        <circle
          cx={size / 2}
          cy={size / 2}
          r={r}
          fill="none"
          strokeWidth={stroke}
          strokeLinecap="round"
          strokeDasharray={c}
          strokeDashoffset={c * (1 - pct / 100)}
          className="stroke-brand transition-[stroke-dashoffset] duration-500"
        />
      </svg>
      <span className="absolute inset-0 grid place-items-center text-[9px] font-semibold tabular-nums text-ink-muted">
        {Math.round(pct)}
      </span>
    </div>
  );
}

export function ProgressBar({
  percent,
  className,
}: {
  percent: number;
  className?: string;
}) {
  const pct = Math.max(0, Math.min(100, percent));
  return (
    <div className={cx("h-1 w-full overflow-hidden rounded-full bg-surface-3", className)}>
      <div
        className="h-full rounded-full bg-brand transition-[width] duration-300"
        style={{ width: `${pct}%` }}
      />
    </div>
  );
}
