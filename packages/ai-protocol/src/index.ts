/**
 * @avid/ai-protocol — scaffold only (Phase 0).
 * Edit-plan (AGENTS §25) and visual-scene (AGENTS §29) schemas +
 * validators land in Phase 7/8. Invalid AI output must never mutate
 * the timeline — validation lives here.
 */

export const SCAFFOLD = "@avid/ai-protocol scaffold" as const;
