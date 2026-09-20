import { describe, expect, it } from "vitest";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { validateTemplate } from "./index";

const TEMPLATES_DIR = new URL("../../../templates", import.meta.url).pathname;

describe("validateTemplate", () => {
  it("accepts a full manifest", () => {
    const result = validateTemplate({
      name: "technical-explainer",
      version: "0.1.0",
      description: "Fast-paced technical explainer.",
      category: "education",
      aspectRatios: ["16:9"],
    });
    expect(result.ok).toBe(true);
  });

  it("rejects bad versions, categories, and aspects without throwing", () => {
    expect(validateTemplate({ name: "x", version: "soon", description: "d", category: "education", aspectRatios: ["16:9"] }).ok).toBe(false);
    expect(validateTemplate({ name: "x", version: "1.0.0", description: "d", category: "vlog", aspectRatios: ["16:9"] }).ok).toBe(false);
    expect(validateTemplate({ name: "x", version: "1.0.0", description: "d", category: "education", aspectRatios: ["21:9"] }).ok).toBe(false);
    expect(validateTemplate(null).ok).toBe(false);
  });

  it("every shipped template.json validates", () => {
    const entries = readdirSync(TEMPLATES_DIR, { withFileTypes: true }).filter((e) => e.isDirectory());
    expect(entries.length).toBeGreaterThanOrEqual(12);
    for (const entry of entries) {
      const raw = readFileSync(join(TEMPLATES_DIR, entry.name, "template.json"), "utf8");
      const result = validateTemplate(JSON.parse(raw) as unknown);
      expect(result.errors).toEqual([]);
      expect(result.ok).toBe(true);
    }
  });
});
