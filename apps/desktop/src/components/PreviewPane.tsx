import { useEffect, useRef, useState } from "react";
import type { Clip, MediaAsset, Timeline } from "@avid/shared-types";
import { Icon } from "@avid/ui";
import { invokeCommand, isTauri } from "../lib/ipc";
import { TIMELINE_CHANGED_EVENT } from "../stores/useJobsStore";
import { usePlaybackStore } from "../stores/usePlaybackStore";

/**
 * Build a `stream://` URL for the backend Range-capable protocol.
 * Each path segment is encoded so spaces and unicode survive; the backend
 * decodes and enforces project/cache containment (traversal → 403).
 */
export function streamUrl(scope: "project" | "cache", relativePath: string): string {
  const encoded = relativePath
    .split("/")
    .map((segment) => encodeURIComponent(segment))
    .join("/");
  return `stream://localhost/${scope}/${encoded}`;
}

export interface PreviewPaneProps {
  clipId: string | null;
}

/**
 * Preview pane (Phase 2 UI slice): plays the selected clip's media — proxy
 * when generated, original otherwise — through the seekable `stream://`
 * protocol. Falls back to the first video clip; empty timelines guide.
 */
export function PreviewPane({ clipId }: PreviewPaneProps) {
  const [src, setSrc] = useState<string | null>(null);
  const [label, setLabel] = useState<string>("Preview");
  const [notice, setNotice] = useState<string | null>(null);
  const [time, setTime] = useState(0);
  const [playing, setPlaying] = useState(false);
  const videoRef = useRef<HTMLVideoElement>(null);
  const seekRequest = usePlaybackStore((state) => state.seekRequest);
  const reportTime = usePlaybackStore((state) => state.reportTime);
  const backend = isTauri();

  // Outward seeks (ruler, transcript): apply to the video element.
  useEffect(() => {
    const video = videoRef.current;
    if (video && seekRequest && Number.isFinite(seekRequest.time)) {
      try {
        video.currentTime = seekRequest.time;
      } catch {
        // Not yet seekable (metadata pending) — the next request retries.
      }
    }
  }, [seekRequest]);

  useEffect(() => {
    if (!backend) return;
    let cancelled = false;
    const load = async (): Promise<void> => {
      try {
        const [timeline, assets] = await Promise.all([
          invokeCommand<Timeline>("timeline_get"),
          invokeCommand<MediaAsset[]>("list_assets"),
        ]);
        if (cancelled) return;
        const byId = new Map(assets.map((asset) => [asset.id, asset]));
        const clips = Object.values(timeline.clips);
        const selected: Clip | undefined =
          (clipId ? timeline.clips[clipId] : undefined) ??
          clips.find((clip) => byId.get(clip.source_media_id)?.dimensions !== undefined) ??
          clips[0];
        if (!selected) {
          setSrc(null);
          setLabel("Preview");
          setNotice(null);
          return;
        }
        const asset = byId.get(selected.source_media_id);
        if (!asset) {
          setSrc(null);
          setLabel(selected.name);
          setNotice("Media for this clip is missing — relink it in a later phase.");
          return;
        }
        setLabel(selected.name);
        setNotice(null);
        setSrc(streamUrl("project", asset.proxy_path ?? asset.relative_path));
      } catch {
        if (!cancelled) {
          setSrc(null);
          setNotice(null);
        }
      }
    };
    void load();
    window.addEventListener(TIMELINE_CHANGED_EVENT, load);
    return () => {
      cancelled = true;
      window.removeEventListener(TIMELINE_CHANGED_EVENT, load);
    };
  }, [backend, clipId]);

  if (!backend) {
    return (
      <div className="flex aspect-video w-full max-w-3xl items-center justify-center rounded-avid-lg border border-avid-border bg-avid-panel">
        <p className="text-sm text-avid-muted">Preview needs the desktop backend.</p>
      </div>
    );
  }

  if (!src) {
    return (
      <div className="flex aspect-video w-full max-w-3xl items-center justify-center rounded-avid-lg border border-avid-border bg-avid-panel">
        <p className="text-sm text-avid-muted">{notice ?? "Import media to preview it here."}</p>
      </div>
    );
  }

  return (
    <div className="flex w-full max-w-3xl flex-col gap-1">
      <video
        key={src}
        ref={videoRef}
        onPlay={() => setPlaying(true)}
        onPause={() => setPlaying(false)}
        src={src}
        controls
        preload="metadata"
        aria-label={`Preview: ${label}`}
        onTimeUpdate={(event) => {
          const current = event.currentTarget.currentTime;
          setTime(current);
          reportTime(current);
        }}
        className="aspect-video w-full rounded-avid-lg border border-avid-border bg-black"
      />
      <div className="flex items-center gap-2">
        <button
          onClick={() => {
            const video = videoRef.current;
            if (!video) return;
            if (video.paused) void video.play().catch(() => undefined);
            else video.pause();
          }}
          aria-label={playing ? "Pause preview" : "Play preview"}
          className="rounded-avid-sm bg-avid-raised px-2.5 py-1 text-xs text-avid-primary hover:bg-avid-overlay"
        >
          {playing ? <Icon name="pause" size={13} /> : <Icon name="play" size={13} />}
        </button>
        <p className="text-xs text-avid-muted">
          {label}
          {src.includes("/proxies/") ? " · proxy" : " · original"} · {time.toFixed(1)}s
        </p>
      </div>
    </div>
  );
}
