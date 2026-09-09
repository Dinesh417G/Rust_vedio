/** Display helpers shared across the app. */

export function humanBytes(n: number): string {
  if (!Number.isFinite(n) || n < 0) return "—";
  const units = ["B", "KB", "MB", "GB", "TB"];
  let v = n;
  let u = 0;
  while (v >= 1024 && u < units.length - 1) {
    v /= 1024;
    u += 1;
  }
  return u === 0 ? `${n} B` : `${v.toFixed(1)} ${units[u]}`;
}

/** `mm:ss`, or `h:mm:ss` once past an hour. */
export function clockTime(secs: number | null | undefined): string {
  if (secs == null || !Number.isFinite(secs) || secs < 0) return "0:00";
  const total = Math.floor(secs);
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  const mm = h > 0 ? String(m).padStart(2, "0") : String(m);
  return h > 0 ? `${h}:${mm}:${String(s).padStart(2, "0")}` : `${mm}:${String(s).padStart(2, "0")}`;
}

export function minutesLabel(mins: number | null | undefined): string | null {
  if (mins == null || mins <= 0) return null;
  if (mins < 60) return `${mins} min`;
  const h = Math.floor(mins / 60);
  const m = mins % 60;
  return m === 0 ? `${h} hr` : `${h} hr ${m} min`;
}

export function relativeDate(ms: number): string {
  const diff = Date.now() - ms;
  const day = 24 * 60 * 60 * 1000;
  if (diff < 60_000) return "just now";
  if (diff < 3_600_000) return `${Math.floor(diff / 60_000)} min ago`;
  if (diff < day) return `${Math.floor(diff / 3_600_000)} hr ago`;
  if (diff < 7 * day) return `${Math.floor(diff / day)} days ago`;
  return new Date(ms).toLocaleDateString();
}

/** Days left before the Trash empties this item for good. */
export function daysRemaining(deletedAt: number, retentionDays: number): number {
  const elapsed = (Date.now() - deletedAt) / (24 * 60 * 60 * 1000);
  return Math.max(0, Math.ceil(retentionDays - elapsed));
}

export function cx(...parts: Array<string | false | null | undefined>): string {
  return parts.filter(Boolean).join(" ");
}

/** The number shown on a chapter row: explicit if set, otherwise its position. */
export function chapterLabel(
  displayNumber: string | null,
  index: number,
): string {
  return displayNumber?.trim() || String(index + 1);
}
