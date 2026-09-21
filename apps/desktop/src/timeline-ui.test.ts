import { describe, expect, it } from "vitest";
import type { Timeline } from "@avid/shared-types";
import { formatRulerLabel, kindFill, layoutTimeline, peaksToPath, rulerStep, rulerTicks, snapTime } from "./components/timelineLayout";

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

describe("rulerTicks", () => {
  it("covers the duration at readable steps", () => {
    expect(rulerStep(60, 24)).toBe(5);
    expect(rulerTicks(10, 5)).toEqual([0, 5, 10]);
    expect(rulerTicks(0, 5)).toEqual([]);
    expect(formatRulerLabel(65)).toBe("1:05");
    expect(formatRulerLabel(5)).toBe("0:05");
  });
});

describe("snapTime", () => {
  it("snaps within threshold and leaves distant times alone", () => {
    expect(snapTime(5.05, [0, 10], 24)).toEqual({ time: 5.05, snapped: false });
    expect(snapTime(9.9, [0, 10], 24)).toEqual({ time: 10, snapped: true });
    expect(snapTime(0.1, [10], 24)).toEqual({ time: 0, snapped: true });
  });
});

describe("peaksToPath", () => {
  it("draws a mirrored polygon scaled to the rect", () => {
    const path = peaksToPath([0, 0.5, 1], 10, 20, 100, 30);
    expect(path.startsWith("M 10")).toBe(true);
    expect(path.endsWith("Z")).toBe(true);
    // Peak 1.0 reaches near the top edge (mid 35, amp 14).
    expect(path).toContain("21.0");
  });

  it("renders silence as a flat midline and never empties", () => {
    expect(peaksToPath([], 10, 20, 100, 30)).toBe("M 10 35 L 110 35");
    expect(peaksToPath([0, 0], 10, 20, 100, 30)).toContain("35.0");
  });
});
