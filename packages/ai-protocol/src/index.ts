/**
 * @avid/ai-protocol — Edit Plan + Visual Scene Spec schemas and validators.
 *
 * Invalid AI output must NEVER mutate the timeline (ADR-008). These pure
 * validators run before any command construction, in the app and in tests.
 * They return structured results — they never throw on AI-controlled input.
 */

export const EDIT_PLAN_VERSION = 1 as const;

export type RemoveRangeOp = {
  type: "remove_range";
  start: number;
  end: number;
  reason: string;
};

export type AddVisualOp = {
  type: "add_visual";
  visualType: "diagram" | "text" | "code" | "chart" | "callout";
  start: number;
  duration: number;
  concept: string;
};

export type AddCaptionOp = {
  type: "add_caption";
  start: number;
  end: number;
  text: string;
};

export type SplitClipOp = {
  type: "split_clip";
  clipId: string;
  at: number;
};

export type EditOperation = RemoveRangeOp | AddVisualOp | AddCaptionOp | SplitClipOp;

export interface EditPlan {
  version: typeof EDIT_PLAN_VERSION;
  goal: string;
  operations: EditOperation[];
}

/** Context the validator checks ranges and references against. */
export interface ValidationContext {
  /** Total media/timeline duration in seconds. */
  mediaDuration: number;
  /** Known clip ids (for `split_clip`); omit to skip reference checks. */
  knownClipIds?: readonly string[];
}

export interface ValidationIssue {
  /** JSON-pointer-ish location, e.g. `/operations/2/end`. */
  path: string;
  message: string;
}

export interface ValidationResult {
  ok: boolean;
  /** Normalized plan (present only when ok). */
  plan: EditPlan | null;
  errors: ValidationIssue[];
}

const VISUAL_TYPES = ["diagram", "text", "code", "chart", "callout"] as const;

function isFiniteNonNegative(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value) && value >= 0;
}

function isNonEmptyString(value: unknown): value is string {
  return typeof value === "string" && value.trim().length > 0;
}

/**
 * Validate an unknown value as an Edit Plan. Accepts parsed JSON or a JSON
 * string (prose around JSON is rejected — the adapter must return clean JSON).
 */
