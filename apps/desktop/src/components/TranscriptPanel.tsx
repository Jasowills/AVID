import { useState } from "react";
import type { FormEvent } from "react";
import { Channel } from "@tauri-apps/api/core";
import { Button, EmptyState, Panel, TextField } from "@avid/ui";
import type { JobEvent, ModelStatus, Transcript } from "@avid/shared-types";
import { invokeCommand, IpcError } from "../lib/ipc";
import { notifyTimelineChanged } from "../stores/useJobsStore";
import { toastSuccess } from "../stores/useToastStore";
import { usePlaybackStore } from "../stores/usePlaybackStore";

/**
 * Transcript panel (Phase 5 UI slice): transcribe an imported asset through
 * the `transcribe_media` command and render timed segments. Clicking a
 * segment shows its range (timeline seek lands with playback in Phase 2+).
 */
export function TranscriptPanel({
  assetId,
  onAssetId: setAssetId,
}: {
  assetId: string;
  onAssetId: (id: string) => void;
}) {
  const [transcript, setTranscript] = useState<Transcript | null>(null);
  const [active, setActive] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [errorCode, setErrorCode] = useState<string | null>(null);
  const requestSeek = usePlaybackStore((state) => state.requestSeek);
  const [busy, setBusy] = useState(false);
  const [modelBusy, setModelBusy] = useState(false);
  const [modelNote, setModelNote] = useState<string | null>(null);
  const [deleting, setDeleting] = useState(false);
  const [deleteNote, setDeleteNote] = useState<string | null>(null);

  async function onTranscribe(event: FormEvent): Promise<void> {
    event.preventDefault();
    setBusy(true);
    setError(null);
    setErrorCode(null);
    setModelNote(null);
    setTranscript(null);
    setActive(null);
    try {
      // Registered as a job (visible + cancellable in the Jobs tab);
      // progress here is indeterminate, the spinner covers it honestly.
      const channel = new Channel<JobEvent>(() => undefined);
      const result = await invokeCommand<Transcript>("transcribe_media", {
        channel,
        assetId: assetId.trim(),
        language: "en",
      });
      setTranscript(result);
      toastSuccess(`Transcript ready — ${result.segments.length} segment${result.segments.length === 1 ? "" : "s"}.`);
    } catch (e) {
      setError(e instanceof IpcError ? e.message : "Transcription failed unexpectedly.");
      setErrorCode(e instanceof IpcError ? e.code : null);
    } finally {
      setBusy(false);
    }
  }

  async function onDownloadModel(): Promise<void> {
    setModelBusy(true);
    setModelNote(null);
    try {
      const status = await invokeCommand<ModelStatus>("ensure_speech_model");
      setModelNote(
        status.downloaded
          ? `Speech model ready (${status.path}). Transcribe again.`
          : "Model download reported incomplete — retry.",
      );
    } catch (e) {
      setModelNote(e instanceof IpcError ? e.message : "Model download failed unexpectedly.");
    } finally {
      setModelBusy(false);
    }
  }

  async function onRemoveSegment(index: number): Promise<void> {
    if (!transcript || !transcript.segments[index]) return;
    const segment = transcript.segments[index];
    setDeleting(true);
    setDeleteNote(null);
    try {
      await invokeCommand("apply_operations", {
        goal: "transcript delete",
        operations: [
          {
            type: "remove_range",
            start: segment.start,
            end: segment.end,
            reason: `transcript delete: ${segment.text.slice(0, 60)}`,
          },
        ],
      });
      setDeleteNote(`Removed ${segment.start.toFixed(1)}s → ${segment.end.toFixed(1)}s (undo in the timeline).`);
      notifyTimelineChanged();
    } catch (e) {
      setDeleteNote(e instanceof IpcError ? e.message : "Delete failed unexpectedly.");
    } finally {
      setDeleting(false);
    }
  }

  return (
    <Panel title="Transcript" className="flex-1">
      <form onSubmit={onTranscribe} className="flex flex-col gap-3">
        <TextField
          label="Asset id"
          placeholder="Paste from the Media panel after import"
          value={assetId}
          onChange={(e) => setAssetId(e.target.value)}
        />
        <Button type="submit" variant="primary" disabled={busy || assetId.trim() === ""} className="min-w-44">
          {busy ? `Transcribing ${assetId.trim().slice(0, 12)}…` : "Transcribe (local Whisper)"}
        </Button>
      </form>

      {error && (
        <div className="mt-3">
          <p role="alert" className="text-sm text-avid-danger">
            {error}
          </p>
          {errorCode === "AVID_TRANSCRIBE_001" && (
            <div className="mt-2 flex flex-col gap-2">
              <Button variant="secondary" disabled={modelBusy} onClick={onDownloadModel}>
                {modelBusy ? "Downloading model (~77 MB)…" : "Download speech model"}
              </Button>
              {modelNote && <p className="text-sm text-avid-secondary">{modelNote}</p>}
            </div>
          )}
        </div>
      )}

      {transcript && (
        <div className="mt-3 flex flex-col gap-1">
          <p className="text-xs text-avid-muted">
            {transcript.segments.length} segments · {transcript.language} · {transcript.provider}
          </p>
          {transcript.segments.map((segment, index) => (
            <button
              key={`${segment.start}-${index}`}
              onClick={() => {
                setActive(active === index ? null : index);
                requestSeek(segment.start);
              }}
              aria-pressed={active === index}
              className={`rounded-avid-sm px-2 py-1 text-left text-sm ${
                active === index ? "bg-avid-accent-muted text-avid-primary" : "text-avid-secondary hover:bg-avid-raised"
              }`}
            >
              <span className="mr-2 font-mono text-xs text-avid-muted">
                {segment.start.toFixed(1)}s
              </span>
              {segment.text || "(silence)"}
            </button>
          ))}
          {active !== null && transcript.segments[active] && (
            <div className="flex flex-col gap-2">
              <p className="text-xs text-avid-muted">
                Range {transcript.segments[active].start.toFixed(2)}s →{" "}
                {transcript.segments[active].end.toFixed(2)}s — timeline seek lands with playback.
              </p>
              <div>
                <Button
                  variant="danger"
                  disabled={deleting}
                  onClick={() => active !== null && onRemoveSegment(active)}
                >
                  {deleting ? "Removing…" : "Remove from timeline"}
                </Button>
              </div>
              {deleteNote && <p className="text-xs text-avid-secondary">{deleteNote}</p>}
            </div>
          )}
        </div>
      )}

      {busy && !transcript && (
        <div className="mt-3 flex flex-col gap-2" aria-label="Transcribing" aria-hidden="true">
          {[0, 1, 2].map((i) => (
            <div key={i} className="avid-skeleton h-8 w-full" style={{ width: `${92 - i * 9}%` }} />
          ))}
        </div>
      )}

      {!transcript && !error && !busy && (
        <div className="mt-3">
          <EmptyState
            title="No transcript yet"
            body="Import audio or video, then transcribe it on-device. Nothing leaves your machine."
          />
        </div>
      )}
    </Panel>
  );
}
