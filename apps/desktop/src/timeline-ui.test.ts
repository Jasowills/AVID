import { describe, expect, it } from "vitest";
import type { Timeline } from "@avid/shared-types";
import { kindFill, layoutTimeline } from "./components/timelineLayout";

const TIMELINE: Timeline = {
  tracks: [
    { id: "v1", kind: "video", index: 0, name: "V1", locked: false, muted: false },
    { id: "a1", kind: "audio", index: 0, name: "A1", locked: false, muted: false },
  ],
  clips: {
    a: { id: "a", source_media_id: "m", track_id: "v1", start: 0, duration: 10, in_point: 0, volume: 1, muted: false, name: "A" },
    b: { id: "b", source_media_id: "m", track_id: "nope", start: 5, duration: 2, in_point: 0, volume: 0.5, muted: false, name: "B" },
  },
};

describe("layoutTimeline", () => {
  it("positions clips by time and lane", () => {
    const { rects, duration } = layoutTimeline(TIMELINE, "a");
    expect(duration).toBe(10);
    const a = rects.find((r) => r.id === "a");
    expect(a).toMatchObject({ lane: 0, selected: true });
    expect(a?.x).toBeGreaterThanOrEqual(64);
    expect(a?.width).toBe(10 * 24);
  });

  it("falls back to lane 0 for unknown tracks and never negative lanes", () => {
    const { rects } = layoutTimeline(TIMELINE, null);
    expect(rects.find((r) => r.id === "b")?.lane).toBe(0);
    expect(rects.find((r) => r.id === "b")?.selected).toBe(false);
  });

  it("handles empty timelines without crashing", () => {
    const layout = layoutTimeline({ tracks: [], clips: {} }, null);
    expect(layout.rects).toEqual([]);
    expect(layout.duration).toBe(0);
  });
});

describe("kindFill", () => {
  it("maps every known kind to a fill", () => {
    for (const kind of ["video", "audio", "text", "graphics", "caption", "unknown"]) {
      expect(kindFill(kind)).toMatch(/^#[0-9a-f]{6}$/);
    }
  });
});
