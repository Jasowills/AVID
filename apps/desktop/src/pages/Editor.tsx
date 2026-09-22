/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/pages/editor.tsx) — the editor grid re-implemented in React
 * against AVID stores and the Rust command engine. Same skeleton: tab-width
 * left rail | canvas | 264px inspector over a resizable, minimizable
 * timeline row (Layers | canvas | tools), divider strips between cells,
 * canvas-only mode with a floating project header. Deviations: AVID keeps
 * its TopBar (undo/redo/save/AI/export per product spec) above the grid;
 * the bottom-right cell holds timeline tools instead of a soundboard. */

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Button, EmptyState, Icon, Tooltip } from "@avid/ui";
import { AiPanel } from "../components/AiPanel";
import { CommandPalette } from "../components/CommandPalette";
import { ExportDialog } from "../components/ExportDialog";
import { InspectorPanel } from "../components/InspectorPanel";
import { JobsPanel } from "../components/JobsPanel";
import { Layers } from "../components/Layers";
import { MediaPanel } from "../components/MediaPanel";
import { PreviewPane } from "../components/PreviewPane";
import { isTauri } from "../lib/ipc";
import { TimelineDock } from "../components/TimelineDock";
import { TopBar } from "../components/TopBar";
import { TranscriptPanel } from "../components/TranscriptPanel";
import { VisualsPanel } from "../components/VisualsPanel";
import { useProjectStore } from "../stores/useProjectStore";
import {
  RULER_HEIGHT,
  useLayoutStore,
} from "../stores/useLayoutStore";

const ACTIVE_TABS = ["Media", "Transcript", "AI", "Jobs", "Visuals"] as const;
type ActiveTab = (typeof ACTIVE_TABS)[number];
const COMING_TABS = ["Scenes", "Templates", "Assets", "Audio", "Captions"] as const;

const TAB_ICONS: Record<ActiveTab, "film" | "mic" | "layers" | "clock" | "grid"> = {
  Media: "film",
  Transcript: "mic",
  AI: "layers",
  Jobs: "clock",
  Visuals: "grid",
};

/** Left rail width follows the tab: composer tabs get room, browsers stay slim. */
function railWidth(tab: ActiveTab): number {
  return tab === "AI" ? 340 : 264;
}

