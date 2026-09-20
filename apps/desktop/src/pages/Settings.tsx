import { Panel } from "@avid/ui";
import { ProvidersPanel } from "../components/ProvidersPanel";

/** Settings shell. Provider configuration UI lands in Phase 6 (AGENTS §80, §146). */
export function Settings() {
  return (
    <main className="mx-auto flex max-w-2xl flex-col gap-4 p-6">
      <h1 className="text-xl font-semibold">Settings</h1>
      <ProvidersPanel />
      <Panel title="Editing">
        <p className="text-sm text-avid-secondary">
          Proxy generation, autosave interval, and keyboard shortcut customization land with their
          phases (2, 3, and 10).
        </p>
      </Panel>
    </main>
  );
}
