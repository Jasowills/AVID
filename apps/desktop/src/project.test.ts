import { describe, expect, it } from "vitest";
import { createProjectConfig, validateNewProject } from "./lib/project";

describe("validateNewProject", () => {
  it("accepts a complete valid input", () => {
    expect(
      validateNewProject({ name: "Kafka explainer", canvas: "16:9", frameRate: 30, resolution: "1080p" }),
    ).toEqual([]);
  });

  it("rejects a blank name", () => {
    const errors = validateNewProject({ name: "   ", canvas: "16:9", frameRate: 30, resolution: "1080p" });
    expect(errors).toEqual([{ field: "name", message: "Give the project a name." }]);
  });

  it("rejects an overlong name", () => {
    const errors = validateNewProject({ name: "x".repeat(81), canvas: "16:9", frameRate: 30, resolution: "1080p" });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("name");
  });

  it("rejects unknown canvas, frame rate, and resolution", () => {
    const errors = validateNewProject({
      name: "ok",
      // @ts-expect-error — intentionally invalid for the rejection test
      canvas: "21:9",
      // @ts-expect-error — intentionally invalid for the rejection test
      frameRate: 48,
      // @ts-expect-error — intentionally invalid for the rejection test
      resolution: "8K",
    });
    expect(errors.map((e) => e.field).sort()).toEqual(["canvas", "frameRate", "resolution"]);
  });
});

describe("createProjectConfig", () => {
  it("trims the name and stamps schema version, template, and timestamps", () => {
    const project = createProjectConfig(
      { name: "  Demo  ", canvas: "9:16", frameRate: 60, resolution: "4K", templateId: null },
      { id: "test-id", now: "2026-09-20T00:00:00.000Z" },
    );
    expect(project).toEqual({
      id: "test-id",
      version: 1,
      name: "Demo",
      canvas: "9:16",
      frameRate: 60,
      resolution: "4K",
      templateId: null,
      createdAt: "2026-09-20T00:00:00.000Z",
      updatedAt: "2026-09-20T00:00:00.000Z",
    });
  });
});
