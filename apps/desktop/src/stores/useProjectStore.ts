import { create } from "zustand";
import type { ProjectConfig } from "@avid/shared-types";

/**
 * Project store (Phase 1).
 *
 * Interim persistence: localStorage. The versioned `project.json` on disk
 * (ADR-003, `avid-project`) replaces this once the Tauri backend lands —
 * the ProjectConfig shape already matches the manifest's project section,
 * so the migration is data-compatible by construction.
 */
const STORAGE_KEY = "avid.projects.v1";

function load(): ProjectConfig[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    const parsed: unknown = JSON.parse(raw);
    return Array.isArray(parsed) ? (parsed as ProjectConfig[]) : [];
  } catch {
    return [];
  }
}

function save(projects: ProjectConfig[]): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(projects));
  } catch {
    // Storage full or unavailable (private mode) — projects still work in-memory.
  }
}

export interface ProjectStore {
  projects: ProjectConfig[];
  /** Currently open project id, or null on Home. */
  openProjectId: string | null;
  addProject: (project: ProjectConfig) => void;
  openProject: (id: string) => void;
  closeProject: () => void;
  touchProject: (id: string, now: string) => void;
}

export const useProjectStore = create<ProjectStore>()((set) => ({
  projects: load(),
  openProjectId: null,
  addProject: (project) =>
    set((state) => {
      const projects = [project, ...state.projects];
      save(projects);
      return { projects, openProjectId: project.id };
    }),
  openProject: (id) => set({ openProjectId: id }),
  closeProject: () => set({ openProjectId: null }),
  touchProject: (id, now) =>
    set((state) => {
      const projects = state.projects.map((p) => (p.id === id ? { ...p, updatedAt: now } : p));
      save(projects);
      return { projects };
    }),
}));

export function selectOpenProject(state: ProjectStore): ProjectConfig | null {
  return state.projects.find((p) => p.id === state.openProjectId) ?? null;
}
