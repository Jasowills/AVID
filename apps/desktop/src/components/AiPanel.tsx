import { useState } from "react";
import type { FormEvent } from "react";
import { Channel } from "@tauri-apps/api/core";
import { validateEditPlan, type EditOperation } from "@avid/ai-protocol";
import type { ApplyReport, CutConfidence, JobEvent, RoughCutProposal } from "@avid/shared-types";
import { Button, Panel } from "@avid/ui";
import { invokeCommand, IpcError } from "../lib/ipc";
import { notifyTimelineChanged } from "../stores/useJobsStore";
import { toastSuccess } from "../stores/useToastStore";

/** Format seconds as mm:ss.d timecode for review rows. Pure — tested. */
export function formatTimecode(totalSeconds: number): string {
  const clamped = Math.max(0, totalSeconds);
  const minutes = Math.floor(clamped / 60);
  const seconds = clamped - minutes * 60;
  return `${String(minutes).padStart(2, "0")}:${seconds.toFixed(1).padStart(4, "0")}`;
}

/** Human line per operation for review checkboxes and reports. Pure — tested. */
export function describeEditOperation(op: EditOperation): string {
  switch (op.type) {
    case "remove_range":
      return `REMOVE ${formatTimecode(op.start)} → ${formatTimecode(op.end)} — ${op.reason}`;
    case "add_visual":
      return `VISUAL ${op.visualType} @ ${formatTimecode(op.start)} for ${op.duration}s — ${op.concept}`;
    case "add_caption":
      return `CAPTION ${formatTimecode(op.start)} → ${formatTimecode(op.end)} — ${op.text}`;
    case "split_clip":
      return `SPLIT ${op.clipId} @ ${formatTimecode(op.at)}`;
  }
}

/** Sources the rough-cut proposer can actually inspect (mirrors backend `analyzed`). */
const ANALYZED_SOURCES = [
  { key: "transcript", label: "Transcript" },
  { key: "silence", label: "Timeline audio" },
  { key: "visual", label: "Visuals" },
  { key: "assets", label: "Assets" },
] as const;

/** Operation types the backend executes today (visuals land in Phase 8). */
const SENDABLE_TYPES = new Set(["remove_range", "add_caption", "split_clip"]);

