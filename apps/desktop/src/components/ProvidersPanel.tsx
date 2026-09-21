import { useState } from "react";
import type { FormEvent } from "react";
import { Button, Panel, TextField } from "@avid/ui";
import { invokeCommand, IpcError } from "../lib/ipc";

export interface ProviderProbe {
  reachable: boolean;
  structured_output: boolean;
  message: string;
}

import { loadProviderSummary, saveProviderSummary } from "../lib/provider";

interface ProviderConfig {
  baseUrl: string;
  model: string;
  apiKey: string;
}

function loadConfig(): ProviderConfig {
  const saved = loadProviderSummary();
  return {
    baseUrl: saved?.baseUrl ?? "http://localhost:11434",
    model: saved?.model ?? "qwen2.5:7b",
    apiKey: "",
  };
}

/**
 * Provider settings (Phase 6 UI slice): OpenAI-compatible endpoint + model +
 * optional cloud key, with a live connection test. Local endpoints need no
 * key; anything with a key is labeled Cloud and asks per-operation consent.
 * Config persists in localStorage until the Rust-side store lands.
 */
export function ProvidersPanel() {
  const [config, setConfig] = useState<ProviderConfig>(loadConfig);
  const [busy, setBusy] = useState(false);
  const [probe, setProbe] = useState<ProviderProbe | null>(null);
  const [error, setError] = useState<string | null>(null);

  const isCloud = config.apiKey.trim() !== "";

  async function onTest(event: FormEvent): Promise<void> {
    event.preventDefault();
    setBusy(true);
    setProbe(null);
    setError(null);
    try {
      const result = await invokeCommand<ProviderProbe>("probe_provider", {
        baseUrl: config.baseUrl.trim(),
        model: config.model.trim(),
        apiKey: config.apiKey.trim() === "" ? null : config.apiKey,
      });
      setProbe(result);
      saveProviderSummary({ baseUrl: config.baseUrl.trim(), model: config.model.trim() });
    } catch (e) {
      setError(e instanceof IpcError ? e.message : "Probe failed unexpectedly.");
    } finally {
      setBusy(false);
    }
  }

  return (
    <Panel
      title="AI Providers"
      actions={
        <span
          className="rounded-full bg-avid-raised px-2 py-0.5 text-xs text-avid-secondary"
          title={isCloud ? "Requests will ask consent per operation" : "On-machine inference"}
        >
          {isCloud ? "Cloud" : "Local"}
        </span>
      }
    >
      <form onSubmit={onTest} className="flex flex-col gap-3">
        <TextField
          label="Base URL (OpenAI-compatible)"
          placeholder="http://localhost:11434"
          value={config.baseUrl}
          onChange={(e) => setConfig({ ...config, baseUrl: e.target.value })}
        />
        <TextField
          label="Model"
          placeholder="qwen2.5:7b"
          value={config.model}
          onChange={(e) => setConfig({ ...config, model: e.target.value })}
        />
        <TextField
          label="API key (cloud only — leave empty for local)"
          type="password"
          autoComplete="off"
          placeholder="sk-…"
          value={config.apiKey}
          onChange={(e) => setConfig({ ...config, apiKey: e.target.value })}
        />
        <div>
          <Button type="submit" variant="primary" disabled={busy}>
            {busy ? "Testing…" : "Test connection"}
          </Button>
        </div>
      </form>
      {error && (
        <p role="alert" className="mt-3 text-sm text-avid-danger">
          {error}
        </p>
      )}
      {probe && (
        <p className={`mt-3 text-sm ${probe.structured_output ? "text-avid-success" : "text-avid-warning"}`}>
          {probe.message}
        </p>
      )}
      <p className="mt-3 text-xs text-avid-muted">
        Keys stay on this machine and travel only to the endpoint above. Every cloud operation asks
        first — nothing uploads silently.
      </p>
    </Panel>
  );
}
