import { Link } from "react-router-dom";
import {
  CircleCheck,
  FolderOpen,
  RefreshCw,
  TriangleAlert,
} from "lucide-react";
import * as ipc from "@/lib/ipc";
import { useIntegrity, useSystemInfo } from "@/lib/queries";
import { useRelink } from "@/lib/useImport";
import { useUi } from "@/store/ui";
import { cx, humanBytes } from "@/lib/format";

export function HealthPage() {
  const { data: issues = [], isFetching, refetch } = useIntegrity();
  const { data: info } = useSystemInfo();
  const relink = useRelink();
  const toast = useUi((s) => s.toast);

  const errors = issues.filter((i) => i.severity === "error");
  const warnings = issues.filter((i) => i.severity !== "error");

  return (
    <div className="h-full overflow-y-auto">
      <div className="mx-auto max-w-3xl space-y-6 px-8 py-10">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-ink">Health</h1>
            <p className="mt-1 text-sm text-ink-muted">
              What RustClass expects to find on disk, and what is actually there.
            </p>
          </div>
          <button className="btn btn-sm" disabled={isFetching} onClick={() => void refetch()}>
            <RefreshCw size={13} className={isFetching ? "animate-spin" : undefined} />
            Re-check
          </button>
        </div>

        {issues.length === 0 && !isFetching && (
          <div className="flex items-center gap-3 rounded-xl border border-ok/40 bg-ok/10 p-5">
            <CircleCheck size={22} className="text-ok" />
            <div>
              <p className="text-sm font-medium text-ink">Everything checks out.</p>
              <p className="text-xs text-ink-muted">
                Every file the course references is where it should be.
              </p>
            </div>
          </div>
        )}

        {[
          { list: errors, label: "Problems", tone: "error" as const },
          { list: warnings, label: "Worth knowing", tone: "warning" as const },
        ].map(
          ({ list, label, tone }) =>
            list.length > 0 && (
              <section key={label}>
                <h2 className="mb-2 text-sm font-semibold uppercase tracking-wide text-ink-faint">
                  {label} ({list.length})
                </h2>
                <ul className="space-y-2">
                  {list.map((issue, i) => (
                    <li
                      key={`${issue.entityId}-${i}`}
                      className={cx(
                        "rounded-xl border p-4",
                        tone === "error"
                          ? "border-danger/40 bg-danger/10"
                          : "border-warn/40 bg-warn/10",
                      )}
                    >
                      <div className="flex items-start gap-2">
                        <TriangleAlert
                          size={16}
                          className={cx(
                            "mt-0.5 shrink-0",
                            tone === "error" ? "text-danger" : "text-warn",
                          )}
                        />
                        <div className="min-w-0 flex-1">
                          <p className="text-sm text-ink">{issue.message}</p>
                          {issue.path && (
                            <p className="mt-1 break-all rounded bg-surface-2/70 px-2 py-1 font-mono text-[11px] text-ink-muted">
                              {issue.path}
                            </p>
                          )}
                          <div className="mt-2 flex flex-wrap gap-1.5">
                            {issue.entityType === "asset" && issue.kind === "missing_file" && (
                              <button
                                className="btn btn-sm btn-primary"
                                onClick={async () => {
                                  if (await relink(issue.entityId, issue.message)) void refetch();
                                }}
                              >
                                Locate file…
                              </button>
                            )}
                            {issue.path && (
                              <button
                                className="btn btn-sm"
                                onClick={() =>
                                  void ipc.revealPath(issue.path as string).catch(() =>
                                    toast({
                                      tone: "error",
                                      message: "That folder could not be opened.",
                                    }),
                                  )
                                }
                              >
                                <FolderOpen size={12} /> Show in folder
                              </button>
                            )}
                            {issue.kind === "partial_file" && (
                              <Link to="/settings" className="btn btn-sm">
                                Clean up storage
                              </Link>
                            )}
                          </div>
                        </div>
                      </div>
                    </li>
                  ))}
                </ul>
              </section>
            ),
        )}

        {info && (
          <section className="rounded-xl border border-line bg-surface-1 p-5">
            <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-ink-faint">
              Library
            </h2>
            <dl className="grid gap-2 text-sm sm:grid-cols-2">
              <Pair label="Chapters" value={String(info.chapterCount)} />
              <Pair label="Blocks" value={String(info.blockCount)} />
              <Pair label="Assets" value={String(info.assetCount)} />
              <Pair label="Media on disk" value={humanBytes(info.mediaBytes)} />
              <Pair label="Tracked in the database" value={humanBytes(info.assetBytesTracked)} />
              <Pair label="Database" value={humanBytes(info.dbBytes)} />
              <Pair label="In the Trash" value={`${info.trashedChapters} chapters, ${info.trashedBlocks} blocks`} />
              <Pair label="Schema" value={`v${info.schemaVersion}`} />
            </dl>
            <p className="mt-3 break-all font-mono text-[11px] text-ink-faint">
              {info.logDir}
            </p>
            <button
              className="btn btn-sm mt-2"
              onClick={() => void ipc.openPath(info.logDir).catch(() => undefined)}
            >
              <FolderOpen size={13} /> Open log folder
            </button>
          </section>
        )}
      </div>
    </div>
  );
}

function Pair({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between gap-2 border-b border-line pb-1">
      <dt className="text-ink-muted">{label}</dt>
      <dd className="text-ink">{value}</dd>
    </div>
  );
}
