import { create } from "zustand";
import type { JobRecord } from "@avid/shared-types";
import { invokeCommand, isTauri } from "../lib/ipc";

export const TIMELINE_CHANGED_EVENT = "avid:timeline-changed";

/** Broadcast that timeline state changed (import/apply/undo) so the dock refreshes. */
export function notifyTimelineChanged(): void {
  window.dispatchEvent(new CustomEvent(TIMELINE_CHANGED_EVENT));
}

interface JobsState {
  jobs: JobRecord[];
  polling: boolean;
  startPolling: () => void;
  stopPolling: () => void;
  refresh: () => Promise<void>;
  cancel: (jobId: string) => Promise<void>;
}

async function fetchJobs(): Promise<JobRecord[]> {
  if (!isTauri()) return [];
  try {
    return await invokeCommand<JobRecord[]>("list_jobs");
  } catch {
    return [];
  }
}

/**
 * Job-center store: polls `list_jobs` while mounted consumers exist.
 * Single interval no matter how many components subscribe.
 */
export const useJobsStore = create<JobsState>()((set, get) => {
  let timer: ReturnType<typeof setInterval> | null = null;
  return {
    jobs: [],
    polling: false,
    startPolling: () => {
      if (get().polling) return;
      set({ polling: true });
      void get().refresh();
      timer = setInterval(() => {
        void get().refresh();
      }, 1500);
    },
    stopPolling: () => {
      if (timer) clearInterval(timer);
      timer = null;
      set({ polling: false });
    },
    refresh: async () => {
      set({ jobs: await fetchJobs() });
    },
    cancel: async (jobId: string) => {
      try {
        await invokeCommand<boolean>("cancel_job", { jobId });
      } catch {
        // Cancellation is best-effort; the next poll shows the truth.
      }
      await get().refresh();
    },
  };
});

/** Number of currently running jobs (for the top-bar pill). */
export function runningCount(jobs: JobRecord[]): number {
  return jobs.filter((job) => job.status === "running").length;
}
