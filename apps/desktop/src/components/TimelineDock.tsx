import { useCallback, useEffect, useState } from "react";
import type { Clip, Timeline } from "@avid/shared-types";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";
import { TIMELINE_CHANGED_EVENT } from "../stores/useJobsStore";
import { TimelineCanvas } from "./TimelineCanvas";

/**
 * Timeline dock (Phase 3 UI slice): live backend timeline with select,
 * split-at-middle, remove, undo, and redo. Every mutation round-trips
 * through the command engine and autosaves `project.json`. Selection is
 * owned by the parent so the inspector shares it.
 */
export function TimelineDock({
  projectId,
  selectedId,
  onSelect,
}: {
  projectId: string;
  selectedId: string | null;
  onSelect: (clipId: string | null) => void;
}) {
  const [timeline, setTimeline] = useState<Timeline | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const backend = isTauri();

  const refresh = useCallback(async () => {
    try {
      setTimeline(await invokeCommand<Timeline>("timeline_get"));
      setNotice(null);
    } catch (e) {
      setTimeline(null);
      setNotice(e instanceof IpcError ? e.message : "Timeline unavailable.");
    }
  }, []);

  useEffect(() => {
    onSelect(null);
    refresh();
  }, [refresh, projectId]);

  useEffect(() => {
    window.addEventListener(TIMELINE_CHANGED_EVENT, refresh);
    return () => window.removeEventListener(TIMELINE_CHANGED_EVENT, refresh);
  }, [refresh]);

  async function mutate(label: string, run: () => Promise<unknown>): Promise<void> {
    setBusy(true);
    try {
      const result = await run();
      if (typeof result === "string" && (label === "Undo" || label === "Redo")) {
        setNotice(`${label}: ${result}`);
      } else {
        setNotice(null);
      }
      onSelect(null);
      await refresh();
    } catch (e) {
      setNotice(e instanceof IpcError ? e.message : `${label} failed unexpectedly.`);
    } finally {
      setBusy(false);
    }
  }

  const selected: Clip | null = selectedId && timeline ? (timeline.clips[selectedId] ?? null) : null;

  const disabledTitle = backend
    ? undefined
    : "Timeline editing needs the desktop backend (running in the browser).";

  return (
    <div className="flex h-full flex-col">
      <div className="flex items-center gap-2 px-2 pb-2">
        <span className="text-xs font-medium text-avid-secondary">Timeline</span>
        <span className="ml-auto flex items-center gap-1">
          <button
            disabled={!backend || busy || !selected}
            title={disabledTitle ?? "Split the selected clip at its middle"}
            onClick={() =>
              selected &&
              mutate("Split", () =>
                invokeCommand("timeline_split_clip", {
                  clipId: selected.id,
                  at: selected.start + selected.duration / 2,
                }),
              )
            }
            className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised disabled:cursor-not-allowed disabled:opacity-50"
          >
            Split
          </button>
          <button
            disabled={!backend || busy || !selected}
            title={disabledTitle ?? "Remove the selected clip"}
            onClick={() => selected && mutate("Remove", () => invokeCommand("timeline_remove_clip", { clipId: selected.id }))}
            className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised disabled:cursor-not-allowed disabled:opacity-50"
          >
            Remove
          </button>
          <button
            disabled={!backend || busy}
            title={disabledTitle ?? "Undo"}
            onClick={() => mutate("Undo", () => invokeCommand<string>("timeline_undo"))}
            className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised disabled:cursor-not-allowed disabled:opacity-50"
          >
            Undo
          </button>
          <button
            disabled={!backend || busy}
            title={disabledTitle ?? "Redo"}
            onClick={() => mutate("Redo", () => invokeCommand<string>("timeline_redo"))}
            className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised disabled:cursor-not-allowed disabled:opacity-50"
          >
            Redo
          </button>
        </span>
      </div>
      <div className="min-h-0 flex-1 overflow-auto rounded-avid-md border border-avid-border-subtle">
        {timeline ? (
          <TimelineCanvas timeline={timeline} selectedId={selectedId} onSelect={onSelect} />
        ) : (
          <div className="flex h-full items-center justify-center p-4">
            <p className="text-xs text-avid-muted">
              {notice ?? "Empty timeline — import media to begin."}
            </p>
          </div>
        )}
      </div>
      {timeline && notice && <p className="px-2 pt-1 text-xs text-avid-muted">{notice}</p>}
    </div>
  );
}
