import { describe, expect, it } from "vitest";
import { parseMermaidFlowchart } from "./mermaid";
import { renderSceneSvg } from "./scene-svg";
import { validateSceneSpec } from "./index";

const KAFKA = `flowchart TD
  P[Producer] --> K[Kafka]
  K --> C1[Partition 1]
  K --> C2[Partition 2]
`;

describe("parseMermaidFlowchart", () => {
  it("compiles the Kafka example to a validatable spec", () => {
    const result = parseMermaidFlowchart(KAFKA);
    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
    const spec = result.spec;
    if (!spec) throw new Error("expected spec");
    expect(spec.elements).toHaveLength(4);
    expect(spec.connections).toHaveLength(3);
    // Depth layout: producer above kafka above partitions.
    const byId = new Map(spec.elements.map((el) => [el.id, el]));
    const p = byId.get("P");
    const k = byId.get("K");
    if (p?.type !== "node" || k?.type !== "node") throw new Error("expected nodes");
    expect(p.y).toBeLessThan(k.y);
    expect(p.label).toBe("Producer");
    // The emitted spec passes our own validator (pipeline coherence).
    expect(validateSceneSpec(spec).ok).toBe(true);
  });

  it("supports LR layout and bare references", () => {
    const result = parseMermaidFlowchart("flowchart LR\n  A --> B\n");
    expect(result.ok).toBe(true);
    const spec = result.spec;
    if (!spec) throw new Error("expected spec");
    expect(spec.elements.map((el) => el.id).sort()).toEqual(["A", "B"]);
    const a = spec.elements.find((el) => el.id === "A");
    const b = spec.elements.find((el) => el.id === "B");
    if (a?.type !== "node" || b?.type !== "node") throw new Error("expected nodes");
    expect(a.x).toBeLessThan(b.x);
    expect(a.label).toBe("A");
  });

  it("rejects non-flowcharts and unsupported statements cleanly", () => {
    expect(parseMermaidFlowchart("").ok).toBe(false);
    expect(parseMermaidFlowchart("graph TD\n A --> B").ok).toBe(false);
    expect(parseMermaidFlowchart("flowchart TD\n  A ==> B").ok).toBe(false);
    expect(parseMermaidFlowchart("flowchart TD\n  subgraph X\n  end").ok).toBe(false);
    const bad = parseMermaidFlowchart("flowchart TD\n  A --> B\n  nonsense (((");
    expect(bad.ok).toBe(false);
    expect(bad.errors[0]?.line).toBe(3);
  });
});

describe("renderSceneSvg", () => {
  it("renders nodes, labels, and arrows deterministically", () => {
    const parsed = parseMermaidFlowchart(KAFKA);
    if (!parsed.spec) throw new Error("expected spec");
    const first = renderSceneSvg(parsed.spec);
    expect(renderSceneSvg(parsed.spec)).toBe(first);
    expect(first).toContain("Producer");
    expect(first).toContain("Partition 2");
    expect(first).toContain("marker-end");
    expect(first).toContain('viewBox="0 0 ');
  });

  it("escapes user text so scenes cannot break markup", () => {
    const parsed = parseMermaidFlowchart("flowchart TD\n  A[<b>Bold & \"quoted\"] --> B[x < y]");
    if (!parsed.spec) throw new Error("expected spec");
    const svg = renderSceneSvg(parsed.spec);
    expect(svg).toContain("&lt;b&gt;Bold &amp; &quot;quoted&quot;");
    expect(svg).not.toContain("<b>Bold");
  });
});
