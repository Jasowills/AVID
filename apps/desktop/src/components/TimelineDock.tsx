import { useCallback, useEffect, useRef, useState } from "react";
import type { Clip, Timeline } from "@avid/shared-types";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";
import { TIMELINE_CHANGED_EVENT } from "../stores/useJobsStore";
import { usePlaybackStore } from "../stores/usePlaybackStore";
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
  zoom,
}: {
  projectId: string;
  selectedId: string | null;
  onSelect: (clipId: string | null) => void;
  /** Zoom level owned by the editor shell (the tools cell renders the control). */
  zoom: number;
}) {
  const [timeline, setTimeline] = useState<Timeline | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [peaksByMedia, setPeaksByMedia] = useState<Record<string, number[]>>({});
  const currentTime = usePlaybackStore((state) => state.currentTime);
  const requestSeek = usePlaybackStore((state) => state.requestSeek);
  const backend = isTauri();

  // Waveforms for every unique media id with audio. Best-effort per
  // source: failures (video-only files) simply leave that clip flat.
  const refreshPeaks = useCallback(async (timeline: Timeline) => {
    const ids = [...new Set(Object.values(timeline.clips).map((clip) => clip.source_media_id))];
    const entries = await Promise.all(
      ids.map(async (sourceId) => {
        try {
          const peaks = await invokeCommand<number[]>("waveform_peaks", {
            assetId: sourceId,
            buckets: 160,
          });
          return [sourceId, peaks] as const;
        } catch {
          return null;
        }
      }),
    );
    const next: Record<string, number[]> = {};
    for (const entry of entries) {
      if (entry) next[entry[0]] = entry[1];
    }
    setPeaksByMedia(next);
  }, []);

  const refreshAll = useCallback(async () => {
    try {
      const timeline = await invokeCommand<Timeline>("timeline_get");
      setTimeline(timeline);
      setNotice(null);
      await refreshPeaks(timeline);
    } catch (e) {
      setTimeline(null);
      setNotice(e instanceof IpcError ? e.message : "Timeline unavailable.");
    }
  }, [refreshPeaks]);

  useEffect(() => {
    onSelect(null);
    void refreshAll();
  }, [refreshAll, projectId]);

  useEffect(() => {
    const reload = (): void => {
      void refreshAll();
    };
    window.addEventListener(TIMELINE_CHANGED_EVENT, reload);
    return () => window.removeEventListener(TIMELINE_CHANGED_EVENT, reload);
  }, [refreshAll]);

  const selected: Clip | null =
    selectedId && timeline ? (timeline.clips[selectedId] ?? null) : null;
  const mutateRef = useRef(mutate);
  mutateRef.current = mutate;

  const handleMove = (clipId: string, start: number): void => {
    void mutate("Move", () => invokeCommand("timeline_move_clip", { clipId, start }));
  };

  const handleTrim = (clipId: string, start: number, duration: number): void => {
    void mutate("Trim", () =>
      invokeCommand("timeline_trim_clip", { clipId, start, duration }),
    );
  };

  // Keyboard editing: S split, Delete remove, Cmd/Ctrl+Z undo, +Shift redo.
  // Skipped inside text fields; browser build shows the same disabled honesty.
  useEffect(() => {
    const onKey = (event: KeyboardEvent): void => {
      if (!backend || busy) return;
      const target = event.target as HTMLElement | null;
      if (
        target &&
        (target.tagName === "INPUT" ||
          target.tagName === "TEXTAREA" ||
          target.tagName === "SELECT" ||
          target.isContentEditable)
      ) {
        return;
      }
      const mod = event.metaKey || event.ctrlKey;
      if (mod && event.key.toLowerCase() === "z") {
        event.preventDefault();
        const redo = event.shiftKey;
        void mutateRef.current(redo ? "Redo" : "Undo", () =>
          invokeCommand<string>(redo ? "timeline_redo" : "timeline_undo"),
        );
      } else if (!mod && (event.key === "Delete" || event.key === "Backspace") && selected) {
        const id = selected.id;
        void mutateRef.current("Remove", () =>
          invokeCommand("timeline_remove_clip", { clipId: id }),
        );
      } else if (!mod && event.key.toLowerCase() === "s" && selected) {
        const id = selected.id;
        const at = selected.start + selected.duration / 2;
        void mutateRef.current("Split", () =>
          invokeCommand("timeline_split_clip", { clipId: id, at }),
        );
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [backend, busy, selected]);

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
      await refreshAll();
    } catch (e) {
      setNotice(e instanceof IpcError ? e.message : `${label} failed unexpectedly.`);
    } finally {
      setBusy(false);
    }
  }

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
          <TimelineCanvas
            timeline={timeline}
            selectedId={selectedId}
            onSelect={onSelect}
            peaksByMedia={peaksByMedia}
            zoom={zoom}
            playhead={currentTime}
            gutter={0}
            hideTrackChrome
            onSeek={backend ? (time) => requestSeek(time) : undefined}
            onMoveClip={backend ? handleMove : undefined}
            onTrimClip={backend ? handleTrim : undefined}
            onToggleLock={undefined}
            onSplitClip={
              backend
                ? (clip) =>
                    mutate("Split", () =>
                      invokeCommand("timeline_split_clip", {
                        clipId: clip.id,
                        at: clip.start + clip.duration / 2,
                      }),
                    )
                : undefined
            }
            onRemoveClip={
              backend
                ? (clip) => mutate("Remove", () => invokeCommand("timeline_remove_clip", { clipId: clip.id }))
                : undefined
            }
          />
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
