import { useState } from "react";
import type { FormEvent } from "react";
import { validateEditPlan, type EditOperation } from "@avid/ai-protocol";
import type { ApplyReport } from "@avid/shared-types";
import { Button, Panel } from "@avid/ui";
import { invokeCommand, IpcError } from "../lib/ipc";
import { notifyTimelineChanged } from "../stores/useJobsStore";

/** Human line per operation for review checkboxes and reports. Pure — tested. */
export function describeEditOperation(op: EditOperation): string {
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
}

/** Operation types the backend executes today (visuals land in Phase 8). */
const SENDABLE_TYPES = new Set(["remove_range", "add_caption", "split_clip"]);

type ReviewOp = { op: EditOperation; accepted: boolean };

/**
 * AI panel (Phase 7 slice): edit-plan dry-run validator + diff/apply.
 *
 * Validate → review each operation (accept/reject) → Apply sends only the
 * accepted ops to `apply_operations`, which executes them as ONE undoable,
 * transactional group. Rejected ops never leave the panel. `add_visual`
 * plans validate for review but the backend reports them unsupported until
 * Phase 8 visuals land — never silently dropped.
 */
export function AiPanel() {
  const [text, setText] = useState(
    '{\n  "version": 1,\n  "goal": "trim dead air",\n  "operations": [\n    { "type": "remove_range", "start": 10, "end": 15, "reason": "silence" }\n  ]\n}',
  );
  const [duration, setDuration] = useState("600");
  const [review, setReview] = useState<ReviewOp[] | null>(null);
  const [goal, setGoal] = useState("");
  const [errors, setErrors] = useState<string[]>([]);
  const [report, setReport] = useState<ApplyReport | null>(null);
  const [applyError, setApplyError] = useState<string | null>(null);
  const [applying, setApplying] = useState(false);

  function onValidate(event: FormEvent): void {
    event.preventDefault();
    setReport(null);
    setApplyError(null);
    const mediaDuration = Number(duration);
    const result = validateEditPlan(text, {
      mediaDuration: Number.isFinite(mediaDuration) ? mediaDuration : 0,
    });
    if (result.ok && result.plan) {
      setGoal(result.plan.goal);
      setReview(result.plan.operations.map((op) => ({ op, accepted: true })));
      setErrors([]);
    } else {
      setReview(null);
      setErrors(result.errors.map((e) => `${e.path || "(root)"}: ${e.message}`));
    }
  }

  async function onApply(): Promise<void> {
    if (!review) return;
    setApplying(true);
    setApplyError(null);
    setReport(null);
    try {
      const accepted = review.filter((item) => item.accepted).map((item) => item.op);
      const sendable = accepted.filter((op) => SENDABLE_TYPES.has(op.type));
      const deferred = accepted.filter((op) => !SENDABLE_TYPES.has(op.type));
      if (sendable.length === 0) {
        setApplyError(
          deferred.length > 0
            ? "Only visual operations selected — deterministic visuals land in Phase 8. Nothing was sent."
            : "Nothing accepted — tick at least one operation.",
        );
        return;
      }
      const result = await invokeCommand<ApplyReport>("apply_operations", {
        goal,
        operations: sendable,
      });
      setReport({
        label: result.label,
        results: [
          ...result.results,
          ...deferred.map((op, i) => ({
            index: sendable.length + i,
            applied: false,
            message: `${describeEditOperation(op)} — deferred to Phase 8 visuals.`,
          })),
        ],
      });
      notifyTimelineChanged();
    } catch (e) {
      setApplyError(e instanceof IpcError ? e.message : "Apply failed unexpectedly.");
    } finally {
      setApplying(false);
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
            rows={8}
            spellCheck={false}
            className="rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 font-mono text-xs text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
          />
        </label>
        <Button type="submit" variant="primary">
          Validate plan
        </Button>
      </form>

      {errors.length > 0 && (
        <div className="mt-3">
          <p className="text-sm text-avid-danger">Plan rejected — nothing would touch the timeline:</p>
          <ul className="mt-2 flex flex-col gap-1">
            {errors.map((line) => (
              <li key={line} role="alert" className="rounded-avid-sm bg-avid-raised px-2 py-1 font-mono text-xs text-avid-danger">
                {line}
              </li>
            ))}
          </ul>
        </div>
      )}

      {review && (
        <div className="mt-3">
          <p className="text-sm text-avid-success">
            Valid plan “{goal}” — review each operation, then apply as one undoable step:
          </p>
          <ul className="mt-2 flex flex-col gap-1">
            {review.map((item, index) => (
              <li key={`${describeEditOperation(item.op)}-${index}`}>
                <label className="flex cursor-pointer items-start gap-2 rounded-avid-sm bg-avid-raised px-2 py-1">
                  <input
                    type="checkbox"
                    checked={item.accepted}
                    onChange={() =>
                      setReview(review.map((r, i) => (i === index ? { ...r, accepted: !r.accepted } : r)))
                    }
                    aria-label={`Accept: ${describeEditOperation(item.op)}`}
                    className="mt-1 accent-[#4f8cff]"
                  />
                  <span className="font-mono text-xs text-avid-primary">{describeEditOperation(item.op)}</span>
                </label>
              </li>
            ))}
          </ul>
          <div className="mt-2">
            <Button
              variant="primary"
              onClick={onApply}
              disabled={applying || !review.some((item) => item.accepted)}
            >
              {applying ? "Applying…" : `Apply accepted (${review.filter((i) => i.accepted).length})`}
            </Button>
          </div>
        </div>
      )}

      {applyError && (
        <p role="alert" className="mt-3 text-sm text-avid-danger">
          {applyError}
        </p>
      )}
      {report && (
        <div className="mt-3">
          <p className="text-sm text-avid-secondary">{report.label} — per-operation outcome:</p>
          <ul className="mt-2 flex flex-col gap-1">
            {report.results.map((result) => (
              <li
                key={result.index}
                className={`rounded-avid-sm px-2 py-1 font-mono text-xs ${result.applied ? "bg-avid-raised text-avid-success" : "bg-avid-raised text-avid-muted"}`}
              >
                [{result.applied ? "applied" : "skipped"}] {result.message}
              </li>
            ))}
          </ul>
        </div>
      )}
    </Panel>
  );
}
