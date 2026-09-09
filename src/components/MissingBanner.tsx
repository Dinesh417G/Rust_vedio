import { Link } from "react-router-dom";
import { TriangleAlert, X } from "lucide-react";
import { useIntegrity } from "@/lib/queries";
import { useUi } from "@/store/ui";

/**
 * Shown on startup and whenever a chapter open turns up a missing file. It is
 * dismissible, because being nagged about a drive you know is unplugged is
 * worse than the problem.
 */
export function MissingBanner() {
  const { data: issues = [] } = useIntegrity();
  const dismissed = useUi((s) => s.dismissedMissingBanner);
  const dismiss = useUi((s) => s.dismissMissingBanner);

  const missing = issues.filter((i) => i.kind === "missing_file");
  if (dismissed || missing.length === 0) return null;

  return (
    <div className="flex items-center gap-3 border-b border-danger/40 bg-danger/10 px-4 py-2 text-sm">
      <TriangleAlert size={15} className="shrink-0 text-danger" />
      <span className="text-ink">
        {missing.length} file{missing.length === 1 ? " is" : "s are"} missing from where
        RustClass expects them.
      </span>
      <Link to="/health" className="btn btn-sm">
        See what is missing
      </Link>
      <button
        className="btn btn-ghost btn-sm ml-auto"
        onClick={dismiss}
        aria-label="Dismiss"
      >
        <X size={14} />
      </button>
    </div>
  );
}
