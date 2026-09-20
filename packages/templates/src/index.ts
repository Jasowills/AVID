/**
 * @avid/templates — template loading + validation (Phase 9 core, built early
 * so AI and UI work against the real format from the start).
 *
 * Templates encode editing BEHAVIOR (caption style, visual rules, pacing),
 * not just visuals. Applying a template produces undoable commands —
 * "use as a starting point", never a destructive rewrite (AGENTS §34).
 */

export interface TemplateManifest {
  name: string;
  version: string;
  description: string;
  category: string;
  aspectRatios: string[];
  supportedFeatures?: string[];
  requiredAssets?: string[];
  rules?: TemplateRules;
}

export interface TemplateRules {
  captions?: { style: string; position: string; wordHighlight: boolean };
  visuals?: { preferDiagramsFor: string[]; maxPerMinute: number };
  pacing?: { targetCutSeconds: [number, number]; removeFillers: boolean };
}

export interface TemplateIssue {
  path: string;
  message: string;
}

const KNOWN_CATEGORIES = [
  "education",
  "youtube",
  "podcast",
  "short",
  "marketing",
  "documentary",
  "general",
] as const;

const KNOWN_ASPECTS = ["16:9", "9:16", "1:1", "4:5"] as const;

/** Validate an unknown value as a template manifest. Never throws. */
export function validateTemplate(input: unknown): { ok: boolean; manifest: TemplateManifest | null; errors: TemplateIssue[] } {
  const errors: TemplateIssue[] = [];
  if (typeof input !== "object" || input === null || Array.isArray(input)) {
    return { ok: false, manifest: null, errors: [{ path: "", message: "Template must be a JSON object." }] };
  }
  const obj = input as Record<string, unknown>;
  const fail = (path: string, message: string): void => {
    errors.push({ path, message });
  };

  if (typeof obj["name"] !== "string" || (obj["name"] as string).trim() === "") fail("/name", "Template needs a name.");
  if (typeof obj["version"] !== "string" || !/^\d+\.\d+\.\d+$/.test(obj["version"] as string))
    fail("/version", "Version must be semver (e.g. 1.0.0).");
  if (typeof obj["description"] !== "string" || (obj["description"] as string).trim() === "")
    fail("/description", "Template needs a description.");
  if (!KNOWN_CATEGORIES.includes(obj["category"] as (typeof KNOWN_CATEGORIES)[number]))
    fail("/category", `Category must be one of: ${KNOWN_CATEGORIES.join(", ")}.`);
  if (!Array.isArray(obj["aspectRatios"]) || (obj["aspectRatios"] as unknown[]).length === 0)
    fail("/aspectRatios", "Template needs at least one aspect ratio.");
  else {
    for (const aspect of obj["aspectRatios"] as unknown[]) {
      if (!KNOWN_ASPECTS.includes(aspect as (typeof KNOWN_ASPECTS)[number])) {
        fail("/aspectRatios", `Unknown aspect ratio: ${String(aspect)}.`);
        break;
      }
    }
  }

  if (errors.length > 0) return { ok: false, manifest: null, errors };
  return { ok: true, manifest: obj as unknown as TemplateManifest, errors: [] };
}
