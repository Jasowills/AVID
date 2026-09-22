/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/components/timeline — Layers column) — track headers beside
 * the timeline canvas, row-aligned to the canvas lanes (28px ruler + 40px
 * lanes, see timelineLayout.ts). Lock toggles run the real command engine;
 * vertical scroll follows the canvas. */

import { useCallback, useEffect, useState } from "react";
import type { Track } from "@avid/shared-types";
import { Icon } from "@avid/ui";
import { invokeCommand, isTauri } from "../lib/ipc";
import { notifyTimelineChanged, TIMELINE_CHANGED_EVENT } from "../stores/useJobsStore";
import { LANE_HEIGHT, RULER_HEIGHT } from "./timelineLayout";

function kindGlyph(kind: Track["kind"]): string {
  switch (kind) {
    case "audio":
      return "A";
    case "text":
      return "T";
    case "graphics":
      return "G";
    case "caption":
      return "C";
    default:
      return "V";
  }
}

export function Layers({
  scrollRef,
  collapsed,
}: {
  scrollRef: { current: HTMLDivElement | null };
  collapsed?: boolean;
}) {
  const [tracks, setTracks] = useState<Track[] | null>(null);
  const backend = isTauri();

  const refresh = useCallback(async () => {
    if (!backend) {
      setTracks([]);
      return;
    }
    try {
      const timeline = await invokeCommand<{ tracks: Track[] }>("timeline_get");
      setTracks(timeline.tracks);
    } catch {
      setTracks([]);
    }
  }, [backend]);

  useEffect(() => {
    void refresh();
    window.addEventListener(TIMELINE_CHANGED_EVENT, refresh);
    return () => window.removeEventListener(TIMELINE_CHANGED_EVENT, refresh);
  }, [refresh]);

  async function toggleLock(track: Track): Promise<void> {
    try {
      await invokeCommand("timeline_set_track_locked", { trackId: track.id, locked: !track.locked });
      notifyTimelineChanged();
    } catch {
      // The timeline area surfaces command errors; headers stay truthful.
    }
  }

  return (
    <div ref={scrollRef} className="h-full overflow-hidden bg-avid-panel" aria-label="Track headers">
      <div style={{ height: RULER_HEIGHT }} className="flex items-end px-2 pb-1">
        <span className="font-mono text-[10px] text-avid-muted">
          {tracks && tracks.length > 0 ? `${tracks.length} track${tracks.length === 1 ? "" : "s"}` : ""}
        </span>
      </div>
      {tracks === null ? (
        <div className="flex flex-col gap-1 p-2" aria-label="Loading tracks">
          {[0, 1].map((i) => (
            <div key={i} className="h-10 animate-pulse rounded-avid-sm bg-avid-raised" />
          ))}
        </div>
      ) : collapsed ? null : (
        tracks.map((track) => (
          <div
            key={track.id}
            style={{ height: LANE_HEIGHT }}
            className="flex items-center gap-1.5 border-t border-avid-border-subtle px-2"
          >
            <span
              aria-hidden
              className="flex h-5 w-5 shrink-0 items-center justify-center rounded-avid-sm bg-avid-raised font-mono text-[10px] text-avid-muted"
            >
              {kindGlyph(track.kind)}
            </span>
            <span className="min-w-0 flex-1 truncate text-xs text-avid-secondary" title={track.name}>
              {track.name}
            </span>
            <button
              onClick={() => void toggleLock(track)}
              disabled={!backend}
              aria-pressed={track.locked}
              title={track.locked ? `Unlock track ${track.name}` : `Lock track ${track.name}`}
              className="shrink-0 rounded-avid-sm p-1 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary disabled:cursor-not-allowed disabled:opacity-40"
            >
              <Icon name={track.locked ? "lock" : "unlock"} size={13} />
            </button>
          </div>
        ))
      )}
    </div>
  );
}
