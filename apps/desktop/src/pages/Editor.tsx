import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { EmptyState, Panel } from "@avid/ui";
import { Button } from "@avid/ui";
import { TopBar } from "../components/TopBar";
import { useProjectStore } from "../stores/useProjectStore";

/**
 * Editor shell (AGENTS §38): top bar, left panel tabs, preview, inspector, timeline.
 * Media ingestion (Phase 2), timeline engine (Phase 3), and AI (Phase 6+)
 * plug into the placeholder regions below. Empty states guide, per §57.
 */
const LEFT_TABS = ["Media", "Transcript", "Scenes", "AI", "Templates", "Assets", "Audio", "Captions"] as const;

export function Editor() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const project = useProjectStore((s) => s.projects.find((p) => p.id === id));
  const openProject = useProjectStore((s) => s.openProject);

  useEffect(() => {
    if (id) openProject(id);
  }, [id, openProject]);

  if (!project) {
    return (
      <main className="mx-auto max-w-xl p-6">
        <EmptyState
          title="Project not found"
          body="It may have been removed, or this link is stale."
          actions={
            <Button variant="primary" onClick={() => navigate("/")}>
              Back to Home
            </Button>
          }
        />
      </main>
    );
  }

  return (
    <div className="flex h-full flex-col">
      <TopBar />
      <div className="grid min-h-0 flex-1 grid-cols-[16rem_1fr_16rem] gap-px bg-avid-border-subtle">
        <aside className="flex min-h-0 flex-col gap-px overflow-y-auto bg-avid-base p-2" aria-label="Side panels">
          <nav className="flex flex-wrap gap-1" aria-label="Panel tabs">
            {LEFT_TABS.map((tab, i) => (
              <button
                key={tab}
                disabled={i > 0}
                title={i === 0 ? "Media library (lands in Phase 2)" : `${tab} panel (lands in its phase)`}
                className="rounded-avid-sm px-2 py-1 text-xs text-avid-muted disabled:cursor-not-allowed disabled:opacity-60 aria-pressed:text-avid-primary"
                aria-pressed={i === 0}
              >
                {tab}
              </button>
            ))}
          </nav>
          <Panel title="Media" className="flex-1">
            <EmptyState
              title="No media yet"
              body="Drop video here or import media. Media ingestion lands in Phase 2."
              actions={
                <Button disabled title="Import lands in Phase 2">
                  Import media
                </Button>
              }
            />
          </Panel>
        </aside>

        <section className="flex min-h-0 flex-col bg-avid-base" aria-label="Preview">
          <div className="flex flex-1 items-center justify-center p-4">
            <div
              className="flex aspect-video w-full max-w-3xl items-center justify-center rounded-avid-lg border border-avid-border bg-avid-panel"
              role="img"
              aria-label="Video preview (lands in Phase 2 with proxy playback)"
            >
              <p className="text-sm text-avid-muted">
                Preview — {project.canvas} · {project.frameRate}fps · {project.resolution}
              </p>
            </div>
          </div>
        </section>

        <aside className="min-h-0 overflow-y-auto bg-avid-base p-2" aria-label="Inspector">
          <Panel title="Inspector">
            <dl className="flex flex-col gap-1 text-sm">
              <div className="flex justify-between">
                <dt className="text-avid-muted">Project</dt>
                <dd className="text-avid-primary">{project.name}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-avid-muted">Canvas</dt>
                <dd className="text-avid-primary">{project.canvas}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-avid-muted">Timeline</dt>
                <dd className="text-avid-muted">Phase 3</dd>
              </div>
            </dl>
          </Panel>
        </aside>
      </div>

      <footer
        className="flex h-40 shrink-0 flex-col border-t border-avid-border bg-avid-panel p-2"
        aria-label="Timeline"
      >
        <div className="flex items-center justify-between px-2 pb-2">
          <span className="text-xs font-medium text-avid-secondary">Timeline</span>
          <span className="text-xs text-avid-muted">Tracks, trim, split, undo land in Phase 3</span>
        </div>
        <div className="flex flex-1 items-center justify-center rounded-avid-md border border-dashed border-avid-border-strong">
          <p className="text-xs text-avid-muted">Empty timeline — import media to begin (Phase 2+)</p>
        </div>
      </footer>
    </div>
  );
}
