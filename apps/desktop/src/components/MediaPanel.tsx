import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import { Button, EmptyState, Panel, TextField } from "@avid/ui";
import type { MediaAsset, MediaInfo } from "@avid/shared-types";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";
import { notifyTimelineChanged } from "../stores/useJobsStore";
import { streamUrl } from "./PreviewPane";

export interface ImportedAsset {
  id: string;
  file_name: string;
  relative_path: string;
  duration: number | null;
}

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
export function MediaPanel({ onTranscribeAsset }: { onTranscribeAsset: (assetId: string) => void }) {
  const [path, setPath] = useState("media/clip.mp4");
  const [info, setInfo] = useState<MediaInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [source, setSource] = useState("");
  const [imported, setImported] = useState<ImportedAsset | null>(null);
  const [importError, setImportError] = useState<string | null>(null);
  const [importBusy, setImportBusy] = useState(false);
  const [assets, setAssets] = useState<MediaAsset[]>([]);
  const [thumbs, setThumbs] = useState<Record<string, string>>({});
  const [thumbBusy, setThumbBusy] = useState<string | null>(null);

  useEffect(() => {
    if (!isTauri()) return;
    invokeCommand<MediaAsset[]>("list_assets").then(setAssets).catch(() => undefined);
  }, [imported]);

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

  async function onImport(event: FormEvent): Promise<void> {
    event.preventDefault();
    setImportBusy(true);
    setImportError(null);
    setImported(null);
    try {
      const asset = await invokeCommand<ImportedAsset>("import_media", {
        sourcePath: source.trim(),
      });
      setImported(asset);
      notifyTimelineChanged();
    } catch (e) {
      setImportError(e instanceof IpcError ? e.message : "Import failed unexpectedly.");
    } finally {
      setImportBusy(false);
    }
  }

  async function onThumbnail(assetId: string): Promise<void> {
    setThumbBusy(assetId);
    try {
      const relative = await invokeCommand<string>("thumbnail_asset", { assetId });
      setThumbs((prev) => ({ ...prev, [assetId]: relative }));
    } catch (e) {
      setImportError(e instanceof IpcError ? e.message : "Thumbnail failed unexpectedly.");
    } finally {
      setThumbBusy(null);
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

      <form onSubmit={onImport} className="mt-4 flex flex-col gap-3 border-t border-avid-border-subtle pt-4">
        <TextField
          label="Import file (full path on this machine)"
          placeholder="/Users/you/Movies/clip.mp4"
          value={source}
          onChange={(e) => setSource(e.target.value)}
        />
        <Button type="submit" variant="secondary" disabled={importBusy || source.trim() === ""}>
          {importBusy ? "Importing…" : "Import into project"}
        </Button>
      </form>

      {importError && (
        <p role="alert" className="mt-3 text-sm text-avid-danger">
          {importError}
        </p>
      )}
      {imported && (
        <p className="mt-3 text-sm text-avid-success">
          Imported {imported.file_name} ({formatDuration(imported.duration)}) — asset {imported.id}
        </p>
      )}

      {assets.length > 0 && (
        <div className="mt-4 border-t border-avid-border-subtle pt-3">
          <h3 className="mb-2 text-xs font-medium text-avid-secondary">Project media ({assets.length})</h3>
          <ul className="flex flex-col gap-1">
            {assets.map((asset) => (
              <li
                key={asset.id}
                className="flex items-center justify-between gap-2 rounded-avid-sm bg-avid-raised px-2 py-1.5"
              >
                <span className="min-w-0">
                  <span className="block truncate text-xs font-medium text-avid-primary">{asset.file_name}</span>
                  <span className="block truncate font-mono text-[11px] text-avid-muted">
                    {asset.id} · {formatDuration(asset.duration)}
                  </span>
                  {thumbs[asset.id] && (
                    <img
                      src={streamUrl("project", thumbs[asset.id] as string)}
                      alt={`Thumbnail of ${asset.file_name}`}
                      className="mt-1 h-16 rounded-avid-sm border border-avid-border object-cover"
                    />
                  )}
                </span>
                <span className="flex shrink-0 flex-col items-end gap-1">
                  {asset.dimensions && (
                    <button
                      onClick={() => onThumbnail(asset.id)}
                      disabled={thumbBusy === asset.id}
                      className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised disabled:opacity-50"
                    >
                      {thumbBusy === asset.id ? "…" : "Thumbnail"}
                    </button>
                  )}
                  <button
                    onClick={() => onTranscribeAsset(asset.id)}
                    className="rounded-avid-sm px-2 py-1 text-xs text-avid-accent hover:bg-avid-accent-muted"
                  >
                    Transcribe →
                  </button>
                </span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </Panel>
  );
}
