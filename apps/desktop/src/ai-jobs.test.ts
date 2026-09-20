import { describe, expect, it } from "vitest";
import { describeEditOperation } from "./components/AiPanel";
import { runningCount } from "./stores/useJobsStore";
import type { JobRecord } from "@avid/shared-types";

describe("describeEditOperation", () => {
  it("renders every operation kind as a review line", () => {
    expect(describeEditOperation({ type: "remove_range", start: 10, end: 15, reason: "silence" })).toBe(
      "REMOVE 10s → 15s — silence",
    );
    expect(
      describeEditOperation({ type: "add_visual", visualType: "diagram", start: 1, duration: 2, concept: "x" }),
    ).toContain("VISUAL diagram");
    expect(describeEditOperation({ type: "add_caption", start: 1, end: 2, text: "Hi" })).toContain("CAPTION");
    expect(describeEditOperation({ type: "split_clip", clipId: "a", at: 3 })).toBe("SPLIT a @ 3s");
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
