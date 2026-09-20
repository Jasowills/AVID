import { describe, expect, it } from "vitest";
import { validateEditPlan, validateSceneSpec } from "./index";

const CTX = { mediaDuration: 600 } as const;

describe("validateEditPlan", () => {
  it("accepts the AGENTS §25 example shape", () => {
    const result = validateEditPlan(
      {
        version: 1,
        goal: "technical_explainer",
        operations: [
          { type: "remove_range", start: 42.2, end: 47.1, reason: "repetition" },
          { type: "add_visual", visualType: "diagram", start: 84.2, duration: 8, concept: "kafka_partitions" },
        ],
      },
      CTX,
    );
    expect(result.ok).toBe(true);
    expect(result.plan?.operations).toHaveLength(2);
  });

  it("rejects prose, malformed JSON, and wrong envelopes", () => {
    for (const bad of ["Sure thing!", "{nope", [1, 2], null, 42]) {
      const result = validateEditPlan(bad, CTX);
      expect(result.ok).toBe(false);
      expect(result.plan).toBeNull();
      expect(result.errors.length).toBeGreaterThan(0);
    }
  });

  it("rejects missing fields and wrong versions", () => {
    expect(validateEditPlan({ version: 2, goal: "x", operations: [] }, CTX).ok).toBe(false);
    expect(validateEditPlan({ version: 1, operations: [] }, CTX).ok).toBe(false);
    expect(validateEditPlan({ version: 1, goal: "x" }, CTX).ok).toBe(false);
  });

  it("rejects invalid timestamps (NaN, negative, inverted, beyond duration)", () => {
    const cases = [
      { type: "remove_range", start: -1, end: 5, reason: "x" },
      { type: "remove_range", start: 10, end: 5, reason: "x" },
      { type: "remove_range", start: 10, end: 10, reason: "x" },
      { type: "remove_range", start: 0, end: 601, reason: "x" },
      { type: "remove_range", start: Number.NaN, end: 5, reason: "x" },
      { type: "remove_range", start: 0, end: 5, reason: "  " },
    ];
    for (const op of cases) {
      const result = validateEditPlan({ version: 1, goal: "g", operations: [op] }, CTX);
      expect(result.ok).toBe(false);
    }
  });

  it("rejects unknown op types, bad visuals, and hallucinated clip ids", () => {
    const unknown = validateEditPlan(
      { version: 1, goal: "g", operations: [{ type: "teleport_clip" }] },
      CTX,
    );
    expect(unknown.ok).toBe(false);

    const badVisual = validateEditPlan(
      { version: 1, goal: "g", operations: [{ type: "add_visual", visualType: "hologram", start: 0, duration: 5, concept: "x" }] },
      CTX,
    );
    expect(badVisual.ok).toBe(false);

    const hallucinated = validateEditPlan(
      { version: 1, goal: "g", operations: [{ type: "split_clip", clipId: "ghost", at: 5 }] },
      { ...CTX, knownClipIds: ["a", "b"] },
    );
    expect(hallucinated.ok).toBe(false);
    const known = validateEditPlan(
      { version: 1, goal: "g", operations: [{ type: "split_clip", clipId: "a", at: 5 }] },
      { ...CTX, knownClipIds: ["a", "b"] },
    );
    expect(known.ok).toBe(true);
  });

  it("accepts JSON strings and trims text fields", () => {
    const result = validateEditPlan(
      JSON.stringify({
        version: 1,
        goal: "g",
        operations: [{ type: "add_caption", start: 1, end: 3, text: "  Hello  " }],
      }),
      CTX,
    );
    expect(result.ok).toBe(true);
    expect(result.plan?.operations[0]).toMatchObject({ text: "Hello" });
  });
});

describe("validateSceneSpec", () => {
  const scene = {
    scene: { width: 1920, height: 1080, duration: 8 },
    elements: [
      { type: "node", id: "producer", x: 200, y: 450, label: "Producer" },
      { type: "node", id: "kafka", x: 800, y: 450, label: "Kafka" },
    ],
    connections: [{ from: "producer", to: "kafka", animation: "flow" }],
  };

  it("accepts a valid scene", () => {
    const result = validateSceneSpec(scene);
    expect(result.ok).toBe(true);
    expect(result.spec?.connections).toHaveLength(1);
  });

  it("rejects dangling connections, duplicate ids, and bad geometry", () => {
    const dangling = validateSceneSpec({ ...scene, connections: [{ from: "ghost", to: "kafka" }] });
    expect(dangling.ok).toBe(false);

    const dup = validateSceneSpec({ ...scene, elements: [...scene.elements, { ...scene.elements[0] }] });
    expect(dup.ok).toBe(false);

    const flat = validateSceneSpec({ ...scene, scene: { width: 0, height: 1080, duration: 8 } });
    expect(flat.ok).toBe(false);
  });

  it("rejects malformed input without throwing", () => {
    for (const bad of ["nope", 42, null, { scene: {} }]) {
      expect(validateSceneSpec(bad).ok).toBe(false);
    }
  });
});
