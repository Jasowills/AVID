import { describe, expect, it } from "vitest";
import {
  clampTimelineHeight,
  DEFAULT_TIMELINE_HEIGHT,
  MAX_TIMELINE_HEIGHT,
  MIN_TIMELINE_HEIGHT,
} from "./stores/useLayoutStore";

describe("clampTimelineHeight", () => {
  it("clamps drag heights into the usable band", () => {
    expect(clampTimelineHeight(234)).toBe(234);
    expect(clampTimelineHeight(0)).toBe(MIN_TIMELINE_HEIGHT);
    expect(clampTimelineHeight(-50)).toBe(MIN_TIMELINE_HEIGHT);
    expect(clampTimelineHeight(9999)).toBe(MAX_TIMELINE_HEIGHT);
    expect(clampTimelineHeight(Number.NaN)).toBe(DEFAULT_TIMELINE_HEIGHT);
  });
});