export function validateEditPlan(input: unknown, ctx: ValidationContext): ValidationResult {
  const errors: ValidationIssue[] = [];
  const fail = (path: string, message: string): null => {
    errors.push({ path, message });
    return null;
  };

  let root: unknown = input;
  if (typeof root === "string") {
    try {
      root = JSON.parse(root) as unknown;
    } catch {
      return { ok: false, plan: null, errors: [{ path: "", message: "Not valid JSON." }] };
    }
  }
  if (typeof root !== "object" || root === null || Array.isArray(root)) {
    return { ok: false, plan: null, errors: [{ path: "", message: "Plan must be a JSON object." }] };
  }
  const obj = root as Record<string, unknown>;

  if (obj["version"] !== EDIT_PLAN_VERSION) {
    fail("/version", `Unsupported plan version (expected ${EDIT_PLAN_VERSION}).`);
  }
  if (!isNonEmptyString(obj["goal"])) {
    fail("/goal", "Plan needs a non-empty goal.");
  }
  if (!Array.isArray(obj["operations"])) {
    fail("/operations", "Plan needs an operations array.");
  }
  if (errors.length > 0) return { ok: false, plan: null, errors };

  const operations: EditOperation[] = [];
  const ops = obj["operations"] as unknown[];
  ops.forEach((op, index) => {
    const base = `/operations/${index}`;
    if (typeof op !== "object" || op === null || Array.isArray(op)) {
      fail(base, "Operation must be an object.");
      return;
    }
    const record = op as Record<string, unknown>;
    switch (record["type"]) {
      case "remove_range": {
        const { start, end, reason } = record;
        if (!isFiniteNonNegative(start)) fail(`${base}/start`, "Needs a finite start >= 0.");
        else if ((start as number) > ctx.mediaDuration)
          fail(`${base}/start`, "Start is beyond the media duration.");
        if (!isFiniteNonNegative(end)) fail(`${base}/end`, "Needs a finite end >= 0.");
        else if (isFiniteNonNegative(start) && (end as number) <= (start as number))
          fail(`${base}/end`, "End must be after start.");
        else if ((end as number) > ctx.mediaDuration)
          fail(`${base}/end`, "End is beyond the media duration.");
        if (!isNonEmptyString(reason)) fail(`${base}/reason`, "Needs a non-empty reason.");
        if (errors.length === 0 || !errors.some((e) => e.path.startsWith(base))) {
          operations.push({
            type: "remove_range",
            start: start as number,
            end: end as number,
            reason: (reason as string).trim(),
          });
        }
        break;
      }
      case "add_visual": {
        const { visualType, start, duration, concept } = record;
        if (!VISUAL_TYPES.includes(visualType as (typeof VISUAL_TYPES)[number]))
          fail(`${base}/visualType`, `Must be one of: ${VISUAL_TYPES.join(", ")}.`);
        if (!isFiniteNonNegative(start)) fail(`${base}/start`, "Needs a finite start >= 0.");
        if (typeof duration !== "number" || !Number.isFinite(duration) || duration <= 0)
          fail(`${base}/duration`, "Needs a finite duration > 0.");
        if (
          isFiniteNonNegative(start) &&
          typeof duration === "number" &&
          (start as number) + duration > ctx.mediaDuration
        )
          fail(`${base}/duration`, "Visual extends beyond the media duration.");
        if (!isNonEmptyString(concept)) fail(`${base}/concept`, "Needs a non-empty concept.");
        if (!errors.some((e) => e.path.startsWith(base))) {
          operations.push({
            type: "add_visual",
            visualType: visualType as AddVisualOp["visualType"],
            start: start as number,
            duration: duration as number,
            concept: (concept as string).trim(),
          });
        }
        break;
      }
      case "add_caption": {
        const { start, end, text } = record;
        if (!isFiniteNonNegative(start)) fail(`${base}/start`, "Needs a finite start >= 0.");
        if (!isFiniteNonNegative(end)) fail(`${base}/end`, "Needs a finite end >= 0.");
        else if (isFiniteNonNegative(start) && (end as number) <= (start as number))
          fail(`${base}/end`, "End must be after start.");
        else if ((end as number) > ctx.mediaDuration)
          fail(`${base}/end`, "End is beyond the media duration.");
        if (!isNonEmptyString(text)) fail(`${base}/text`, "Needs non-empty caption text.");
        if (!errors.some((e) => e.path.startsWith(base))) {
          operations.push({
            type: "add_caption",
            start: start as number,
            end: end as number,
            text: (text as string).trim(),
          });
        }
        break;
      }
      case "split_clip": {
        const { clipId, at } = record;
        if (!isNonEmptyString(clipId)) fail(`${base}/clipId`, "Needs a non-empty clip id.");
        else if (ctx.knownClipIds && !ctx.knownClipIds.includes(clipId as string))
          fail(`${base}/clipId`, "References an unknown clip id.");
        if (!isFiniteNonNegative(at)) fail(`${base}/at`, "Needs a finite split position >= 0.");
        else if ((at as number) > ctx.mediaDuration)
          fail(`${base}/at`, "Split position is beyond the media duration.");
        if (!errors.some((e) => e.path.startsWith(base))) {
          operations.push({ type: "split_clip", clipId: (clipId as string).trim(), at: at as number });
        }
        break;
      }
      default:
        fail(`${base}/type`, `Unknown operation type: ${String(record["type"])}.`);
    }
  });

  if (errors.length > 0) return { ok: false, plan: null, errors };
  return {
    ok: true,
    plan: { version: EDIT_PLAN_VERSION, goal: (obj["goal"] as string).trim(), operations },
    errors: [],
  };
}

// ---------------------------------------------------------------------------
// Visual Scene Spec (AGENTS §29)
// ---------------------------------------------------------------------------

export type SceneElement =
  | { type: "node"; id: string; x: number; y: number; label: string }
  | { type: "text"; id: string; x: number; y: number; text: string; size?: number };

export interface SceneConnection {
  from: string;
  to: string;
  animation?: "flow" | "none";
}

export interface VisualSceneSpec {
  scene: { width: number; height: number; duration: number };
  elements: SceneElement[];
  connections: SceneConnection[];
}

export interface SceneValidationResult {
  ok: boolean;
  spec: VisualSceneSpec | null;
  errors: ValidationIssue[];
}