export function Editor() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const project = useProjectStore((s) => s.projects.find((p) => p.id === id));
  const openProject = useProjectStore((s) => s.openProject);
  const uiVisible = useLayoutStore((s) => s.uiVisible);
  const timelineMinimized = useLayoutStore((s) => s.timelineMinimized);
  const timelineHeight = useLayoutStore((s) => s.timelineHeight);
  const toggleUI = useLayoutStore((s) => s.toggleUI);
  const toggleTimeline = useLayoutStore((s) => s.toggleTimeline);
  const setTimelineHeight = useLayoutStore((s) => s.setTimelineHeight);

  const [tab, setTab] = useState<ActiveTab>("Media");
  const [exportOpen, setExportOpen] = useState(false);
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [transcriptAssetId, setTranscriptAssetId] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [zoom, setZoom] = useState(1);
  const [resizing, setResizing] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const resizeStart = useRef({ y: 0, height: 0 });
  const layersScroll = useRef<HTMLDivElement | null>(null);

  const reduceMotion = useMemo(
    () =>
      typeof window !== "undefined" &&
      typeof window.matchMedia === "function" &&
      window.matchMedia("(prefers-reduced-motion: reduce)").matches,
    [],
  );

  useEffect(() => {
    if (id) openProject(id);
  }, [id, openProject]);

  // Global keys: ⌘/Ctrl+K palette, Alt+U interface, Alt+T timeline.
  // Skipped inside text fields (except palette toggle, which is safe).
  useEffect(() => {
    const onKey = (event: KeyboardEvent): void => {
      const mod = event.metaKey || event.ctrlKey;
      if (mod && event.key.toLowerCase() === "k") {
        event.preventDefault();
        setPaletteOpen((v) => !v);
        return;
      }
      const target = event.target as HTMLElement | null;
      const inField =
        target &&
        (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.tagName === "SELECT" || target.isContentEditable);
      if (inField || mod) return;
      if (event.altKey && event.key.toLowerCase() === "u") {
        event.preventDefault();
        toggleUI();
      } else if (event.altKey && event.key.toLowerCase() === "t") {
        event.preventDefault();
        toggleTimeline();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [toggleUI, toggleTimeline]);

  // Timeline resize drag (pointer capture on the divider handle).
  useEffect(() => {
    if (!resizing) return;
    const onMove = (event: PointerEvent): void => {
      setTimelineHeight(resizeStart.current.height + (resizeStart.current.y - event.clientY));
    };
    const onUp = (): void => setResizing(false);
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
    return () => {
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
    };
  }, [resizing, setTimelineHeight]);

  const beginResize = useCallback(
    (event: React.PointerEvent) => {
      event.preventDefault();
      resizeStart.current = { y: event.clientY, height: timelineHeight };
      setResizing(true);
    },
    [timelineHeight],
  );

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

  const leftW = railWidth(tab);
  const rowH = timelineMinimized ? RULER_HEIGHT : timelineHeight;

  return (
    <div className="flex h-full flex-col">
      <TopBar
        saveStatus={isTauri() ? "Autosaved to project.json" : "Kept in this browser only"}
        onExport={() => setExportOpen(true)}
      />
      <ExportDialog open={exportOpen} onClose={() => setExportOpen(false)} />
      <CommandPalette
        open={paletteOpen}
        onClose={() => setPaletteOpen(false)}
        onTab={setTab}
        onExport={() => setExportOpen(true)}
        onNewProject={() => navigate("/projects/new")}
        onSettings={() => navigate("/settings")}
        onHome={() => navigate("/")}
      />

      {uiVisible ? (
        <div
          className="grid min-h-0 flex-1"
          style={{
            gridTemplateColumns: `${leftW}px 1px 1fr 1px 264px`,
            gridTemplateRows: `1fr 1px ${rowH}px`,
            transition: reduceMotion ? undefined : "grid-template-columns 200ms ease-out",
          }}
        >
          <aside className="flex min-h-0 min-w-0 flex-col bg-avid-panel" aria-label="Side panels">
            <div className="flex h-10 shrink-0 items-center gap-1 border-b border-avid-border-subtle px-2">
              <div className="relative">
                <button
                  onClick={() => setMenuOpen((v) => !v)}
                  aria-expanded={menuOpen}
                  aria-label="Project menu"
                  title={project.name}
                  className="rounded-avid-sm p-1.5 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
                >
                  <Icon name="folder" size={15} />
                </button>
                {menuOpen && (
                  <>
                    <button
                      aria-label="Close project menu"
                      className="fixed inset-0 z-40 cursor-default"
                      onClick={() => setMenuOpen(false)}
                    />
                    <div
                      role="menu"
                      aria-label="Project menu"
                      className="absolute left-0 top-full z-50 mt-1 w-52 rounded-avid-md border border-avid-border bg-avid-overlay p-1 shadow-xl"
                    >
                      {[
                        { id: "home", label: "Back to Home", run: () => navigate("/") },
                        { id: "new", label: "New project", run: () => navigate("/projects/new") },
                        { id: "settings", label: "Open Settings", run: () => navigate("/settings") },
                        { id: "palette", label: "Command palette", hint: "⌘K", run: () => setPaletteOpen(true) },
                      ].map((item) => (
                        <button
                          key={item.id}
                          role="menuitem"
                          onClick={() => {
                            setMenuOpen(false);
                            item.run();
                          }}
                          className="flex w-full items-center gap-2 rounded-avid-sm px-2 py-1.5 text-left text-xs text-avid-primary hover:bg-avid-raised"
                        >
                          <span className="flex-1">{item.label}</span>
                          {item.hint && <span className="font-mono text-[10px] text-avid-muted">{item.hint}</span>}
                        </button>
                      ))}
                    </div>
                  </>
                )}
              </div>
              <span className="min-w-0 flex-1 truncate text-xs text-avid-muted" title={project.name}>
                {project.name}
              </span>
              <Tooltip label={timelineMinimized ? "Restore timeline" : "Minimize timeline"} shortcut="Alt+T">
                <button
                  onClick={toggleTimeline}
                  aria-label={timelineMinimized ? "Restore timeline" : "Minimize timeline"}
                  className="rounded-avid-sm p-1.5 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
                >
                  <Icon name="panelBottom" size={15} />
                </button>
              </Tooltip>
              <Tooltip label="Hide interface (focus canvas)" shortcut="Alt+U">
                <button
                  onClick={toggleUI}
                  aria-label="Hide interface"
                  className="rounded-avid-sm p-1.5 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
                >
                  <Icon name="panelLeft" size={15} />
                </button>
              </Tooltip>
            </div>
            <nav
              className="mx-2 mt-2 flex flex-wrap gap-0.5 rounded-avid-md bg-avid-raised p-0.5"
              aria-label="Panel tabs"
            >
              {ACTIVE_TABS.map((name) => (
                <button
                  key={name}
                  onClick={() => setTab(name)}
                  aria-pressed={tab === name}
                  title={name}
                  className={`flex items-center gap-1.5 rounded-avid-sm px-2 py-1 text-xs ${
                    tab === name
                      ? "bg-avid-overlay text-avid-primary shadow-sm"
                      : "text-avid-muted hover:text-avid-secondary"
                  }`}
                >
                  <Icon name={TAB_ICONS[name]} size={13} />
                  {name}
                </button>
              ))}
              {COMING_TABS.map((name) => (
                <button
                  key={name}
                  disabled
                  title={`${name} panel (lands in its phase)`}
                  className="hidden rounded-avid-sm px-2 py-1 text-xs text-avid-muted disabled:cursor-not-allowed disabled:opacity-60"
                >
                  {name}
                </button>
              ))}
            </nav>
            <div className="flex min-h-0 flex-1 flex-col overflow-y-auto p-2">
              {tab === "Media" ? (
                <MediaPanel
                  onTranscribeAsset={(assetId) => {
                    setTranscriptAssetId(assetId);
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
            </div>
          </aside>
          <div className="bg-avid-border-strong" aria-hidden />

          <section className="flex min-h-0 min-w-0 flex-col bg-avid-base" aria-label="Preview">
            <div className="flex flex-1 items-center justify-center overflow-auto p-4">
              <PreviewPane clipId={selectedId} />
            </div>
          </section>
          <div className="bg-avid-border-strong" aria-hidden />

          <aside className="min-h-0 min-w-0 overflow-y-auto bg-avid-panel p-2" aria-label="Inspector">
            <InspectorPanel clipId={selectedId} />
          </aside>

          <div className="relative col-span-full bg-avid-border-strong">
            {!timelineMinimized && (
              <div
                role="separator"
                aria-orientation="horizontal"
                aria-label="Resize timeline"
                onPointerDown={beginResize}
                className="group absolute inset-x-0 -top-px z-10 h-2 cursor-ns-resize"
              >
                <div
                  className={`absolute inset-x-0 top-px h-px transition-colors group-hover:bg-avid-accent ${
                    resizing ? "bg-avid-accent" : ""
                  }`}
                />
              </div>
            )}
          </div>

          <Layers scrollRef={layersScroll} collapsed={timelineMinimized} />
          <div className="bg-avid-border-strong" aria-hidden />

          <div
            className="min-h-0 min-w-0 bg-avid-panel"
            aria-label="Timeline"
            onScrollCapture={(e) => {
              if (layersScroll.current) layersScroll.current.scrollTop = e.currentTarget.scrollTop;
            }}
          >
            {timelineMinimized ? (
              <button
                onClick={toggleTimeline}
                className="flex h-full w-full items-center gap-2 px-3 text-xs text-avid-muted hover:text-avid-secondary"
              >
                <Icon name="chevronUp" size={13} />
                Timeline minimized — restore
              </button>
            ) : (
              <div className="h-full p-2">
                <TimelineDock
                  projectId={project.id}
                  selectedId={selectedId}
                  onSelect={setSelectedId}
                  zoom={zoom}
                />
              </div>
            )}
          </div>
          <div className="bg-avid-border-strong" aria-hidden />

          <div className="flex min-h-0 min-w-0 flex-col justify-center gap-1 bg-avid-panel px-2" aria-label="Timeline tools">
            <div className="flex items-center gap-0.5" role="group" aria-label="Zoom">
              {[0.5, 1, 2, 4].map((level) => (
                <button
                  key={level}
                  onClick={() => setZoom(level)}
                  aria-pressed={zoom === level}
                  title={`Zoom ${level}x`}
                  className={`rounded-avid-sm px-1.5 py-1 font-mono text-[11px] ${
                    zoom === level ? "bg-avid-raised text-avid-primary" : "text-avid-muted hover:text-avid-secondary"
                  }`}
                >
                  {level}x
                </button>
              ))}
            </div>
            <div className="flex items-center gap-1">
              <Tooltip label={timelineMinimized ? "Restore timeline" : "Minimize timeline"} shortcut="Alt+T">
                <button
                  onClick={toggleTimeline}
                  aria-label={timelineMinimized ? "Restore timeline" : "Minimize timeline"}
                  aria-pressed={timelineMinimized}
                  className="rounded-avid-sm p-1.5 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
                >
                  <Icon name={timelineMinimized ? "chevronUp" : "chevronDown"} size={14} />
                </button>
              </Tooltip>
              <Tooltip label="Hide interface (focus canvas)" shortcut="Alt+U">
                <button
                  onClick={toggleUI}
                  aria-label="Hide interface"
                  className="rounded-avid-sm p-1.5 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
                >
                  <Icon name="hideUI" size={14} />
                </button>
              </Tooltip>
            </div>
          </div>
        </div>
      ) : (
        <section className="relative flex min-h-0 flex-1 flex-col bg-avid-base" aria-label="Preview">
          <div className="flex items-center gap-2 self-start rounded-avid-md border border-avid-border bg-avid-panel px-2 py-1.5 shadow-lg" style={{ margin: "12px 0 0 12px" }}>
            <span className="max-w-48 truncate text-xs text-avid-secondary" title={project.name}>
              {project.name}
            </span>
            <Tooltip label="Show interface" shortcut="Alt+U">
              <button
                onClick={toggleUI}
                aria-label="Show interface"
                className="rounded-avid-sm p-1 text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
              >
                <Icon name="showUI" size={14} />
              </button>
            </Tooltip>
          </div>
          <div className="flex flex-1 items-center justify-center overflow-auto p-4">
            <PreviewPane clipId={selectedId} />
          </div>
        </section>
      )}
    </div>
  );
}
