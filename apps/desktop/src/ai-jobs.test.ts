import { describe, expect, it } from "vitest";
import { describeEditOperation, formatTimecode, matchDirectorIntent } from "./components/AiPanel";
import { runningCount } from "./stores/useJobsStore";
import type { JobRecord } from "@avid/shared-types";

describe("describeEditOperation", () => {
  it("renders every operation kind as a review line", () => {
    expect(describeEditOperation({ type: "remove_range", start: 10, end: 15, reason: "silence" })).toBe(
      "REMOVE 00:10.0 → 00:15.0 — silence",
    );
    expect(
      describeEditOperation({ type: "add_visual", visualType: "diagram", start: 1, duration: 2, concept: "x" }),
    ).toContain("VISUAL diagram");
    expect(describeEditOperation({ type: "add_caption", start: 1, end: 2, text: "Hi" })).toContain("CAPTION");
    expect(describeEditOperation({ type: "split_clip", clipId: "a", at: 3 })).toBe("SPLIT a @ 00:03.0");
  });
});

describe("matchDirectorIntent", () => {
  it("routes editing language to real operations, everything else to help", () => {
    expect(matchDirectorIntent("remove the dead air")).toBe("cut");
    expect(matchDirectorIntent("Clean up the umms and long pauses")).toBe("cut");
    expect(matchDirectorIntent("validate this plan JSON")).toBe("validate");
    expect(matchDirectorIntent("check my plan")).toBe("validate");
    expect(matchDirectorIntent("what can you do?")).toBe("help");
    expect(matchDirectorIntent("make it viral")).toBe("help");
  });
});

describe("formatTimecode", () => {
  it("renders mm:ss.d timecodes for review rows", () => {
    expect(formatTimecode(0)).toBe("00:00.0");
    expect(formatTimecode(12.4)).toBe("00:12.4");
    expect(formatTimecode(72.44)).toBe("01:12.4");
    expect(formatTimecode(-3)).toBe("00:00.0");
  });
});

describe("runningCount", () => {
  const job = (status: JobRecord["status"]): JobRecord => ({
    id: status,
    kind: "render",
    label: status,
    status,
    progress: null,
    message: null,
  });

  it("counts only running jobs", () => {
    expect(runningCount([job("running"), job("running"), job("finished"), job("failed")])).toBe(2);
    expect(runningCount([])).toBe(0);
  });
});
