import { describe, expect, it } from "vitest";
import { streamUrl } from "./components/PreviewPane";

describe("streamUrl", () => {
  it("builds scoped stream URLs with encoded segments", () => {
    expect(streamUrl("project", "media/clip.mp4")).toBe("stream://localhost/project/media/clip.mp4");
    expect(streamUrl("cache", "avid-audio/x.wav")).toBe("stream://localhost/cache/avid-audio/x.wav");
    expect(streamUrl("project", "media/my clip (1).mp4")).toBe(
      "stream://localhost/project/media/my%20clip%20(1).mp4",
    );
  });
});
