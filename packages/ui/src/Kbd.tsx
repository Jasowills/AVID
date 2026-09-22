/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/components/ui/kbd.tsx) — keyboard hint chip for the palette,
 * tooltips, and shortcut lists. */

export function Kbd({ keys }: { keys: string }) {
  return (
    <kbd className="rounded-avid-sm border border-avid-border bg-avid-raised px-1.5 py-0.5 font-mono text-[10px] text-avid-secondary">
      {keys}
    </kbd>
  );
}
