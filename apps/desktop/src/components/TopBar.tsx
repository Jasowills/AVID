import { Link, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";
import { loadProviderSummary } from "../lib/provider";
import { selectOpenProject, useProjectStore } from "../stores/useProjectStore";
import { notifyTimelineChanged, runningCount, useJobsStore } from "../stores/useJobsStore";

export interface TopBarProps {
  /** Save status text; Editor passes the autosave state. */
  saveStatus?: string;
  /** Opens the export dialog. Absent in contexts without a timeline. */
  onExport?: () => void;
}

/**
 * Editor top bar: project name, working undo/redo, save status, live AI
 * provider pill, job activity, Export, Settings. Controls that need the
 * backend disable honestly outside Tauri — never fake buttons.
 */
export function TopBar({ saveStatus = "Not saved yet", onExport }: TopBarProps) {
  const navigate = useNavigate();
  const project = useProjectStore(selectOpenProject);
  const [undoError, setUndoError] = useState<string | null>(null);
  const backend = isTauri();
  const provider = loadProviderSummary();

  async function history(action: "timeline_undo" | "timeline_redo"): Promise<void> {
    setUndoError(null);
    try {
      await invokeCommand(action);
      notifyTimelineChanged();
    } catch (e) {
      setUndoError(e instanceof IpcError ? e.message : "History unavailable.");
    }
  }
  const jobs = useJobsStore((s) => s.jobs);
  const startPolling = useJobsStore((s) => s.startPolling);
  const running = runningCount(jobs);

  useEffect(() => {
    startPolling();
  }, [startPolling]);

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
          onClick={() => history("timeline_undo")}
          disabled={!backend || !project}
          title={backend ? "Undo (Cmd/Ctrl+Z)" : "Undo needs the desktop backend"}
          className="rounded-avid-sm px-2 py-1 text-sm text-avid-secondary hover:bg-avid-raised disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent"
        >
          Undo
        </button>
        <button
          onClick={() => history("timeline_redo")}
          disabled={!backend || !project}
          title={backend ? "Redo (Cmd/Ctrl+Shift+Z)" : "Redo needs the desktop backend"}
          className="rounded-avid-sm px-2 py-1 text-sm text-avid-secondary hover:bg-avid-raised disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent"
        >
          Redo
        </button>
      </div>
      {undoError && (
        <span role="alert" className="max-w-48 truncate text-xs text-avid-danger" title={undoError}>
          {undoError}
        </span>
      )}

      <span className="text-xs text-avid-muted">{saveStatus}</span>

      {running > 0 && (
        <span
          className="rounded-full bg-avid-accent-muted px-2 py-0.5 text-xs text-avid-primary"
          role="status"
          aria-label={`${running} background tasks running`}
        >
          {running} task{running === 1 ? "" : "s"}
        </span>
      )}

      <div className="ml-auto flex items-center gap-3">
        <Link
          to="/settings"
          className="inline-flex items-center gap-1.5 rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised"
          title={provider ? `Provider: ${provider.model} @ ${provider.baseUrl}` : "No provider tested yet — open Settings"}
        >
          <span
            aria-hidden="true"
            className={`inline-block size-2 rounded-full ${provider ? "bg-avid-success" : "bg-avid-muted"}`}
          />
          AI: {provider ? provider.model : "defaults"}
        </Link>
        <button
          onClick={onExport}
          disabled={!onExport}
          title={onExport ? "Export the timeline (renders + verifies)" : "Export (open a project first)"}
          className="rounded-avid-md bg-avid-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-avid-accent-hover disabled:cursor-not-allowed disabled:bg-avid-accent-muted disabled:text-avid-muted"
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
