import { Link } from "react-router-dom";
import { useEffect } from "react";
import { Button, EmptyState, Icon, Panel } from "@avid/ui";
import { NavRail } from "../components/NavRail";
import { loadProviderSummary } from "../lib/provider";
import { BUILD_ID } from "../buildInfo";
import { runningCount, useJobsStore } from "../stores/useJobsStore";
import { useProjectStore } from "../stores/useProjectStore";

/**
 * Home dashboard: slim nav rail + project cards + honest status panels.
 * No metrics, no decoration — resume work, create, or configure.
 */
export function Home() {
  const projects = useProjectStore((s) => s.projects);
  const openProject = useProjectStore((s) => s.openProject);
  const jobs = useJobsStore((s) => s.jobs);
  const startPolling = useJobsStore((s) => s.startPolling);
  const provider = loadProviderSummary();
  const running = runningCount(jobs);

  useEffect(() => {
    startPolling();
  }, [startPolling]);

  return (
    <div className="flex h-full bg-avid-base text-avid-primary">
      <NavRail />

      <main className="min-w-0 flex-1 overflow-y-auto">
        <div className="mx-auto flex max-w-5xl flex-col gap-5 p-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-lg font-semibold">Projects</h1>
              <p className="text-sm text-avid-secondary">
                {projects.length === 0
                  ? "Create a project to start editing."
                  : `${projects.length} project${projects.length === 1 ? "" : "s"}, most recent first.`}
              </p>
            </div>
            <Link to="/projects/new">
              <Button variant="primary">New project</Button>
            </Link>
          </div>

          {projects.length === 0 ? (
            <EmptyState
              title="No projects yet"
              body="Create a project to start editing. Import footage, transcribe it, cut it, and export — everything stays on this machine."
              actions={
                <Link to="/projects/new">
                  <Button variant="primary">New project</Button>
                </Link>
              }
            />
          ) : (
            <ul className="grid grid-cols-1 gap-3 md:grid-cols-2 xl:grid-cols-3">
              {projects.map((p) => (
                <li key={p.id}>
                  <Link
                    to={`/editor/${p.id}`}
                    onClick={() => openProject(p.id)}
                    className="block rounded-avid-lg border border-avid-border bg-avid-panel p-4 transition-colors hover:border-avid-border-strong"
                  >
                    <span className="block truncate text-sm font-medium text-avid-primary" title={p.name}>
                      {p.name}
                    </span>
                    <span className="mt-1 block font-mono text-xs text-avid-muted">
                      {p.canvas} · {p.frameRate}fps · {p.resolution}
                    </span>
                    <span className="mt-2 flex items-center justify-between text-xs text-avid-muted">
                      <span>Edited {new Date(p.updatedAt).toLocaleString()}</span>
                      <span className="inline-flex items-center gap-1 text-avid-accent">
                        Open <Icon name="chevronRight" size={13} />
                      </span>
                    </span>
                  </Link>
                </li>
              ))}
            </ul>
          )}

          <p className="font-mono text-[10px] text-avid-muted">build {BUILD_ID}</p>
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
            <Panel title="Templates">
              <p className="text-sm text-avid-secondary">
                12 templates validate (technical explainer carries full content). The template
                browser with preview and apply lands with Phase 9.
              </p>
            </Panel>
            <Panel title="Examples">
              <p className="text-sm text-avid-secondary">
                1 shipped example project parses as manifest and timeline. The in-app gallery
                arrives with the template browser.
              </p>
            </Panel>
            <Panel
              title="AI status"
              actions={
                <Link to="/settings" className="text-xs text-avid-accent hover:underline">
                  Settings
                </Link>
              }
            >
              {provider ? (
                <p className="text-sm text-avid-secondary">
                  Last tested: <span className="font-mono">{provider.model}</span> @{" "}
                  <span className="font-mono">{provider.baseUrl}</span>
                </p>
              ) : (
                <p className="text-sm text-avid-secondary">
                  No provider tested yet — Ollama at{" "}
                  <span className="font-mono">localhost:11434</span> works out of the box. Basic
                  editing works offline regardless.
                </p>
              )}
            </Panel>
            <Panel title="Background jobs">
              {running > 0 ? (
                <p className="text-sm text-avid-secondary">
                  {running} task{running === 1 ? "" : "s"} running — open a project and use the
                  Jobs tab for progress and cancel.
                </p>
              ) : (
                <p className="text-sm text-avid-secondary">
                  No jobs running. Exports and transcriptions report here with live progress.
                </p>
              )}
            </Panel>
          </div>
        </div>
      </main>
    </div>
  );
}
