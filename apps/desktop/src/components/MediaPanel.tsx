/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/components/sidebar-left/assets.tsx + asset-item.tsx) —
 * asset grid re-implemented in React against AVID's media commands:
 * thumbnail cards with duration badges, select ring, search, import, and a
 * context menu whose every item runs a real command. No folders: AVID assets
 * are flat, so no breadcrumb bar is faked. */

import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import { Button, ContextArea, EmptyState, Icon, Panel, TextField } from "@avid/ui";
import type { ContextMenuItem } from "@avid/ui";
import type { MediaAsset, MediaInfo, Timeline, Track } from "@avid/shared-types";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";
import { notifyTimelineChanged } from "../stores/useJobsStore";
import { usePlaybackStore } from "../stores/usePlaybackStore";
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

function round3(n: number): number {
  return Math.round(n * 1000) / 1000;
}

export function MediaPanel({ onTranscribeAsset }: { onTranscribeAsset: (assetId: string) => void }) {
  const [assets, setAssets] = useState<MediaAsset[]>([]);
  const [assetsLoading, setAssetsLoading] = useState(true);
  const [thumbs, setThumbs] = useState<Record<string, string>>({});
  const [query, setQuery] = useState("");
  const [view, setView] = useState<"grid" | "list">("grid");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [actionBusy, setActionBusy] = useState<string | null>(null);
  const [importOpen, setImportOpen] = useState(false);
  const [probeOpen, setProbeOpen] = useState(false);
  const [path, setPath] = useState("media/clip.mp4");
  const [info, setInfo] = useState<MediaInfo | null>(null);
  const [probeError, setProbeError] = useState<string | null>(null);
  const [probeBusy, setProbeBusy] = useState(false);
  const [source, setSource] = useState("");
  const [imported, setImported] = useState<ImportedAsset | null>(null);
  const [importError, setImportError] = useState<string | null>(null);
  const [importBusy, setImportBusy] = useState(false);
  const currentTime = usePlaybackStore((s) => s.currentTime);
  const backend = isTauri();

  useEffect(() => {
    if (!backend) {
      setAssetsLoading(false);
      return;
    }
    setAssetsLoading(true);
    invokeCommand<MediaAsset[]>("list_assets")
      .then((list) => {
        setAssets(list);
        setSelectedId((prev) => (prev && list.some((a) => a.id === prev) ? prev : null));
      })
      .catch(() => undefined)
      .finally(() => setAssetsLoading(false));
  }, [backend, imported]);

  // Thumbnails resolve lazily after each list refresh, best-effort per asset.
  useEffect(() => {
    if (!backend) return;
    let cancelled = false;
    const missing = assets.filter((asset) => asset.dimensions && !thumbs[asset.id]);
    if (missing.length === 0) return;
    void (async () => {
      for (const asset of missing) {
        if (cancelled) return;
        try {
          const relative = await invokeCommand<string>("thumbnail_asset", { assetId: asset.id });
          if (!cancelled) setThumbs((prev) => ({ ...prev, [asset.id]: relative }));
        } catch {
          // Per-asset silence; the icon fallback stays.
        }
      }
    })();
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [assets, backend]);

  async function runProbe(relativePath: string): Promise<void> {
    setProbeBusy(true);
    setProbeError(null);
    setInfo(null);
    try {
      const result = await invokeCommand<MediaInfo>("probe_media", { relativePath });
      setInfo(result);
    } catch (e) {
      setProbeError(e instanceof IpcError ? e.message : "Probing failed unexpectedly.");
    } finally {
      setProbeBusy(false);
    }
  }

  function onProbe(event: FormEvent): void {
    event.preventDefault();
    setProbeOpen(true);
    void runProbe(path.trim());
  }

  async function onImport(event: FormEvent): Promise<void> {
    event.preventDefault();
    setImportBusy(true);
    setImportError(null);
    setImported(null);
    try {
      const asset = await invokeCommand<ImportedAsset>("import_media", { sourcePath: source.trim() });
      setImported(asset);
      setSelectedId(asset.id);
      notifyTimelineChanged();
    } catch (e) {
      setImportError(e instanceof IpcError ? e.message : "Import failed unexpectedly.");
    } finally {
      setImportBusy(false);
    }
  }

  /** Insert the asset as a clip at the playhead on the first video track. */
  async function insertAtPlayhead(asset: MediaAsset): Promise<void> {
    if (!backend) {
      setActionError("Insert needs the desktop backend (running in the browser).");
      return;
    }
    setActionBusy(asset.id);
    setActionError(null);
    try {
      const timeline = await invokeCommand<Timeline>("timeline_get");
      const track: Track | undefined =
        timeline.tracks.find((t) => t.kind === "video") ?? timeline.tracks[0];
      if (!track) throw new Error("The timeline has no tracks yet.");
      let duration = asset.duration;
      if (duration === null) {
        const probed = await invokeCommand<MediaInfo>("probe_media", {
          relativePath: asset.relative_path,
        });
        duration = probed.duration;
      }
      if (duration === null || !(duration > 0)) {
        throw new Error(`Duration unknown for ${asset.file_name} — probe it first.`);
      }
      await invokeCommand("timeline_add_clip", {
        clip: {
          id: crypto.randomUUID(),
          source_media_id: asset.id,
          track_id: track.id,
          start: round3(Math.max(0, currentTime)),
          duration,
          in_point: 0,
          volume: 1,
          muted: false,
          name: asset.file_name,
        },
      });
      notifyTimelineChanged();
    } catch (e) {
      setActionError(e instanceof IpcError ? e.message : (e as Error).message);
    } finally {
      setActionBusy(null);
    }
  }

  async function copyAssetId(asset: MediaAsset): Promise<void> {
    try {
      await navigator.clipboard.writeText(asset.id);
    } catch {
      setActionError("Clipboard unavailable in this context.");
    }
  }

  function probeDetails(asset: MediaAsset): void {
    setPath(asset.relative_path);
    setProbeOpen(true);
    void runProbe(asset.relative_path);
  }

  function menuItems(asset: MediaAsset): ContextMenuItem[] {
    return [
      {
        id: "insert",
        label: "Insert at playhead",
        run: () => void insertAtPlayhead(asset),
      },
      {
        id: "transcribe",
        label: "Transcribe…",
        run: () => onTranscribeAsset(asset.id),
      },
      {
        id: "copy",
        label: "Copy asset id",
        run: () => void copyAssetId(asset),
      },
      {
        id: "probe",
        label: "Probe details",
        run: () => probeDetails(asset),
      },
    ];
  }

  const filtered = filterAssets(assets, query);
  const selected = selectedId ? (assets.find((a) => a.id === selectedId) ?? null) : null;

  return (
    <Panel title="Media" className="flex-1">
      <div className="flex items-center gap-2">
        <div className="relative min-w-0 flex-1">
          <span className="pointer-events-none absolute left-2 top-1/2 -translate-y-1/2 text-avid-muted">
            <Icon name="search" size={13} />
          </span>
          <input
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search media…"
            aria-label="Search media"
            className="h-7 w-full rounded-avid-md border border-avid-border bg-avid-raised pl-7 pr-2 text-xs text-avid-primary placeholder:text-avid-muted focus-visible:outline-2 focus-visible:outline-avid-accent"
          />
        </div>
        <span className="flex gap-0.5" role="group" aria-label="Library view">
          {(["grid", "list"] as const).map((mode) => (
            <button
              key={mode}
              onClick={() => setView(mode)}
              aria-pressed={view === mode}
              title={`${mode} view`}
              className={`rounded-avid-sm p-1.5 ${
                view === mode ? "bg-avid-raised text-avid-primary" : "text-avid-muted hover:text-avid-secondary"
              }`}
            >
              {mode === "grid" ? <Icon name="grid" size={13} /> : <Icon name="list" size={13} />}
            </button>
          ))}
        </span>
        <button
          onClick={() => setImportOpen((v) => !v)}
          aria-expanded={importOpen}
          title="Import media into the project"
          className="rounded-avid-sm p-1.5 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
        >
          <Icon name="plus" size={14} />
        </button>
      </div>

      {importOpen && (
        <form onSubmit={onImport} className="mt-2 flex flex-col gap-2 rounded-avid-md border border-avid-border-subtle p-2">
          <TextField
            label="Import file (full path on this machine)"
            placeholder="/Users/you/Movies/clip.mp4"
            value={source}
            onChange={(e) => setSource(e.target.value)}
          />
          <Button type="submit" variant="secondary" disabled={importBusy || source.trim() === ""}>
            {importBusy ? "Importing…" : "Import into project"}
          </Button>
          {importError && (
            <p role="alert" className="text-xs text-avid-danger">
              {importError}
            </p>
          )}
          {imported && (
            <p className="text-xs text-avid-success">
              Imported {imported.file_name} ({formatDuration(imported.duration)})
            </p>
          )}
        </form>
      )}

      {assetsLoading ? (
        <div className="mt-2 grid grid-cols-2 gap-2" aria-label="Loading media">
          {[0, 1, 2, 3].map((i) => (
            <div key={i}>
              <div className="avid-skeleton aspect-video w-full" />
              <div className="avid-skeleton mx-0.5 mb-1 mt-1.5 h-3 w-2/3" />
            </div>
          ))}
        </div>
      ) : assets.length === 0 ? (
        <div className="mt-2">
          <EmptyState
            title="No media yet"
            body={backend ? "Import footage to build the library." : "Media needs the desktop backend. Import runs in the AVID app."}
            actions={
              backend ? (
                <Button variant="secondary" onClick={() => setImportOpen(true)}>
                  Import media
                </Button>
              ) : undefined
            }
          />
        </div>
      ) : filtered.length === 0 ? (
        <p className="mt-2 text-xs text-avid-muted">No media matches “{query}”.</p>
      ) : view === "grid" ? (
        <ul className="mt-2 grid grid-cols-2 gap-2" aria-label={`Project media, ${filtered.length} items`}>
          {filtered.map((asset) => (
            <li key={asset.id}>
              <ContextArea items={menuItems(asset)}>
                {(open) => (
                  <div
                    role="button"
                    tabIndex={0}
                    aria-pressed={selectedId === asset.id}
                    aria-label={`${asset.file_name}${selectedId === asset.id ? ", selected" : ""}`}
                    onClick={() => setSelectedId(asset.id)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter" || e.key === " ") {
                        e.preventDefault();
                        setSelectedId(asset.id);
                      }
                    }}
                    onContextMenu={(e) => {
                      setSelectedId(asset.id);
                      open(e);
                    }}
                    className="flex cursor-pointer flex-col gap-1 text-left"
                  >
                    <div
                      className={`relative aspect-video w-full overflow-hidden rounded-avid-md bg-avid-raised ${
                        selectedId === asset.id ? "ring-2 ring-inset ring-avid-accent" : ""
                      }`}
                    >
                      {thumbs[asset.id] ? (
                        <img
                          src={streamUrl("project", thumbs[asset.id] as string)}
                          alt=""
                          aria-hidden
                          className="h-full w-full object-cover"
                          loading="lazy"
                        />
                      ) : (
                        <span className="flex h-full items-center justify-center text-avid-muted">
                          <Icon name={asset.dimensions ? "film" : "music"} size={22} />
                        </span>
                      )}
                      {asset.duration !== null && (
                        <span className="absolute left-1 top-1 rounded bg-black/70 px-1 font-mono text-[10px] text-white">
                          {formatDuration(asset.duration)}
                        </span>
                      )}
                    </div>
                    <span className="truncate text-xs text-avid-primary" title={asset.file_name}>
                      {asset.file_name}
                    </span>
                  </div>
                )}
              </ContextArea>
            </li>
          ))}
        </ul>
      ) : (
        <ul className="mt-2 flex flex-col gap-1" aria-label={`Project media, ${filtered.length} items`}>
          {filtered.map((asset) => (
            <li
              key={asset.id}
              className={`flex items-center gap-2 rounded-avid-sm px-2 py-1.5 ${
                selectedId === asset.id ? "bg-avid-raised" : "hover:bg-avid-raised/60"
              }`}
            >
              <button onClick={() => setSelectedId(asset.id)} className="flex min-w-0 flex-1 items-center gap-2 text-left">
                <Icon name={asset.dimensions ? "film" : "music"} size={14} className="shrink-0 text-avid-muted" />
                <span className="min-w-0">
                  <span className="block truncate text-xs font-medium text-avid-primary">{asset.file_name}</span>
                  <span className="block font-mono text-[10px] text-avid-muted">{formatDuration(asset.duration)}</span>
                </span>
              </button>
              <button
                onClick={() => void insertAtPlayhead(asset)}
                disabled={actionBusy === asset.id}
                title={`Insert ${asset.file_name} at playhead`}
                className="shrink-0 rounded-avid-sm px-1.5 py-1 text-xs text-avid-accent hover:bg-avid-accent-muted disabled:opacity-50"
              >
                {actionBusy === asset.id ? "…" : "Insert"}
              </button>
            </li>
          ))}
        </ul>
      )}

      {selected && (
        <div className="mt-2 rounded-avid-md border border-avid-border-subtle p-2">
          <p className="truncate text-xs font-medium text-avid-primary" title={selected.file_name}>
            {selected.file_name}
          </p>
          <p className="mt-0.5 font-mono text-[10px] text-avid-muted">
            {formatDuration(selected.duration)}
            {selected.dimensions ? ` · ${selected.dimensions[0]}×${selected.dimensions[1]}` : ""}
          </p>
          <div className="mt-1.5 flex flex-wrap gap-1">
            <button
              onClick={() => void insertAtPlayhead(selected)}
              disabled={actionBusy === selected.id}
              className="rounded-avid-sm bg-avid-raised px-2 py-1 text-xs text-avid-primary hover:bg-avid-overlay disabled:opacity-50"
            >
              {actionBusy === selected.id ? "Inserting…" : "Insert at playhead"}
            </button>
            <button
              onClick={() => onTranscribeAsset(selected.id)}
              className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised"
            >
              Transcribe…
            </button>
            <button
              onClick={() => void copyAssetId(selected)}
              className="rounded-avid-sm px-2 py-1 text-xs text-avid-secondary hover:bg-avid-raised"
            >
              Copy id
            </button>
          </div>
        </div>
      )}
      {actionError && (
        <p role="alert" className="mt-2 text-xs text-avid-danger">
          {actionError}
        </p>
      )}

      <div className="mt-2 border-t border-avid-border-subtle pt-2">
        <button
          onClick={() => setProbeOpen((v) => !v)}
          aria-expanded={probeOpen}
          className="text-xs text-avid-muted hover:text-avid-secondary"
        >
          {probeOpen ? "▾" : "▸"} Probe a project file
        </button>
        {probeOpen && (
          <form onSubmit={onProbe} className="mt-2 flex flex-col gap-2">
            <TextField
              label="Project-relative file"
              placeholder="media/clip.mp4"
              value={path}
              onChange={(e) => setPath(e.target.value)}
            />
            <Button type="submit" variant="secondary" disabled={probeBusy || path.trim() === ""}>
              {probeBusy ? "Probing…" : "Probe file"}
            </Button>
            {probeError && (
              <p role="alert" className="text-xs text-avid-danger">
                {probeError}
              </p>
            )}
            {info && (
              <dl className="flex flex-col gap-1 text-xs">
                <div className="flex justify-between">
                  <dt className="text-avid-muted">Duration</dt>
                  <dd className="font-mono text-avid-primary">{formatDuration(info.duration)}</dd>
                </div>
                <div className="flex justify-between">
                  <dt className="text-avid-muted">Format</dt>
                  <dd className="font-mono text-avid-primary">{info.format || "—"}</dd>
                </div>
                {info.streams.map((stream) => (
                  <div key={stream.index} className="flex justify-between gap-2">
                    <dt className="shrink-0 text-avid-muted">
                      {stream.codec_type || "stream"} #{stream.index}
                    </dt>
                    <dd className="truncate font-mono text-avid-primary">
                      {stream.codec_name}
                      {stream.width && stream.height ? ` · ${stream.width}×${stream.height}` : ""}
                      {stream.sample_rate ? ` · ${stream.sample_rate}Hz` : ""}
                    </dd>
                  </div>
                ))}
              </dl>
            )}
          </form>
        )}
      </div>
    </Panel>
  );
}
