/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/context/layout.tsx) — layout state re-implemented with
 * zustand + localStorage against AVID's editor. No engine code carried over. */

import { create } from "zustand";

/** Default timeline dock height (px). Mirrors the reference default. */
export const DEFAULT_TIMELINE_HEIGHT = 234;
/** Minimum draggable timeline height (px). */
export const MIN_TIMELINE_HEIGHT = 120;
/** Maximum draggable timeline height (px) — keeps the preview usable. */
export const MAX_TIMELINE_HEIGHT = 560;
/** Ruler-only height when the timeline is minimized. */
export const RULER_HEIGHT = 28;

const STORAGE_KEY = "avid.layout.v1";

export interface LayoutState {
  uiVisible: boolean;
  timelineMinimized: boolean;
  timelineHeight: number;
}

export function clampTimelineHeight(height: number): number {
  if (!Number.isFinite(height)) return DEFAULT_TIMELINE_HEIGHT;
  return Math.min(MAX_TIMELINE_HEIGHT, Math.max(MIN_TIMELINE_HEIGHT, height));
}

/** Pure load (testable): stored values win, garbage falls back to defaults. */
export function loadLayoutState(): LayoutState {
  const fallback: LayoutState = {
    uiVisible: true,
    timelineMinimized: false,
    timelineHeight: DEFAULT_TIMELINE_HEIGHT,
  };
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return fallback;
    const parsed = JSON.parse(raw) as Partial<LayoutState>;
    return {
      uiVisible: typeof parsed.uiVisible === "boolean" ? parsed.uiVisible : true,
      timelineMinimized: typeof parsed.timelineMinimized === "boolean" ? parsed.timelineMinimized : false,
      timelineHeight:
        typeof parsed.timelineHeight === "number"
          ? clampTimelineHeight(parsed.timelineHeight)
          : DEFAULT_TIMELINE_HEIGHT,
    };
  } catch {
    return fallback;
  }
}

function save(state: LayoutState): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  } catch {
    // Private mode / full storage — layout still works in-memory.
  }
}

interface LayoutStore extends LayoutState {
  toggleUI: () => void;
  toggleTimeline: () => void;
  setTimelineHeight: (height: number) => void;
}

export const useLayoutStore = create<LayoutStore>()((set) => ({
  ...loadLayoutState(),
  toggleUI: () =>
    set((state) => {
      const next = { ...state, uiVisible: !state.uiVisible };
      save(next);
      return next;
    }),
  toggleTimeline: () =>
    set((state) => {
      const next = { ...state, timelineMinimized: !state.timelineMinimized };
      save(next);
      return next;
    }),
  setTimelineHeight: (height: number) =>
    set((state) => {
      const next = { ...state, timelineHeight: clampTimelineHeight(height) };
      save(next);
      return next;
    }),
}));
