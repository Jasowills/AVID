import type { SceneConnection, SceneElement, VisualSceneSpec } from "./index";

/**
 * Minimal Mermaid flowchart → VisualSceneSpec compiler (Phase 8 slice).
 *
 * Supported subset (documented limits, everything else is a clean error):
 * - `flowchart TD|TB|LR|RL|BT` header (required, first non-empty line)
 * - Node definitions: `ID[Label]`, `ID(Label)`, `ID{Label}`
 * - Bare references: `A --> B` (label defaults to the id)
 * - Edges: `A --> B` only (no other arrow forms, edge labels, subgraphs, or styling)
 *
 * Layout is deterministic auto-layout: nodes are layered by longest-path
 * depth from roots, ordered by first appearance. Hand layout stays possible
 * by editing the emitted spec (x/y are plain numbers).
 */

export interface MermaidIssue {
  line: number;
  message: string;
}

export interface MermaidResult {
  ok: boolean;
  spec: VisualSceneSpec | null;
  errors: MermaidIssue[];
}

const NODE_WIDTH = 180;
const NODE_HEIGHT = 64;
const GAP_X = 220;
const GAP_Y = 120;
const PAD = 20;

export function parseMermaidFlowchart(input: string): MermaidResult {
  const errors: MermaidIssue[] = [];
  const fail = (line: number, message: string): MermaidResult => {
    errors.push({ line, message });
    return { ok: false, spec: null, errors };
  };

  const lines = input
    .split("\n")
    .map((raw, index) => ({ text: raw.split("%%")[0]?.trim() ?? "", line: index + 1 }))
    .filter((entry) => entry.text !== "");
  if (lines.length === 0) return fail(0, "Empty diagram.");
  const header = /^flowchart\s+(TD|TB|BT|LR|RL)\s*;?\s*$/.exec(lines[0]?.text ?? "");
  if (!header) {
    return fail(lines[0]?.line ?? 1, "First line must be a flowchart header, e.g. `flowchart TD`.");
  }
  const direction = header[1] as string;
  const vertical = direction === "TD" || direction === "TB" || direction === "BT";

  const labels = new Map<string, string>();
  const order: string[] = [];
  const edges: Array<{ from: string; to: string; line: number }> = [];

  const remember = (id: string, label?: string): void => {
    if (!labels.has(id)) {
      labels.set(id, label?.trim() || id);
      order.push(id);
    } else if (label?.trim()) {
      labels.set(id, (label as string).trim());
    }
  };

  const nodePattern = /([A-Za-z0-9_]+)\s*[\[\(]\s*([^\]\)]+?)\s*[\]\)]/g;
  const edgePattern = /([A-Za-z0-9_]+)\s*-->\s*([A-Za-z0-9_]+)/g;

  for (const { text, line } of lines.slice(1)) {
    if (/^flowchart\s/.test(text)) return fail(line, "Only one flowchart header is supported.");
    let matched = false;
    for (const match of text.matchAll(nodePattern)) {
      matched = true;
      remember(match[1] as string, match[2]);
    }
    // Strip `ID[Label]` definitions first so edges sharing the line
    // (`P[Producer] --> K[Kafka]`) still match.
    const bare = text.replace(/\[[^\]]*\]|\([^)]*\)|\{[^}]*\}/g, "");
    for (const match of bare.matchAll(edgePattern)) {
      matched = true;
      const from = match[1] as string;
      const to = match[2] as string;
      remember(from);
      remember(to);
      edges.push({ from, to, line });
    }
    if (!matched) return fail(line, `Unsupported statement (only ID[Label] and A --> B): ${text}`);
  }

  if (order.length === 0) return fail(lines[1]?.line ?? 2, "No nodes found.");

  // Depth = longest path from any root (nodes with no incoming edge).
  const incoming = new Map<string, number>(order.map((id) => [id, 0]));
  for (const edge of edges) incoming.set(edge.to, (incoming.get(edge.to) ?? 0) + 1);
  const depth = new Map<string, number>(order.map((id) => [id, 0]));
  let changed = true;
  for (let pass = 0; pass < order.length && changed; pass++) {
    changed = false;
    for (const edge of edges) {
      const next = (depth.get(edge.from) ?? 0) + 1;
      if (next > (depth.get(edge.to) ?? 0)) {
        depth.set(edge.to, next);
        changed = true;
      }
    }
  }
  const byDepth = new Map<number, string[]>();
  for (const id of order) {
    const list = byDepth.get(depth.get(id) ?? 0) ?? [];
    list.push(id);
    byDepth.set(depth.get(id) ?? 0, list);
  }

  const elements: SceneElement[] = order.map((id) => {
    const layer = byDepth.get(depth.get(id) ?? 0) ?? [id];
    const slot = layer.indexOf(id);
    const x = vertical ? PAD + slot * GAP_X : PAD + (depth.get(id) ?? 0) * GAP_X;
    const y = vertical ? PAD + (depth.get(id) ?? 0) * GAP_Y : PAD + slot * GAP_Y;
    return { type: "node", id, x, y, label: labels.get(id) ?? id };
  });

  const connections: SceneConnection[] = edges.map((edge) => ({
    from: edge.from,
    to: edge.to,
    animation: "flow" as const,
  }));

  const maxDepth = Math.max(...depth.values());
  const maxSlot = Math.max(...[...byDepth.values()].map((list) => list.length));
  const width = vertical ? PAD * 2 + (maxSlot - 1) * GAP_X + NODE_WIDTH : PAD * 2 + maxDepth * GAP_X + NODE_WIDTH;
  const height = vertical ? PAD * 2 + maxDepth * GAP_Y + NODE_HEIGHT : PAD * 2 + (maxSlot - 1) * GAP_Y + NODE_HEIGHT;

  return {
    ok: true,
    spec: {
      scene: { width, height, duration: 8 },
      elements,
      connections,
    },
    errors: [],
  };
}
