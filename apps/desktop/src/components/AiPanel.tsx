import { useState } from "react";
import type { FormEvent } from "react";
import { validateEditPlan } from "@avid/ai-protocol";
import { Button, Panel } from "@avid/ui";

/**
 * AI panel v1 (Phase 7 slice): edit-plan dry-run validator.
 *
 * Paste a plan JSON (or the output of any configured model) and validate it
 * against the exact gate that guards the timeline (ADR-008). Accepted plans
 * list their operations for review; rejected plans show per-operation errors.
 * Nothing here mutates state — application lands with the diff UI.
 */
export function AiPanel() {
  const [text, setText] = useState(
    '{\n  "version": 1,\n  "goal": "trim dead air",\n  "operations": [\n    { "type": "remove_range", "start": 10, "end": 15, "reason": "silence" }\n  ]\n}',
  );
  const [duration, setDuration] = useState("600");
  const [output, setOutput] = useState<
    | { kind: "idle" }
    | { kind: "ok"; summary: string; operations: string[] }
    | { kind: "errors"; errors: string[] }
  >({ kind: "idle" });

  function onValidate(event: FormEvent): void {
    event.preventDefault();
    const mediaDuration = Number(duration);
    const result = validateEditPlan(text, {
      mediaDuration: Number.isFinite(mediaDuration) ? mediaDuration : 0,
    });
    if (result.ok && result.plan) {
      setOutput({
        kind: "ok",
        summary: `Valid plan "${result.plan.goal}" — ${result.plan.operations.length} operation(s), safe to review.`,
        operations: result.plan.operations.map((op) => {
          switch (op.type) {
            case "remove_range":
              return `REMOVE ${op.start}s → ${op.end}s — ${op.reason}`;
            case "add_visual":
              return `VISUAL ${op.visualType} @ ${op.start}s for ${op.duration}s — ${op.concept}`;
            case "add_caption":
              return `CAPTION ${op.start}s → ${op.end}s — ${op.text}`;
            case "split_clip":
              return `SPLIT ${op.clipId} @ ${op.at}s`;
          }
        }),
      });
    } else {
      setOutput({
        kind: "errors",
        errors: result.errors.map((e) => `${e.path || "(root)"}: ${e.message}`),
      });
    }
  }

  return (
    <Panel title="AI — plan check" className="flex-1">
      <form onSubmit={onValidate} className="flex flex-col gap-3">
        <label className="flex flex-col gap-1 text-sm text-avid-secondary">
          Media duration (seconds)
          <input
            value={duration}
            onChange={(e) => setDuration(e.target.value)}
            inputMode="decimal"
            className="rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 text-sm text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
          />
        </label>
        <label className="flex flex-col gap-1 text-sm text-avid-secondary">
          Edit plan JSON
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            rows={10}
            spellCheck={false}
            className="rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 font-mono text-xs text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
          />
        </label>
        <Button type="submit" variant="primary">
          Validate plan
        </Button>
      </form>

      {output.kind === "ok" && (
        <div className="mt-3">
          <p className="text-sm text-avid-success">{output.summary}</p>
          <ul className="mt-2 flex flex-col gap-1">
            {output.operations.map((line) => (
              <li key={line} className="rounded-avid-sm bg-avid-raised px-2 py-1 font-mono text-xs text-avid-primary">
                {line}
              </li>
            ))}
          </ul>
        </div>
      )}
      {output.kind === "errors" && (
        <div className="mt-3">
          <p className="text-sm text-avid-danger">Plan rejected — nothing would touch the timeline:</p>
          <ul className="mt-2 flex flex-col gap-1">
            {output.errors.map((line) => (
              <li key={line} role="alert" className="rounded-avid-sm bg-avid-raised px-2 py-1 font-mono text-xs text-avid-danger">
                {line}
              </li>
            ))}
          </ul>
        </div>
      )}
    </Panel>
  );
}
