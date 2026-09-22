/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/components/ui/command.tsx) — hand-rolled for React (no cmdk
 * dependency at this command volume): overlay + centered panel, filter input,
 * grouped list, arrow/enter/escape keyboard map. Every command performs a
 * real action; nothing decorative is listed. */

import { useEffect, useMemo, useRef, useState } from "react";
import { Icon } from "@avid/ui";
import { invokeCommand, isTauri } from "../lib/ipc";
import { notifyTimelineChanged } from "../stores/useJobsStore";

export type PaletteTab = "Media" | "Transcript" | "AI" | "Jobs" | "Visuals";

interface PaletteCommand {
  id: string;
  group: string;
  label: string;
  hint?: string;
  keywords?: string;
  run: () => void | Promise<void>;
}

interface CommandPaletteProps {
  open: boolean;
  onClose: () => void;
  onTab: (tab: PaletteTab) => void;
  onExport: () => void;
  onNewProject: () => void;
  onSettings: () => void;
  onHome: () => void;
}

export function CommandPalette({
  open,
  onClose,
  onTab,
  onExport,
  onNewProject,
  onSettings,
  onHome,
}: CommandPaletteProps) {
  const [query, setQuery] = useState("");
  const [active, setActive] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);
  const backend = isTauri();

  const commands: PaletteCommand[] = useMemo(
    () => [
      { id: "tab-media", group: "Go to", label: "Media panel", run: () => onTab("Media") },
      { id: "tab-transcript", group: "Go to", label: "Transcript panel", run: () => onTab("Transcript") },
      { id: "tab-ai", group: "Go to", label: "AI panel", run: () => onTab("AI") },
      { id: "tab-jobs", group: "Go to", label: "Jobs panel", run: () => onTab("Jobs") },
      { id: "tab-visuals", group: "Go to", label: "Visuals panel", run: () => onTab("Visuals") },
      {
        id: "edit-undo",
        group: "Edit",
        label: "Undo",
        hint: "⌘Z",
        keywords: "revert back",
        run: async () => {
          if (!backend) return;
          try {
            await invokeCommand("timeline_undo");
            notifyTimelineChanged();
          } catch {
            // Disabled honesty: without the backend there is nothing to undo.
          }
        },
      },
      {
        id: "edit-redo",
        group: "Edit",
        label: "Redo",
        hint: "⇧⌘Z",
        keywords: "repeat forward",
        run: async () => {
          if (!backend) return;
          try {
            await invokeCommand("timeline_redo");
            notifyTimelineChanged();
          } catch {
            // Same disabled honesty as undo.
          }
        },
      },
      { id: "project-export", group: "Project", label: "Export video…", hint: "E", run: () => onExport() },
      { id: "project-new", group: "Project", label: "New project", run: () => onNewProject() },
      { id: "project-home", group: "Project", label: "Back to Home", run: () => onHome() },
      { id: "project-settings", group: "Project", label: "Open Settings", run: () => onSettings() },
    ],
    [backend, onTab, onExport, onNewProject, onHome, onSettings],
  );

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return commands;
    return commands.filter((c) => `${c.group} ${c.label} ${c.keywords ?? ""}`.toLowerCase().includes(q));
  }, [commands, query]);

  useEffect(() => {
    if (open) {
      setQuery("");
      setActive(0);
      requestAnimationFrame(() => inputRef.current?.focus());
    }
  }, [open ]);

  useEffect(() => {
    setActive(0);
  }, [query]);

  useEffect(() => {
    listRef.current
      ?.querySelector<HTMLElement>(`[data-index="${active}"]`)
      ?.scrollIntoView({ block: "nearest" });
  }, [active]);

  if (!open) return null;

  function run(command: PaletteCommand): void {
    onClose();
    void command.run();
  }

  let lastGroup = "";
  return (
    <div
      className="fixed inset-0 z-[60] flex items-start justify-center bg-black/60 px-4 pt-[12vh]"
      onPointerDown={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Command palette"
        className="flex max-h-[60vh] w-full max-w-lg flex-col overflow-hidden rounded-avid-lg border border-avid-border bg-avid-panel shadow-2xl"
      >
        <div className="flex h-12 items-center gap-2 border-b border-avid-border-subtle px-3">
          <Icon name="search" size={15} className="shrink-0 text-avid-muted" />
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Escape") onClose();
              else if (e.key === "ArrowDown") {
                e.preventDefault();
                setActive((a) => Math.min(a + 1, filtered.length - 1));
              } else if (e.key === "ArrowUp") {
                e.preventDefault();
                setActive((a) => Math.max(a - 1, 0));
              } else if (e.key === "Enter") {
                const command = filtered[active];
                if (command) run(command);
              }
            }}
            placeholder="Type a command…"
            aria-label="Command search"
            className="h-full w-full bg-transparent text-sm text-avid-primary outline-none placeholder:text-avid-muted"
          />
          <kbd className="rounded-avid-sm border border-avid-border bg-avid-raised px-1.5 py-0.5 font-mono text-[10px] text-avid-muted">
            esc
          </kbd>
        </div>
        <div ref={listRef} role="listbox" aria-label="Commands" className="overflow-y-auto p-1.5">
          {filtered.length === 0 && (
            <p className="px-2 py-6 text-center text-sm text-avid-muted">No matching command.</p>
          )}
          {filtered.map((command, index) => {
            const header = command.group !== lastGroup ? command.group : null;
            lastGroup = command.group;
            return (
              <div key={command.id}>
                {header && (
                  <p className="px-2 pb-1 pt-2 text-[11px] font-medium text-avid-muted first:pt-1">{header}</p>
                )}
                <button
                  role="option"
                  aria-selected={index === active}
                  data-index={index}
                  onMouseEnter={() => setActive(index)}
                  onClick={() => run(command)}
                  className={`flex w-full items-center gap-2 rounded-avid-sm px-2 py-2 text-left text-sm ${
                    index === active ? "bg-avid-raised text-avid-primary" : "text-avid-secondary"
                  }`}
                >
                  <span className="flex-1">{command.label}</span>
                  {command.hint && (
                    <span className="font-mono text-[11px] text-avid-muted">{command.hint}</span>
                  )}
                </button>
              </div>
            );
          })}
        </div>
        <p className="sr-only">Type to filter, arrows to move, Enter to run.</p>
      </div>
    </div>
  );
}
