import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import type { Clip, Timeline } from "@avid/shared-types";
import { Button, EmptyState, Panel } from "@avid/ui";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";
import { notifyTimelineChanged, TIMELINE_CHANGED_EVENT } from "../stores/useJobsStore";

export interface InspectorPanelProps {
  clipId: string | null;
}

/**
 * Inspector (Phase 3 UI slice): selected-clip properties with a working
 * trim form (start/duration via `timeline_trim_clip`). Resolves the clip
 * from the backend on selection change; empty selection guides.
 */
export function InspectorPanel({ clipId }: InspectorPanelProps) {
  const [clip, setClip] = useState<Clip | null>(null);
  const [start, setStart] = useState("");
  const [duration, setDuration] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    setStart("");
    setDuration("");
    setError(null);
    if (!clipId || !isTauri()) {
      setClip(null);
      return;
    }
    let cancelled = false;
    const load = (): void => {
      invokeCommand<Timeline>("timeline_get")
        .then((timeline) => {
          if (!cancelled) setClip(timeline.clips[clipId] ?? null);
        })
        .catch(() => {
          if (!cancelled) setClip(null);
        });
    };
    load();
    window.addEventListener(TIMELINE_CHANGED_EVENT, load);
    return () => {
      cancelled = true;
      window.removeEventListener(TIMELINE_CHANGED_EVENT, load);
    };
  }, [clipId]);

  if (!clip) {
    return (
      <Panel title="Inspector" className="flex-1">
        <EmptyState title="Nothing selected" body="Click a clip in the timeline to inspect and trim it." />
      </Panel>
    );
  }

  async function onTrim(event: FormEvent): Promise<void> {
    event.preventDefault();
    const target = clip;
    if (!target) return;
    setBusy(true);
    setError(null);
    try {
      await invokeCommand("timeline_trim_clip", {
        clipId: target.id,
        start: start === "" ? target.start : Number(start),
        duration: duration === "" ? target.duration : Number(duration),
      });
      setStart("");
      setDuration("");
      notifyTimelineChanged();
    } catch (e) {
      setError(e instanceof IpcError ? e.message : "Trim failed unexpectedly.");
    } finally {
      setBusy(false);
    }
  }

  const inputClass =
    "rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 font-mono text-sm text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent";

  return (
    <Panel title="Inspector" className="flex-1">
      <dl className="flex flex-col gap-1 text-sm">
        <div className="flex justify-between">
          <dt className="text-avid-muted">Clip</dt>
          <dd className="text-avid-primary">{clip.name}</dd>
        </div>
        <div className="flex justify-between">
          <dt className="text-avid-muted">Position</dt>
          <dd className="font-mono text-avid-primary">
            {clip.start.toFixed(2)}s + {clip.duration.toFixed(2)}s
          </dd>
        </div>
        <div className="flex justify-between">
          <dt className="text-avid-muted">Source in</dt>
          <dd className="font-mono text-avid-primary">{clip.in_point.toFixed(2)}s</dd>
        </div>
      </dl>
      <form onSubmit={onTrim} className="mt-3 flex flex-col gap-2 border-t border-avid-border-subtle pt-3">
        <div className="grid grid-cols-2 gap-2">
          <label className="flex flex-col gap-1 text-xs text-avid-secondary">
            Start (s)
            <input value={start} onChange={(e) => setStart(e.target.value)} placeholder={clip.start.toFixed(2)} inputMode="decimal" className={inputClass} />
          </label>
          <label className="flex flex-col gap-1 text-xs text-avid-secondary">
            Duration (s)
            <input value={duration} onChange={(e) => setDuration(e.target.value)} placeholder={clip.duration.toFixed(2)} inputMode="decimal" className={inputClass} />
          </label>
        </div>
        <Button type="submit" variant="secondary" disabled={busy}>
          {busy ? "Trimming…" : "Apply trim"}
        </Button>
      </form>
      {error && (
        <p role="alert" className="mt-2 text-xs text-avid-danger">
          {error}
        </p>
      )}
    </Panel>
  );
}
