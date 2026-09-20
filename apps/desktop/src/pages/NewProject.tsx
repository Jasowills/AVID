import { useState } from "react";
import type { FormEvent } from "react";
import { Link, useNavigate } from "react-router-dom";
import { Button, Panel, TextField } from "@avid/ui";
import { FRAME_RATES, RESOLUTIONS, type NewProjectInput } from "@avid/shared-types";
import { CANVAS_PRESETS, createProjectConfig, validateNewProject } from "../lib/project";
import { useProjectStore } from "../stores/useProjectStore";

/** New Project dialog as a page (AGENTS §37). Template picker lands in Phase 9. */
export function NewProject() {
  const navigate = useNavigate();
  const addProject = useProjectStore((s) => s.addProject);
  const [name, setName] = useState("");
  const [canvas, setCanvas] = useState<NewProjectInput["canvas"]>("16:9");
  const [frameRate, setFrameRate] = useState<NewProjectInput["frameRate"]>(30);
  const [resolution, setResolution] = useState<NewProjectInput["resolution"]>("1080p");
  const [errors, setErrors] = useState<Record<string, string>>({});

  function onSubmit(event: FormEvent): void {
    event.preventDefault();
    const input: NewProjectInput = { name, canvas, frameRate, resolution, templateId: null };
    const validation = validateNewProject(input);
    if (validation.length > 0) {
      setErrors(Object.fromEntries(validation.map((e) => [e.field, e.message])));
      return;
    }
    const now = new Date().toISOString();
    const project = createProjectConfig(input, { id: crypto.randomUUID(), now });
    addProject(project);
    navigate(`/editor/${project.id}`);
  }

  const selectClass =
    "rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 text-sm text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent";

  return (
    <main className="mx-auto max-w-xl p-6">
      <Panel title="Create project">
        <form onSubmit={onSubmit} className="flex flex-col gap-4" noValidate>
          <TextField
            label="Project name"
            placeholder="Kafka explainer"
            value={name}
            onChange={(e) => setName(e.target.value)}
            error={errors.name}
            autoFocus
          />

          <fieldset>
            <legend className="mb-1 text-sm text-avid-secondary">Canvas</legend>
            <div className="flex flex-wrap gap-2" role="radiogroup" aria-label="Canvas aspect ratio">
              {CANVAS_PRESETS.map((preset) => (
                <button
                  key={preset.canvas}
                  type="button"
                  role="radio"
                  aria-checked={canvas === preset.canvas}
                  onClick={() => setCanvas(preset.canvas)}
                  className={`rounded-avid-md border px-3 py-2 text-sm ${
                    canvas === preset.canvas
                      ? "border-avid-accent bg-avid-accent-muted text-avid-primary"
                      : "border-avid-border bg-avid-raised text-avid-secondary hover:border-avid-border-strong"
                  }`}
                >
                  {preset.label}
                </button>
              ))}
            </div>
            {errors.canvas && (
              <p role="alert" className="mt-1 text-xs text-avid-danger">
                {errors.canvas}
              </p>
            )}
          </fieldset>

          <div className="grid grid-cols-2 gap-4">
            <label className="flex flex-col gap-1 text-sm text-avid-secondary">
              Frame rate
              <select
                value={frameRate}
                onChange={(e) => setFrameRate(Number(e.target.value) as NewProjectInput["frameRate"])}
                className={selectClass}
              >
                {FRAME_RATES.map((fps) => (
                  <option key={fps} value={fps}>
                    {fps} fps
                  </option>
                ))}
              </select>
            </label>
            <label className="flex flex-col gap-1 text-sm text-avid-secondary">
              Resolution
              <select
                value={resolution}
                onChange={(e) => setResolution(e.target.value as NewProjectInput["resolution"])}
                className={selectClass}
              >
                {RESOLUTIONS.map((res) => (
                  <option key={res} value={res}>
                    {res}
                  </option>
                ))}
              </select>
            </label>
          </div>

          <div className="flex items-center justify-end gap-2 pt-2">
            <Link to="/">
              <Button type="button" variant="ghost">
                Cancel
              </Button>
            </Link>
            <Button type="submit" variant="primary">
              Create project
            </Button>
          </div>
        </form>
      </Panel>
    </main>
  );
}