type ReviewOp = { op: EditOperation; accepted: boolean; confidence?: CutConfidence };

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
  const [restoring, setRestoring] = useState(false);
  const [proposalAsset, setProposalAsset] = useState("");
  const [minSilence, setMinSilence] = useState("1.0");
  const [includeFillers, setIncludeFillers] = useState(true);
  const [proposing, setProposing] = useState(false);
  const [proposalNote, setProposalNote] = useState<string | null>(null);
  /** Backend `analyzed` split into source keys — null until a proposal (or pasted plan) exists. */
  const [analyzedBy, setAnalyzedBy] = useState<string[] | null>(null);

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
      // Pasted plan: AVID analyzed nothing — the review rows stand on their own.
      setAnalyzedBy([]);
      setErrors([]);
    } else {
      setReview(null);
      setErrors(result.errors.map((e) => `${e.path || "(root)"}: ${e.message}`));
    }
  }

  async function onPropose(event: FormEvent): Promise<void> {
    event.preventDefault();
    setProposing(true);
    setProposalNote(null);
    setReport(null);
    setApplyError(null);
    try {
      const channel = new Channel<JobEvent>(() => undefined);
      const proposal = await invokeCommand<RoughCutProposal>("propose_rough_cut", {
        channel,
        assetId: proposalAsset.trim(),
        minSilenceSeconds: Number(minSilence),
        includeFillers,
      });
      if (proposal.cuts.length === 0) {
        setProposalNote(`Nothing worth cutting found (${proposal.analyzed}). Footage stays untouched.`);
        return;
      }
      setGoal("rough cut");
      setReview(
        proposal.cuts.map((cut) => ({
          op: { type: "remove_range", start: cut.start, end: cut.end, reason: cut.reason } as EditOperation,
          accepted: true,
          confidence: cut.confidence,
        })),
      );
      setAnalyzedBy(proposal.analyzed.split("+").filter(Boolean));
      setErrors([]);
      setProposalNote(
        `${proposal.cuts.length} cuts proposed, ${proposal.removable_seconds.toFixed(1)}s removable (${proposal.analyzed}). Review below — nothing applied yet.`,
      );
    } catch (e) {
      setProposalNote(e instanceof IpcError ? e.message : "Proposal failed unexpectedly.");
    } finally {
      setProposing(false);
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
        snapshot: result.snapshot,
        results: [
          ...result.results,
          ...deferred.map((op, i) => ({
            index: sendable.length + i,
            applied: false,
            message: `${describeEditOperation(op)} — deferred to Phase 8 visuals.`,
          })),
        ],
      });
      const applied = result.results.filter((r) => r.applied).length;
      notifyTimelineChanged();
      toastSuccess(`${result.label} — ${applied} operation${applied === 1 ? "" : "s"} applied.`, {
        label: "Undo",
        run: () => {
          invokeCommand("timeline_undo").then(() => notifyTimelineChanged()).catch(() => undefined);
        },
      });
    } catch (e) {
      setApplyError(e instanceof IpcError ? e.message : "Apply failed unexpectedly.");
    } finally {
      setApplying(false);
    }
  }

  return (
    <Panel title="AI — plan check" className="flex-1">
      <form onSubmit={onPropose} className="flex flex-col gap-3 border-b border-avid-border-subtle pb-4">
        <p className="text-sm font-medium text-avid-primary">Rough-cut proposal</p>
        <label className="flex flex-col gap-1 text-sm text-avid-secondary">
          Asset id
          <input
            value={proposalAsset}
            onChange={(e) => setProposalAsset(e.target.value)}
            placeholder="From the Media tab"
            className="rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 font-mono text-xs text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
          />
        </label>
        <div className="flex flex-col gap-2">
          <label className="flex flex-col gap-1 text-sm text-avid-secondary">
            Min silence (s)
            <input
              value={minSilence}
              onChange={(e) => setMinSilence(e.target.value)}
              inputMode="decimal"
              className="rounded-avid-md border border-avid-border bg-avid-raised px-3 py-2 text-sm text-avid-primary focus-visible:outline-2 focus-visible:outline-avid-accent"
            />
          </label>
          <div className="flex items-center justify-between gap-2">
            <label className="flex items-center gap-2 text-sm text-avid-secondary">
              <input
                type="checkbox"
                checked={includeFillers}
                onChange={(e) => setIncludeFillers(e.target.checked)}
                className="accent-avid-accent"
              />
              Fillers
            </label>
            <Button type="submit" variant="secondary" disabled={proposing || proposalAsset.trim() === ""}>
              {proposing ? `Analyzing ${proposalAsset.trim().slice(0, 12)}…` : "Propose"}
            </Button>
          </div>
        </div>
        {proposalNote && <p className="text-xs text-avid-muted">{proposalNote}</p>}
        {analyzedBy && (
          <div className="rounded-avid-md border border-avid-border-subtle bg-avid-raised px-3 py-2">
            <p className="text-xs font-medium text-avid-secondary">Analyzed for this proposal</p>
            <ul className="mt-1 flex flex-col gap-0.5">
              {ANALYZED_SOURCES.map((source) => {
                const covered = analyzedBy.includes(source.key);
                return (
                  <li key={source.key} className="flex items-center gap-2 text-xs">
                    <span aria-hidden className={covered ? "text-avid-success" : "text-avid-faint"}>
                      {covered ? "✓" : "—"}
                    </span>
                    <span className="text-avid-primary">{source.label}</span>
                    <span className="text-avid-faint">{covered ? "checked" : "not covered"}</span>
                  </li>
                );
              })}
            </ul>
          </div>
        )}
      </form>

      <form onSubmit={onValidate} className="mt-4 flex flex-col gap-3">
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
                  <span className="mt-0.5 w-6 shrink-0 text-right font-mono text-xs text-avid-faint">{index + 1}.</span>
                  <input
                    type="checkbox"
                    checked={item.accepted}
                    onChange={() =>
                      setReview(review.map((r, i) => (i === index ? { ...r, accepted: !r.accepted } : r)))
                    }
                    aria-label={`Accept: ${describeEditOperation(item.op)}`}
                    className="mt-1 accent-avid-accent"
                  />
                  <span className="font-mono text-xs text-avid-primary">
                    {describeEditOperation(item.op)}
                    {item.confidence && (
                      <span className="ml-2 text-avid-faint">
                        · {item.confidence === "high" ? "High" : "Medium"} confidence
                      </span>
                    )}
                  </span>
                </label>
              </li>
            ))}
          </ul>
          <div className="mt-2">
            <Button
              variant="primary"
              onClick={onApply}
              disabled={applying || !review.some((item) => item.accepted)}
              className="min-w-40"
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
          {report.snapshot && (
            <div className="mt-2">
              <Button
                variant="ghost"
                disabled={restoring}
                onClick={async () => {
                  setRestoring(true);
                  try {
                    await invokeCommand("restore_snapshot", { name: report.snapshot });
                    notifyTimelineChanged();
                    setReport(null);
                    setReview(null);
                  } catch (e) {
                    setApplyError(e instanceof IpcError ? e.message : "Restore failed unexpectedly.");
                  } finally {
                    setRestoring(false);
                  }
                }}
              >
                {restoring ? "Restoring…" : "Restore pre-apply state"}
              </Button>
            </div>
          )}
        </div>
      )}
    </Panel>
  );
}
