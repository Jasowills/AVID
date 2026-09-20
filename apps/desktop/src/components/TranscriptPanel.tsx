import { useState } from "react";
import type { FormEvent } from "react";
import { Channel } from "@tauri-apps/api/core";
import { Button, EmptyState, Panel, TextField } from "@avid/ui";
import type { JobEvent, ModelStatus, Transcript } from "@avid/shared-types";
import { invokeCommand, IpcError } from "../lib/ipc";

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
  const [busy, setBusy] = useState(false);
  const [modelBusy, setModelBusy] = useState(false);
  const [modelNote, setModelNote] = useState<string | null>(null);

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

  return (
    <Panel title="Transcript" className="flex-1">
      <form onSubmit={onTranscribe} className="flex flex-col gap-3">
        <TextField
          label="Asset id"
          placeholder="Paste from the Media panel after import"
          value={assetId}
          onChange={(e) => setAssetId(e.target.value)}
        />
        <Button type="submit" variant="primary" disabled={busy || assetId.trim() === ""}>
          {busy ? "Transcribing…" : "Transcribe (local Whisper)"}
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
              onClick={() => setActive(active === index ? null : index)}
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
            <p className="text-xs text-avid-muted">
              Range {transcript.segments[active].start.toFixed(2)}s →{" "}
              {transcript.segments[active].end.toFixed(2)}s — timeline seek lands with playback.
            </p>
          )}
        </div>
      )}

      {!transcript && !error && (
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
