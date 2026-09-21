import type { VisualSceneSpec } from "./index";

/** Escape text for SVG content/attributes. */
export function escapeXml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

const NODE_W = 180;
const NODE_H = 64;

/**
 * Render a VisualSceneSpec to a standalone SVG string (deterministic:
 * same spec → same markup). Nodes are rounded rects with centered labels,
 * text elements are plain labels, connections are lines with arrowheads.
 * All user text is XML-escaped — scene content can never break the markup.
 */
export function renderSceneSvg(spec: VisualSceneSpec): string {
  const { width, height } = spec.scene;
  const parts: string[] = [
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}" role="img">`,
    `<defs><marker id="avid-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M 0 1 L 9 5 L 0 9" fill="none" stroke="#4f8cff" stroke-width="1.5"/></marker></defs>`,
    `<rect x="0" y="0" width="${width}" height="${height}" fill="#0b0d10"/>`,
  ];
  const centers = new Map<string, { x: number; y: number }>();

  for (const element of spec.elements) {
    if (element.type === "node") {
      centers.set(element.id, { x: element.x + NODE_W / 2, y: element.y + NODE_H / 2 });
      parts.push(
        `<g data-node-id="${escapeXml(element.id)}">` +
          `<rect x="${element.x}" y="${element.y}" width="${NODE_W}" height="${NODE_H}" rx="8" fill="#181c22" stroke="#4f8cff" stroke-width="1.5"/>` +
          `<text x="${element.x + NODE_W / 2}" y="${element.y + NODE_H / 2 + 5}" text-anchor="middle" font-family="Inter, system-ui, sans-serif" font-size="14" fill="#e8eaed">${escapeXml(element.label)}</text>` +
          `</g>`,
      );
    } else {
      centers.set(element.id, { x: element.x, y: element.y });
      const size = element.size ?? 14;
      parts.push(
        `<text data-text-id="${escapeXml(element.id)}" x="${element.x}" y="${element.y}" font-family="Inter, system-ui, sans-serif" font-size="${size}" fill="#a7b0bb">${escapeXml(element.text)}</text>`,
      );
    }
  }

  for (const connection of spec.connections) {
    const from = centers.get(connection.from);
    const to = centers.get(connection.to);
    if (!from || !to) continue;
    const color = connection.animation === "flow" ? "#4f8cff" : "#455060";
    const dash = connection.animation === "flow" ? "" : ' stroke-dasharray="5,4"';
    parts.push(
      `<line x1="${from.x}" y1="${from.y}" x2="${to.x}" y2="${to.y}" stroke="${color}" stroke-width="1.5"${dash} marker-end="url(#avid-arrow)"/>`,
    );
  }
  parts.push("</svg>");
  return parts.join("");
}
