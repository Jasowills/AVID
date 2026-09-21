import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import { Button, EmptyState, Icon, Panel, TextField } from "@avid/ui";
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

/** Filter assets by file name or id (case-insensitive). Pure — tested. */
export function filterAssets(assets: MediaAsset[], query: string): MediaAsset[] {
  const needle = query.trim().toLowerCase();
  if (needle === "") return assets;
  return assets.filter(
    (asset) =>
      asset.file_name.toLowerCase().includes(needle) ||
      asset.id.toLowerCase().includes(needle),
  );
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
  const [assetsLoading, setAssetsLoading] = useState(true);
  const [thumbs, setThumbs] = useState<Record<string, string>>({});
  const [thumbBusy, setThumbBusy] = useState<string | null>(null);
  const [query, setQuery] = useState("");
  const [view, setView] = useState<"grid" | "list">("grid");

  useEffect(() => {
    if (!isTauri()) {
      setAssetsLoading(false);
      return;
    }
    setAssetsLoading(true);
    invokeCommand<MediaAsset[]>("list_assets")
      .then(setAssets)
      .catch(() => undefined)
      .finally(() => setAssetsLoading(false));
  }, [imported]);

  // Thumbnails for video assets resolve lazily after each list refresh.
  // Best-effort per asset: one failure never blocks the rest.
  useEffect(() => {
    if (!isTauri()) return;
    let cancelled = false;
    const missing = assets.filter(
      (asset) => asset.dimensions && !thumbs[asset.id],
    );
    if (missing.length === 0) return;
    void (async () => {
      for (const asset of missing) {
        if (cancelled) return;
        try {
          const relative = await invokeCommand<string>("thumbnail_asset", {
            assetId: asset.id,
          });
          if (!cancelled) setThumbs((prev) => ({ ...prev, [asset.id]: relative }));
        } catch {
          // Thumbnail failures stay silent per asset; the manual button retries.
        }
      }
    })();
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [assets]);

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

  const filtered = filterAssets(assets, query);

  return (
    <Panel title="Media" className="flex-1">
      <form onSubmit={onProbe} className="flex flex-col gap-3">
        <TextField
          label="Project-relative file"
          placeholder="media/clip.mp4"
          value={path}
          onChange={(e) => setPath(e.target.value)}
        />
        <Button type="submit" variant="primary" disabled={busy || path.trim() === ""} className="min-w-36">
          {busy ? `Probing ${path.trim().split("/").pop() || "file"}…` : "Probe file"}
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
            body="Enter a path inside the project folder and probe it, or import media below."
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
        <Button type="submit" variant="secondary" disabled={importBusy || source.trim() === ""} className="min-w-44">
          {importBusy ? `Importing ${source.trim().split("/").pop() || "file"}…` : "Import into project"}
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

      {assetsLoading ? (
        <div className="mt-4 border-t border-avid-border-subtle pt-3" aria-label="Loading media">
          <div className="grid grid-cols-2 gap-2" aria-hidden="true">
            {[0, 1, 2, 3].map((i) => (
              <div key={i} className="overflow-hidden rounded-avid-md border border-avid-border">
                <div className="avid-skeleton aspect-video w-full" />
                <div className="avid-skeleton mx-2 mb-2 mt-2 h-3 w-2/3" />
              </div>
            ))}
          </div>
        </div>
      ) : (
        assets.length > 0 && (
          <div className="mt-4 border-t border-avid-border-subtle pt-3">
          <div className="mb-2 flex items-center gap-2">
            <h3 className="text-xs font-medium text-avid-secondary">
              Project media ({filtered.length}/{assets.length})
            </h3>
            <input
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search media…"
              aria-label="Search media"
              className="min-w-0 flex-1 rounded-avid-sm border border-avid-border bg-avid-raised px-2 py-1 text-xs text-avid-primary placeholder:text-avid-muted focus-visible:outline-2 focus-visible:outline-avid-accent"
            />
            <span className="flex gap-0.5" role="group" aria-label="Library view">
              {(["grid", "list"] as const).map((mode) => (
                <button
                  key={mode}
                  onClick={() => setView(mode)}
                  aria-pressed={view === mode}
                  title={`${mode} view`}
                  className={`rounded-avid-sm px-2 py-1 text-xs ${
                    view === mode
                      ? "bg-avid-raised text-avid-primary"
                      : "text-avid-muted hover:text-avid-secondary"
                  }`}
                >
                  {mode === "grid" ? <Icon name="grid" size={13} /> : <Icon name="list" size={13} />}
                </button>
              ))}
            </span>
          </div>
          {filtered.length === 0 ? (
            <p className="text-xs text-avid-muted">No media matches “{query}”.</p>
          ) : view === "grid" ? (
            <ul className="grid grid-cols-2 gap-2">
              {filtered.map((asset) => (
                <li
                  key={asset.id}
                  className="overflow-hidden rounded-avid-md border border-avid-border bg-avid-raised"
                >
                  <div className="relative aspect-video bg-avid-overlay">
                    {thumbs[asset.id] ? (
                      <img
                        src={streamUrl("project", thumbs[asset.id] as string)}
                        alt={`Thumbnail of ${asset.file_name}`}
                        className="h-full w-full object-cover"
                        loading="lazy"
                      />
                    ) : (
                      <span className="flex h-full items-center justify-center text-avid-muted">
                        <Icon name={asset.dimensions ? "film" : "music"} size={22} />
                      </span>
                    )}
                    {asset.duration !== null && (
                      <span className="absolute bottom-1 right-1 rounded bg-black/70 px-1 font-mono text-[10px] text-white">
                        {formatDuration(asset.duration)}
                      </span>
                    )}
                  </div>
                  <div className="flex items-center justify-between gap-1 p-1.5">
                    <span className="min-w-0 truncate text-xs font-medium text-avid-primary" title={asset.file_name}>
                      {asset.file_name}
                    </span>
                    <button
                      onClick={() => onTranscribeAsset(asset.id)}
                      title={`Transcribe ${asset.file_name}`}
                      className="shrink-0 rounded-avid-sm px-1.5 py-0.5 text-xs text-avid-accent hover:bg-avid-accent-muted"
                    >
                      →
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            <ul className="flex flex-col gap-1">
              {filtered.map((asset) => (
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
                        loading="lazy"
                      />
                    )}
                  </span>
                  <span className="flex shrink-0 flex-col items-end gap-1">
                    {asset.dimensions && !thumbs[asset.id] && (
                      <button
                        onClick={() => onThumbnail(asset.id)}
                        disabled={thumbBusy === asset.id}
                        className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised disabled:opacity-50"
                      >
                        {thumbBusy === asset.id ? "…" : "Retry thumb"}
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
          )}
        </div>
        )
      )}
    </Panel>
  );
}
