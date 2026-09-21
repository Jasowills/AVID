import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import { Channel } from "@tauri-apps/api/core";
import { EXPORT_PRESETS, type ExportResult, type JobEvent, type Timeline } from "@avid/shared-types";
import { Button, Icon, Panel } from "@avid/ui";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";

export interface ExportDialogProps {
  open: boolean;
  onClose: () => void;
}

/**
 * Export dialog (Phase 4 UI slice): preset, filename, verified result.
 * Fetches the timeline for the duration estimate on open; renders on the
 * async `render_export` command (never blocks the UI thread); offers
 * "Show in folder" via the opener plugin on success.
 */
export function ExportDialog({ open, onClose }: ExportDialogProps) {
  const [presetId, setPresetId] = useState("youtube-1080p");
  const [filename, setFilename] = useState("avid-export.mp4");
  const [duration, setDuration] = useState<number | null>(null);
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<ExportResult | null>(null);
  const backend = isTauri();

  useEffect(() => {
    if (!open) return;
    setError(null);
    setResult(null);
    if (!backend) return;
    invokeCommand<Timeline>("timeline_get")
      .then((timeline) => {
        const end = Math.max(
          0,
          ...Object.values(timeline.clips).map((clip) => clip.start + clip.duration),
        );
        setDuration(end);
      })
      .catch((e: unknown) => {
        setError(e instanceof IpcError ? e.message : "Couldn't read the timeline.");
      });
  }, [open, backend]);

  if (!open) return null;

  async function onExport(event: FormEvent): Promise<void> {
    event.preventDefault();
    setBusy(true);
    setProgress(0);
    setError(null);
    setResult(null);
    try {
      const preset = EXPORT_PRESETS.find((p) => p.id === presetId);
      const channel = new Channel<JobEvent>((jobEvent) => {
        if (jobEvent.progress !== null) setProgress(jobEvent.progress);
      });
      const exported = await invokeCommand<ExportResult>("render_export", {
        channel,
        request: {
          presetId,
          customWidth: presetId === "custom" ? (preset?.width ?? null) : null,
          customFps: presetId === "custom" ? (preset?.fps ?? null) : null,
          filename: filename.trim(),
        },
      });
      setProgress(1);
      setResult(exported);
    } catch (e) {
      setError(e instanceof IpcError ? e.message : "Export failed unexpectedly.");
    } finally {
      setBusy(false);
    }
  }

  async function onReveal(): Promise<void> {
    if (!result) return;
    try {
      const { revealItemInDir } = await import("@tauri-apps/plugin-opener");
      await revealItemInDir(result.absolute_path);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Couldn't open the exports folder.");
    }
  }

  const selectClass =
    "rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 text-sm text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent";

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-label="Export video"
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
      onClick={onClose}
    >
      <div onClick={(e) => e.stopPropagation()}>
        <Panel
          title="Export video"
          className="w-full max-w-md"
          actions={
            <button onClick={onClose} aria-label="Close export dialog" className="rounded-avid-sm p-1 text-avid-muted hover:text-avid-primary">
              <Icon name="close" size={14} />
            </button>
          }
        >
          {!backend ? (
            <p className="text-sm text-avid-secondary">
              Export needs the desktop backend — open this project in the AVID app to render.
            </p>
          ) : (
            <form onSubmit={onExport} className="flex flex-col gap-4">
              {duration !== null && (
                <p className="text-sm text-avid-secondary">
                  Timeline duration: <span className="font-mono">{duration.toFixed(1)}s</span>
                </p>
              )}
              <label className="flex flex-col gap-1 text-sm text-avid-secondary">
                Preset
                <select value={presetId} onChange={(e) => setPresetId(e.target.value)} className={selectClass}>
                  {EXPORT_PRESETS.map((preset) => (
                    <option key={preset.id} value={preset.id}>
                      {preset.label} ({preset.width}p, {preset.fps}fps)
                    </option>
                  ))}
                </select>
              </label>
              <label className="flex flex-col gap-1 text-sm text-avid-secondary">
                Filename
                <input
                  value={filename}
                  onChange={(e) => setFilename(e.target.value)}
                  placeholder="avid-export.mp4"
                  className="rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 font-mono text-sm text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
                />
              </label>
              <div className="flex justify-end gap-2">
                <Button type="button" variant="ghost" onClick={onClose}>
                  Cancel
                </Button>
                <Button type="submit" variant="primary" disabled={busy || filename.trim() === ""}>
                  {busy ? "Rendering…" : "Export"}
                </Button>
              </div>
              {busy && progress !== null && (
                <div
                  role="progressbar"
                  aria-valuenow={Math.round(progress * 100)}
                  aria-valuemin={0}
                  aria-valuemax={100}
                  aria-label="Export progress"
                  className="h-1.5 overflow-hidden rounded-full bg-avid-overlay"
                >
                  <div className="h-full bg-avid-accent" style={{ width: `${progress * 100}%` }} />
                </div>
              )}
            </form>
          )}

          {error && (
            <p role="alert" className="mt-3 text-sm text-avid-danger">
              {error}
            </p>
          )}
          {result && (
            <div className="mt-3 rounded-avid-md border border-avid-success/40 p-3">
              <p className="text-sm text-avid-success">
                Exported {result.relative_path} ({result.duration.toFixed(1)}s verified, {result.width}p
                {result.fps}).
              </p>
              {result.caption_path && (
                <p className="mt-1 font-mono text-xs text-avid-secondary">
                  Captions: {result.caption_path}
                </p>
              )}
              <div className="mt-2 flex gap-2">
                <Button variant="secondary" onClick={onReveal}>
                  Show in folder
                </Button>
                <Button variant="ghost" onClick={onClose}>
                  Done
                </Button>
              </div>
            </div>
          )}
        </Panel>
      </div>
    </div>
  );
}
