import { useState } from "react";
import type { FormEvent } from "react";
import { Button, EmptyState, Panel, TextField } from "@avid/ui";
import type { Transcript } from "@avid/shared-types";
import { invokeCommand, IpcError } from "../lib/ipc";

/**
 * Transcript panel (Phase 5 UI slice): transcribe an imported asset through
 * the `transcribe_media` command and render timed segments. Clicking a
 * segment shows its range (timeline seek lands with playback in Phase 2+).
 */
export function TranscriptPanel() {
  const [assetId, setAssetId] = useState("");
  const [transcript, setTranscript] = useState<Transcript | null>(null);
  const [active, setActive] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function onTranscribe(event: FormEvent): Promise<void> {
    event.preventDefault();
    setBusy(true);
    setError(null);
    setTranscript(null);
    setActive(null);
    try {
      const result = await invokeCommand<Transcript>("transcribe_media", {
        assetId: assetId.trim(),
        language: "en",
      });
      setTranscript(result);
    } catch (e) {
      setError(e instanceof IpcError ? e.message : "Transcription failed unexpectedly.");
    } finally {
      setBusy(false);
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
        <p role="alert" className="mt-3 text-sm text-avid-danger">
          {error}
        </p>
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
