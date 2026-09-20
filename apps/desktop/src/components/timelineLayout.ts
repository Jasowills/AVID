import type { Timeline } from "@avid/shared-types";

/** Pixels per second at zoom 1. */
export const PX_PER_SECOND = 24;
/** Row height per track lane. */
export const LANE_HEIGHT = 40;
/** Left gutter for track names. */
export const GUTTER_WIDTH = 64;

export interface ClipRect {
  id: string;
  name: string;
  x: number;
  width: number;
  lane: number;
  kind: string;
  selected: boolean;
}

/** Pure layout: timeline + selection → positioned clip rects. Unit-tested. */
export function layoutTimeline(
  timeline: Timeline,
  selectedId: string | null,
  zoom = 1,
): { rects: ClipRect[]; totalWidth: number; totalHeight: number; duration: number } {
  const scale = PX_PER_SECOND * zoom;
  const lanes = timeline.tracks;
  const duration = Object.values(timeline.clips).reduce(
    (max, clip) => Math.max(max, clip.start + clip.duration),
    0,
  );
  const rects: ClipRect[] = Object.values(timeline.clips).map((clip) => {
    const lane = Math.max(
      0,
      lanes.findIndex((track) => track.id === clip.track_id),
    );
    const track = lanes[lane];
    return {
      id: clip.id,
      name: clip.name,
      x: GUTTER_WIDTH + clip.start * scale,
      width: Math.max(4, clip.duration * scale),
      lane,
      kind: track?.kind ?? "video",
      selected: clip.id === selectedId,
    };
  });
  return {
    rects,
    totalWidth: GUTTER_WIDTH + Math.max(duration * scale, 200),
    totalHeight: Math.max(lanes.length, 1) * LANE_HEIGHT + 28,
    duration,
  };
}

/** Clip fill per track kind (pairs with the Rust timeline colors). */
export function kindFill(kind: string): string {
  switch (kind) {
    case "audio":
      return "#3f7d5d";
    case "text":
      return "#7a6a3f";
    case "graphics":
      return "#6a4f8a";
    case "caption":
      return "#4f8a84";
    default:
      return "#3a5f8a";
  }
}
