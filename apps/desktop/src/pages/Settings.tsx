import { Panel } from "@avid/ui";
import { ProvidersPanel } from "../components/ProvidersPanel";
import { NavRail } from "../components/NavRail";

/** Settings: providers today; appearance, keyboard, and editor sections follow. */
export function Settings() {
  return (
    <div className="flex h-full bg-avid-base text-avid-primary">
      <NavRail />
      <main className="min-w-0 flex-1 overflow-y-auto">
        <div className="mx-auto flex max-w-2xl flex-col gap-4 p-6">
          <h1 className="text-lg font-semibold">Settings</h1>
      <ProvidersPanel />
      <Panel title="Editing">
        <p className="text-sm text-avid-secondary">
          Proxy generation runs per asset; autosave persists every committed change. Keyboard
          shortcut customization and density settings follow.
        </p>
      </Panel>
        </div>
      </main>
    </div>
  );
}
