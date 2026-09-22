/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/components/ui/tooltip.tsx) — hand-rolled for React (no
 * Kobalte dependency): CSS hover/focus reveal with a small delay, arrowless,
 * always paired with an aria-label on the trigger. */

import type { ReactNode } from "react";

export interface TooltipProps {
  /** Visible label (also used as the trigger's fallback aria-label). */
  label: string;
  /** Optional keyboard hint rendered as <Kbd>. */
  shortcut?: string;
  children: ReactNode;
}

/**
 * Tooltip wrapper. The trigger keeps its own semantics — wrap buttons and
 * icon buttons, never plain text. Content is hover/focus revealed; screen
 * readers get the label via aria-label, so tooltips never carry unique info.
 */
export function Tooltip({ label, shortcut, children }: TooltipProps) {
  return (
    <span className="group/tooltip relative inline-flex">
      {children}
      <span
        role="tooltip"
        className="pointer-events-none absolute bottom-full left-1/2 z-50 mb-1.5 -translate-x-1/2 whitespace-nowrap rounded-avid-sm border border-avid-border bg-avid-overlay px-2 py-1 text-[11px] text-avid-primary opacity-0 shadow-lg transition-opacity delay-150 duration-100 group-hover/tooltip:opacity-100 group-focus-visible/tooltip:opacity-100"
      >
        {label}
        {shortcut && <span className="ml-1.5 font-mono text-[10px] text-avid-muted">{shortcut}</span>}
      </span>
    </span>
  );
}