/** Validate an unknown value as a Visual Scene Spec (editable diagram source). */
export function validateSceneSpec(input: unknown): SceneValidationResult {
  const errors: ValidationIssue[] = [];
  let root: unknown = input;
  if (typeof root === "string") {
    try {
      root = JSON.parse(root) as unknown;
    } catch {
      return { ok: false, spec: null, errors: [{ path: "", message: "Not valid JSON." }] };
    }
  }
  if (typeof root !== "object" || root === null || Array.isArray(root)) {
    return { ok: false, spec: null, errors: [{ path: "", message: "Scene must be a JSON object." }] };
  }
  const obj = root as Record<string, unknown>;
  const scene = obj["scene"] as Record<string, unknown> | undefined;
  if (!scene || typeof scene.width !== "number" || scene.width <= 0 || typeof scene.height !== "number" || scene.height <= 0) {
    errors.push({ path: "/scene", message: "Scene needs positive width and height." });
  }
  if (!scene || typeof scene.duration !== "number" || !Number.isFinite(scene.duration) || scene.duration <= 0) {
    errors.push({ path: "/scene/duration", message: "Scene needs a finite duration > 0." });
  }
  if (!Array.isArray(obj["elements"])) {
    errors.push({ path: "/elements", message: "Scene needs an elements array." });
  }
  if (errors.length > 0) return { ok: false, spec: null, errors };

  const ids = new Set<string>();
  const elements: SceneElement[] = [];
  (obj["elements"] as unknown[]).forEach((el, index) => {
    const base = `/elements/${index}`;
    if (typeof el !== "object" || el === null) {
      errors.push({ path: base, message: "Element must be an object." });
      return;
    }
    const record = el as Record<string, unknown>;
    if (!isNonEmptyString(record["id"])) {
      errors.push({ path: `${base}/id`, message: "Element needs a non-empty id." });
      return;
    }
    const id = (record["id"] as string).trim();
    if (ids.has(id)) {
      errors.push({ path: `${base}/id`, message: `Duplicate element id: ${id}.` });
      return;
    }
    ids.add(id);
    if (typeof record["x"] !== "number" || !Number.isFinite(record["x"]) || typeof record["y"] !== "number" || !Number.isFinite(record["y"])) {
      errors.push({ path: base, message: "Element needs finite x and y." });
      return;
    }
    if (record["type"] === "node" && isNonEmptyString(record["label"])) {
      elements.push({ type: "node", id, x: record["x"] as number, y: record["y"] as number, label: (record["label"] as string).trim() });
    } else if (record["type"] === "text" && isNonEmptyString(record["text"])) {
      const size = record["size"];
      elements.push({
        type: "text",
        id,
        x: record["x"] as number,
        y: record["y"] as number,
        text: (record["text"] as string).trim(),
        ...(typeof size === "number" && size > 0 ? { size } : {}),
      });
    } else {
      errors.push({ path: `${base}/type`, message: "Element must be a node with a label or text with content." });
    }
  });

  const connections: SceneConnection[] = [];
  const rawConnections = obj["connections"];
  if (rawConnections !== undefined) {
    if (!Array.isArray(rawConnections)) {
      errors.push({ path: "/connections", message: "Connections must be an array." });
    } else {
      rawConnections.forEach((conn, index) => {
        const base = `/connections/${index}`;
        const record = (typeof conn === "object" && conn !== null ? conn : {}) as Record<string, unknown>;
        if (!isNonEmptyString(record["from"]) || !ids.has((record["from"] as string).trim())) {
          errors.push({ path: `${base}/from`, message: "Connection references an unknown element." });
          return;
        }
        if (!isNonEmptyString(record["to"]) || !ids.has((record["to"] as string).trim())) {
          errors.push({ path: `${base}/to`, message: "Connection references an unknown element." });
          return;
        }
        const animation = record["animation"];
        connections.push({
          from: (record["from"] as string).trim(),
          to: (record["to"] as string).trim(),
          ...(animation === "flow" || animation === "none" ? { animation } : {}),
        });
      });
    }
  }

  if (errors.length > 0) return { ok: false, spec: null, errors };
  return {
    ok: true,
    spec: {
      scene: {
        width: (scene as Record<string, number>)["width"],
        height: (scene as Record<string, number>)["height"],
        duration: (scene as Record<string, number>)["duration"],
      },
      elements,
      connections,
    },
    errors: [],
  };
}

export * from "./mermaid";
export * from "./scene-svg";
