import { useEffect, useState } from "react";
import type { FormEvent } from "react";
import {
  parseMermaidFlowchart,
  renderSceneSvg,
  validateSceneSpec,
  type VisualSceneSpec,
} from "@avid/ai-protocol";
import { Button, EmptyState, Panel, TextField } from "@avid/ui";
import { invokeCommand, IpcError, isTauri } from "../lib/ipc";
import { notifyTimelineChanged } from "../stores/useJobsStore";

interface SavedScene {
  id: string;
  spec: VisualSceneSpec;
}

const DEFAULT_MERMAID = [
  "flowchart TD",
  "  P[Producer] --> K[Kafka]",
  "  K --> C1[Partition 1]",
  "  K --> C2[Partition 2]",
  "",
].join("\n");

/**
 * Visuals panel (Phase 8 UI slice): Mermaid flowchart → validated scene →
 * SVG preview → label editing → save → place on the graphics track.
 * Export does not burn visuals in yet — the preview and the stored,
 * editable scene are the real surface (stated in the panel, not hidden).
 */
export function VisualsPanel() {
  const [mermaid, setMermaid] = useState(DEFAULT_MERMAID);
  const [spec, setSpec] = useState<VisualSceneSpec | null>(null);
  const [issues, setIssues] = useState<string[]>([]);
  const [sceneId, setSceneId] = useState("kafka-diagram");
  const [placeStart, setPlaceStart] = useState("0");
  const [saved, setSaved] = useState<SavedScene[]>([]);
  const [notice, setNotice] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const backend = isTauri();

  const refreshSaved = async (): Promise<void> => {
    if (!backend) return;
    try {
      setSaved(await invokeCommand<SavedScene[]>("list_visual_scenes"));
    } catch {
      setSaved([]);
    }
  };

  useEffect(() => {
    void refreshSaved();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function onConvert(event: FormEvent): void {
    event.preventDefault();
    setNotice(null);
    const parsed = parseMermaidFlowchart(mermaid);
    if (!parsed.ok || !parsed.spec) {
      setSpec(null);
      setIssues(parsed.errors.map((e) => `line ${e.line}: ${e.message}`));
      return;
    }
    const validated = validateSceneSpec(parsed.spec);
    if (!validated.ok || !validated.spec) {
      setSpec(null);
      setIssues(validated.errors.map((e) => `${e.path || "scene"}: ${e.message}`));
      return;
    }
    setIssues([]);
    setSpec(validated.spec);
  }

  function setNodeLabel(id: string, label: string): void {
    setSpec((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        elements: prev.elements.map((el) =>
          el.id === id && el.type === "node" ? { ...el, label } : el,
        ),
      };
    });
  }

  async function onSave(): Promise<void> {
    if (!spec || sceneId.trim() === "") return;
    setBusy(true);
    setNotice(null);
    try {
      await invokeCommand("save_visual_scene", { id: sceneId.trim(), spec });
      setNotice(`Saved scene “${sceneId.trim()}”.`);
      await refreshSaved();
    } catch (e) {
      setNotice(e instanceof IpcError ? e.message : "Save failed unexpectedly.");
    } finally {
      setBusy(false);
    }
  }

  async function onPlace(id: string): Promise<void> {
    setBusy(true);
    setNotice(null);
    try {
      await invokeCommand("place_visual_on_timeline", {
        sceneId: id,
        start: Number(placeStart) || 0,
      });
      setNotice(`Placed “${id}” at ${Number(placeStart) || 0}s (undoable).`);
      notifyTimelineChanged();
    } catch (e) {
      setNotice(e instanceof IpcError ? e.message : "Place failed unexpectedly.");
    } finally {
      setBusy(false);
    }
  }

  return (
    <Panel title="Visuals" className="flex-1">
      <form onSubmit={onConvert} className="flex flex-col gap-3">
        <label className="flex flex-col gap-1 text-sm text-avid-secondary">
          Mermaid flowchart (TD/TB/LR/RL/BT, ID[Label], A --&gt; B)
          <textarea
            value={mermaid}
            onChange={(e) => setMermaid(e.target.value)}
            rows={6}
            spellCheck={false}
            className="rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 font-mono text-xs text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
          />
        </label>
        <div>
          <Button type="submit" variant="secondary">
            Convert to scene
          </Button>
        </div>
      </form>

      {issues.length > 0 && (
        <ul className="mt-3 flex flex-col gap-1">
          {issues.map((line) => (
            <li key={line} role="alert" className="rounded-avid-sm bg-avid-raised px-2 py-1 font-mono text-xs text-avid-danger">
              {line}
            </li>
          ))}
        </ul>
      )}

      {spec && (
        <div className="mt-3 flex flex-col gap-3">
          <div
            role="img"
            aria-label="Diagram preview"
            className="overflow-auto rounded-avid-md border border-avid-border"
            dangerouslySetInnerHTML={{ __html: renderSceneSvg(spec) }}
          />
          <div className="flex flex-col gap-2">
            {spec.elements
              .filter((el) => el.type === "node")
              .map((el) => (
                <label key={el.id} className="flex items-center gap-2 text-xs text-avid-secondary">
                  <span className="w-16 shrink-0 font-mono">{el.id}</span>
                  <input
                    value={el.type === "node" ? el.label : ""}
                    onChange={(e) => setNodeLabel(el.id, e.target.value)}
                    aria-label={`Label for node ${el.id}`}
                    className="min-w-0 flex-1 rounded-avid-sm border border-avid-border bg-avid-raised px-2 py-1 text-sm text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
                  />
                </label>
              ))}
          </div>
          <div className="flex flex-wrap items-end gap-2">
            <div className="flex-1">
              <TextField
                label="Scene id"
                value={sceneId}
                onChange={(e) => setSceneId(e.target.value)}
              />
            </div>
            <Button variant="primary" disabled={busy || !backend} title={backend ? "Save scene" : "Saving needs the desktop backend"} onClick={onSave}>
              Save scene
            </Button>
          </div>
        </div>
      )}

      {saved.length > 0 && (
        <div className="mt-4 border-t border-avid-border-subtle pt-3">
          <h3 className="mb-2 text-xs font-medium text-avid-secondary">Saved scenes ({saved.length})</h3>
          <ul className="flex flex-col gap-3">
            {saved.map((scene) => (
              <li key={scene.id} className="rounded-avid-md border border-avid-border bg-avid-raised p-2">
                <div className="mb-2 flex items-center justify-between gap-2">
                  <span className="font-mono text-xs text-avid-primary">{scene.id}</span>
                  <span className="flex items-center gap-2">
                    <input
                      value={placeStart}
                      onChange={(e) => setPlaceStart(e.target.value)}
                      inputMode="decimal"
                      aria-label={`Start time for ${scene.id}`}
                      className="w-16 rounded-avid-sm border border-avid-border bg-avid-base px-2 py-1 font-mono text-xs text-avid-primary"
                    />
                    <Button variant="secondary" disabled={busy} onClick={() => onPlace(scene.id)}>
                      Place
                    </Button>
                  </span>
                </div>
                <div
                  role="img"
                  aria-label={`Preview of ${scene.id}`}
                  className="overflow-auto rounded-avid-sm"
                  dangerouslySetInnerHTML={{ __html: renderSceneSvg(scene.spec) }}
                />
              </li>
            ))}
          </ul>
        </div>
      )}

      {!spec && saved.length === 0 && (
        <div className="mt-3">
          <EmptyState
            title="No visuals yet"
            body="Convert a Mermaid flowchart above, edit labels, save the scene, and place it on the graphics track. Exports show scenes as data for now — burn-in follows with the text-capable sidecar."
          />
        </div>
      )}

      {notice && <p className="mt-3 text-sm text-avid-secondary">{notice}</p>}
    </Panel>
  );
}
