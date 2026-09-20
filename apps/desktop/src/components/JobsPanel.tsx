import { useEffect } from "react";
import { Button, EmptyState, Panel } from "@avid/ui";
import { runningCount, useJobsStore } from "../stores/useJobsStore";

/**
 * Job center panel (AGENTS §50): every background job with progress,
 * cancel, and failure reasons. Polls `list_jobs` while visible.
 */
export function JobsPanel() {
  const jobs = useJobsStore((s) => s.jobs);
  const startPolling = useJobsStore((s) => s.startPolling);
  const stopPolling = useJobsStore((s) => s.stopPolling);
  const cancel = useJobsStore((s) => s.cancel);

  useEffect(() => {
    startPolling();
    return () => stopPolling();
  }, [startPolling, stopPolling]);

  if (jobs.length === 0) {
    return (
      <Panel title="Jobs" className="flex-1">
        <EmptyState
          title="No jobs yet"
          body="Exports and transcriptions run here with live progress and cancel."
        />
      </Panel>
    );
  }

  return (
    <Panel title={`Jobs (${runningCount(jobs)} running)`} className="flex-1">
      <ul className="flex flex-col gap-2">
        {jobs.map((job) => (
          <li key={job.id} className="rounded-avid-md border border-avid-border bg-avid-raised p-3">
            <div className="flex items-center justify-between gap-2">
              <span className="text-sm font-medium text-avid-primary">{job.label}</span>
              <span className="text-xs text-avid-muted">
                {job.kind} · {job.status}
                {job.progress !== null ? ` · ${Math.round(job.progress * 100)}%` : ""}
              </span>
            </div>
            {job.progress !== null && job.status === "running" && (
              <div
                role="progressbar"
                aria-valuenow={Math.round(job.progress * 100)}
                aria-valuemin={0}
                aria-valuemax={100}
                aria-label={`${job.label} progress`}
                className="mt-2 h-1.5 overflow-hidden rounded-full bg-avid-overlay"
              >
                <div className="h-full bg-avid-accent" style={{ width: `${job.progress * 100}%` }} />
              </div>
            )}
            {job.message && <p className="mt-1 text-xs text-avid-secondary">{job.message}</p>}
            {job.status === "running" && (
              <Button variant="ghost" onClick={() => cancel(job.id)} className="mt-2">
                Cancel
              </Button>
            )}
          </li>
        ))}
      </ul>
    </Panel>
  );
}
