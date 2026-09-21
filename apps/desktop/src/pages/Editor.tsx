import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { EmptyState } from "@avid/ui";
import { Button } from "@avid/ui";
import { AiPanel } from "../components/AiPanel";
import { ExportDialog } from "../components/ExportDialog";
import { InspectorPanel } from "../components/InspectorPanel";
import { JobsPanel } from "../components/JobsPanel";
import { MediaPanel } from "../components/MediaPanel";
import { PreviewPane } from "../components/PreviewPane";
import { isTauri } from "../lib/ipc";
import { TimelineDock } from "../components/TimelineDock";
import { TopBar } from "../components/TopBar";
import { TranscriptPanel } from "../components/TranscriptPanel";
import { VisualsPanel } from "../components/VisualsPanel";
import { useProjectStore } from "../stores/useProjectStore";

/**
 * Editor shell (AGENTS §38): top bar, left panel tabs, preview, inspector, timeline.
 * Media ingestion (Phase 2), timeline engine (Phase 3), and AI (Phase 6+)
 * plug into the placeholder regions below. Empty states guide, per §57.
 */
const ACTIVE_TABS = ["Media", "Transcript", "AI", "Jobs", "Visuals"] as const;
type ActiveTab = (typeof ACTIVE_TABS)[number];
const COMING_TABS = ["Scenes", "Templates", "Assets", "Audio", "Captions"] as const;

export function Editor() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const project = useProjectStore((s) => s.projects.find((p) => p.id === id));
  const openProject = useProjectStore((s) => s.openProject);
  const [tab, setTab] = useState<ActiveTab>("Media");
  const [exportOpen, setExportOpen] = useState(false);
  const [transcriptAssetId, setTranscriptAssetId] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(null);

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
      <TopBar
        saveStatus={isTauri() ? "Autosaved to project.json" : "Kept in this browser only"}
        onExport={() => setExportOpen(true)}
      />
      <ExportDialog open={exportOpen} onClose={() => setExportOpen(false)} />
      <div className="grid min-h-0 flex-1 grid-cols-[16rem_1fr_16rem] gap-px bg-avid-border-subtle">
        <aside className="flex min-h-0 flex-col gap-px overflow-y-auto bg-avid-base p-2" aria-label="Side panels">
          <nav className="flex flex-wrap gap-1" aria-label="Panel tabs">
            {ACTIVE_TABS.map((name) => (
              <button
                key={name}
                onClick={() => setTab(name)}
                aria-pressed={tab === name}
                className={`rounded-avid-sm px-2 py-1 text-xs ${
                  tab === name ? "bg-avid-raised text-avid-primary" : "text-avid-muted hover:text-avid-secondary"
                }`}
              >
                {name}
              </button>
            ))}
            {COMING_TABS.map((name) => (
              <button
                key={name}
                disabled
                title={`${name} panel (lands in its phase)`}
                className="rounded-avid-sm px-2 py-1 text-xs text-avid-muted disabled:cursor-not-allowed disabled:opacity-60"
              >
                {name}
              </button>
            ))}
          </nav>
          {tab === "Media" ? (
            <MediaPanel
              onTranscribeAsset={(id) => {
                setTranscriptAssetId(id);
                setTab("Transcript");
              }}
            />
          ) : tab === "Transcript" ? (
            <TranscriptPanel assetId={transcriptAssetId} onAssetId={setTranscriptAssetId} />
          ) : tab === "AI" ? (
            <AiPanel />
          ) : tab === "Jobs" ? (
            <JobsPanel />
          ) : (
            <VisualsPanel />
          )}
        </aside>

        <section className="flex min-h-0 flex-col bg-avid-base" aria-label="Preview">
          <div className="flex flex-1 items-center justify-center overflow-auto p-4">
            <PreviewPane clipId={selectedId} />
          </div>
        </section>

        <aside className="min-h-0 overflow-y-auto bg-avid-base p-2" aria-label="Inspector">
          <InspectorPanel clipId={selectedId} />
        </aside>
      </div>

      <footer
        className="h-56 shrink-0 border-t border-avid-border bg-avid-panel p-2"
        aria-label="Timeline"
      >
        <TimelineDock projectId={project.id} selectedId={selectedId} onSelect={setSelectedId} />
      </footer>
    </div>
  );
}
