const STORAGE_KEY = "avid.provider.v1";

export interface ProviderSummary {
  baseUrl: string;
  model: string;
}

/** Last tested provider endpoint/model, if any (URL + model only — keys are never persisted). */
export function loadProviderSummary(): ProviderSummary | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;
    const parsed = JSON.parse(raw) as Partial<ProviderSummary>;
    if (typeof parsed.baseUrl === "string" && typeof parsed.model === "string") {
      return { baseUrl: parsed.baseUrl, model: parsed.model };
    }
    return null;
  } catch {
    return null;
  }
}

/** Persist endpoint/model after a successful connection test. Never pass a key here. */
export function saveProviderSummary(summary: ProviderSummary): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(summary));
  } catch {
    // Private mode etc. — config simply doesn't persist.
  }
}
