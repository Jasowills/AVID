import { Link, useNavigate } from "react-router-dom";
import { selectOpenProject, useProjectStore } from "../stores/useProjectStore";

export interface TopBarProps {
  /** "Saved · 12:04" style status. Autosave indicator lands in Phase 3. */
  saveStatus?: string;
}

/**
 * Editor top bar (AGENTS §39): project name, undo/redo (wired in Phase 3),
 * save status, AI status, preview quality (Phase 4), Export (Phase 4), Settings.
 * Unwired controls are visibly disabled with honest titles — never fake buttons.
 */
export function TopBar({ saveStatus = "Not saved yet" }: TopBarProps) {
  const navigate = useNavigate();
  const project = useProjectStore(selectOpenProject);

  return (
    <header className="flex h-12 shrink-0 items-center gap-3 border-b border-avid-border bg-avid-panel px-4">
      <button
        onClick={() => navigate("/")}
        className="text-sm font-semibold text-avid-primary hover:text-avid-accent-hover"
        aria-label="Back to Home"
      >
        AVID
      </button>
      <span className="truncate text-sm text-avid-secondary" aria-live="polite">
        {project ? project.name : "No project open"}
      </span>

      <div className="ml-2 flex items-center gap-1" role="toolbar" aria-label="Edit">
        <button
          disabled
          title="Undo (lands in Phase 3 with the command engine)"
          className="rounded-avid-sm px-2 py-1 text-sm text-avid-muted disabled:cursor-not-allowed disabled:opacity-50"
        >
          Undo
        </button>
        <button
          disabled
          title="Redo (lands in Phase 3 with the command engine)"
          className="rounded-avid-sm px-2 py-1 text-sm text-avid-muted disabled:cursor-not-allowed disabled:opacity-50"
        >
          Redo
        </button>
      </div>

      <span className="text-xs text-avid-muted">{saveStatus}</span>

      <div className="ml-auto flex items-center gap-3">
        <span
          className="inline-flex items-center gap-1.5 text-xs text-avid-secondary"
          title="AI runtime status (provider wiring lands in Phase 6)"
        >
          <span aria-hidden="true" className="inline-block size-2 rounded-full bg-avid-muted" />
          AI: not configured
        </span>
        <button
          disabled
          title="Export (lands in Phase 4 with the render pipeline)"
          className="rounded-avid-md bg-avid-accent-muted px-3 py-1.5 text-sm font-medium text-avid-muted disabled:cursor-not-allowed"
        >
          Export
        </button>
        <Link
          to="/settings"
          className="rounded-avid-sm px-2 py-1 text-sm text-avid-secondary hover:text-avid-primary"
        >
          Settings
        </Link>
      </div>
    </header>
  );
}
