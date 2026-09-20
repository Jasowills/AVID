import { describe, expect, it } from "vitest";
import { formatDuration } from "./components/MediaPanel";

describe("formatDuration", () => {
  it("formats seconds as m:ss.t", () => {
    expect(formatDuration(0)).toBe("0:00.0");
    expect(formatDuration(65.25)).toBe("1:05.3");
    expect(formatDuration(600)).toBe("10:00.0");
  });

  it("renders unknown durations as an em dash", () => {
    expect(formatDuration(null)).toBe("—");
    expect(formatDuration(Number.NaN)).toBe("—");
    expect(formatDuration(-1)).toBe("—");
  });
});
