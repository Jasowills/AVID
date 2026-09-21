import { describe, expect, it } from "vitest";
import { filterAssets, formatDuration } from "./components/MediaPanel";
import type { MediaAsset } from "@avid/shared-types";

const ASSETS: MediaAsset[] = [
  { id: "a1", file_name: "Kafka Talk.mp4", relative_path: "media/a.mp4", duration: 60, dimensions: [1920, 1080], hash: null, proxy_path: null },
  { id: "b2", file_name: "intro.wav", relative_path: "media/b.wav", duration: 5, dimensions: null, hash: null, proxy_path: null },
];

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


describe("filterAssets", () => {
  it("matches file names case-insensitively", () => {
    expect(filterAssets(ASSETS, "kafka").map((a) => a.id)).toEqual(["a1"]);
    expect(filterAssets(ASSETS, "WAV").map((a) => a.id)).toEqual(["b2"]);
  });

  it("matches ids and returns everything on blank queries", () => {
    expect(filterAssets(ASSETS, "b2").map((a) => a.id)).toEqual(["b2"]);
    expect(filterAssets(ASSETS, "  ")).toHaveLength(2);
    expect(filterAssets(ASSETS, "zzz")).toHaveLength(0);
  });
});
