import { Link } from "react-router-dom";
import { Button, EmptyState, Panel } from "@avid/ui";
import { useProjectStore } from "../stores/useProjectStore";

/** Home screen (AGENTS §35): recents, create, templates, examples, AI status. */
export function Home() {
  const projects = useProjectStore((s) => s.projects);
  const openProject = useProjectStore((s) => s.openProject);

  return (
    <main className="mx-auto flex max-w-4xl flex-col gap-6 p-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-semibold">Welcome to AVID</h1>
          <p className="text-sm text-avid-secondary">
            Local-first AI video editing. Import → Understand → Edit → Visualize → Export.
          </p>
        </div>
        <Link to="/projects/new">
          <Button variant="primary">Create project</Button>
        </Link>
      </div>

      <Panel title="Recent projects">
        {projects.length === 0 ? (
          <EmptyState
            title="No projects yet"
            body="Create a project to start editing, or open an example to inspect a finished timeline."
            actions={
              <>
                <Link to="/projects/new">
                  <Button variant="primary">Create project</Button>
                </Link>
                <Button disabled title="Example projects land in Phase 9">
                  Try an example
                </Button>
              </>
            }
          />
        ) : (
          <ul className="flex flex-col gap-2">
            {projects.map((p) => (
              <li key={p.id}>
                <Link
                  to={`/editor/${p.id}`}
                  onClick={() => openProject(p.id)}
                  className="flex items-center justify-between rounded-avid-md border border-avid-border bg-avid-raised px-4 py-3 hover:border-avid-border-strong"
                >
                  <span>
                    <span className="block text-sm font-medium text-avid-primary">{p.name}</span>
                    <span className="block text-xs text-avid-muted">
                      {p.canvas} · {p.frameRate}fps · {p.resolution} · edited{" "}
                      {new Date(p.updatedAt).toLocaleString()}
                    </span>
                  </span>
                  <span className="text-sm text-avid-secondary">Open →</span>
                </Link>
              </li>
            ))}
          </ul>
        )}
      </Panel>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
        <Panel title="Templates">
          <p className="text-sm text-avid-secondary">
            Data-driven editing templates (technical explainer, talking head, podcast…) land in
            Phase 9.
          </p>
        </Panel>
        <Panel title="Local AI status">
          <p className="text-sm text-avid-secondary">
            No provider configured. Ollama + local Whisper wiring lands in Phase 5–6. Basic editing
            works offline regardless.
          </p>
        </Panel>
      </div>
    </main>
  );
}
