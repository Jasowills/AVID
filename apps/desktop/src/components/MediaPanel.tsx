import { useState } from "react";
import type { FormEvent } from "react";
import { Button, EmptyState, Panel, TextField } from "@avid/ui";
import type { MediaInfo } from "@avid/shared-types";
import { invokeCommand, IpcError } from "../lib/ipc";

/** Format seconds as m:ss.t for display. Pure — unit-tested. */
export function formatDuration(seconds: number | null): string {
  if (seconds === null || !Number.isFinite(seconds) || seconds < 0) return "—";
  const minutes = Math.floor(seconds / 60);
  const rest = seconds - minutes * 60;
  return `${minutes}:${rest.toFixed(1).padStart(4, "0")}`;
}

/**
 * Media panel (Phase 2 slice): probes a project-relative file through the
 * `probe_media` command and renders structured metadata. Outside Tauri the
 * form explains why probing needs the desktop backend — never a fake result.
 */
export function MediaPanel() {
  const [path, setPath] = useState("media/clip.mp4");
  const [info, setInfo] = useState<MediaInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function onProbe(event: FormEvent): Promise<void> {
    event.preventDefault();
    setBusy(true);
    setError(null);
    setInfo(null);
    try {
      const result = await invokeCommand<MediaInfo>("probe_media", { relativePath: path.trim() });
      setInfo(result);
    } catch (e) {
      setError(e instanceof IpcError ? e.message : "Probing failed unexpectedly.");
    } finally {
      setBusy(false);
    }
  }

  return (
    <Panel title="Media" className="flex-1">
      <form onSubmit={onProbe} className="flex flex-col gap-3">
        <TextField
          label="Project-relative file"
          placeholder="media/clip.mp4"
          value={path}
          onChange={(e) => setPath(e.target.value)}
        />
        <Button type="submit" variant="primary" disabled={busy || path.trim() === ""}>
          {busy ? "Probing…" : "Probe file"}
        </Button>
      </form>

      {error && (
        <p role="alert" className="mt-3 text-sm text-avid-danger">
          {error}
        </p>
      )}

      {info && (
        <dl className="mt-3 flex flex-col gap-1 text-sm">
          <div className="flex justify-between">
            <dt className="text-avid-muted">Duration</dt>
            <dd className="font-mono text-avid-primary">{formatDuration(info.duration)}</dd>
          </div>
          <div className="flex justify-between">
            <dt className="text-avid-muted">Format</dt>
            <dd className="font-mono text-avid-primary">{info.format || "—"}</dd>
          </div>
          {info.streams.map((stream) => (
            <div key={stream.index} className="flex justify-between">
              <dt className="text-avid-muted">
                {stream.codec_type || "stream"} #{stream.index}
              </dt>
              <dd className="font-mono text-avid-primary">
                {stream.codec_name}
                {stream.width && stream.height ? ` · ${stream.width}×${stream.height}` : ""}
                {stream.sample_rate ? ` · ${stream.sample_rate}Hz` : ""}
              </dd>
            </div>
          ))}
        </dl>
      )}

      {!info && !error && (
        <div className="mt-3">
          <EmptyState
            title="No file probed"
            body="Enter a path inside the project folder and probe it. Import and thumbnails land next."
          />
        </div>
      )}
    </Panel>
  );
}
