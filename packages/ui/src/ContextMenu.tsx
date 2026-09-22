/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Ported structure: diffusionstudio/editor@57c3983
 * (apps/web/src/components/ui/context-menu.tsx, app-context-menu.tsx) —
 * hand-rolled for React: fixed-position menu, outside-click + Escape close,
 * menu/menuitem roles, disabled honesty. No submenus (keep it predictable). */

import { useEffect, useRef, useState } from "react";
import type { ReactNode } from "react";

export interface ContextMenuItem {
  id: string;
  label: string;
  /** Hint shown right-aligned (e.g. "S", "⌫"). */
  hint?: string;
  danger?: boolean;
  disabled?: boolean;
  disabledReason?: string;
  run: () => void;
}

export interface ContextMenuProps {
  /** Viewport anchor from the contextmenu event. */
  x: number;
  y: number;
  items: ContextMenuItem[];
  onClose: () => void;
}

const MENU_WIDTH = 224;

/** Clamp the menu inside the viewport (never clipped off-screen). */
export function clampMenuPosition(x: number, y: number, menuHeight: number): { left: number; top: number } {
  const width = typeof window === "undefined" ? 1024 : window.innerWidth;
  const height = typeof window === "undefined" ? 768 : window.innerHeight;
  return {
    left: Math.max(8, Math.min(x, width - MENU_WIDTH - 8)),
    top: Math.max(8, Math.min(y, height - menuHeight - 8)),
  };
}

export function ContextMenu({ x, y, items, onClose }: ContextMenuProps) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onKey = (event: KeyboardEvent): void => {
      if (event.key === "Escape") onClose();
    };
    const onPointer = (event: PointerEvent): void => {
      if (ref.current && !ref.current.contains(event.target as Node)) onClose();
    };
    window.addEventListener("keydown", onKey);
    window.addEventListener("pointerdown", onPointer);
    ref.current?.querySelector<HTMLElement>("[role='menuitem']:not([aria-disabled='true'])")?.focus();
    return () => {
      window.removeEventListener("keydown", onKey);
      window.removeEventListener("pointerdown", onPointer);
    };
  }, [onClose]);

  // Rough height estimate for clamping before first paint (36px per row).
  const { left, top } = clampMenuPosition(x, y, items.length * 36 + 16);

  return (
    <div
      ref={ref}
      role="menu"
      aria-label="Context menu"
      className="fixed z-[70] w-56 rounded-avid-md border border-avid-border bg-avid-overlay p-1 shadow-xl"
      style={{ left, top }}
    >
      {items.map((item) => (
        <button
          key={item.id}
          role="menuitem"
          disabled={item.disabled}
          title={item.disabled ? (item.disabledReason ?? item.label) : undefined}
          onClick={() => {
            onClose();
            item.run();
          }}
          className={`flex w-full items-center gap-2 rounded-avid-sm px-2 py-1.5 text-left text-xs ${
            item.danger ? "text-avid-danger" : "text-avid-primary"
          } hover:bg-avid-raised focus-visible:bg-avid-raised focus-visible:outline-2 focus-visible:outline-avid-accent disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent`}
        >
          <span className="flex-1">{item.label}</span>
          {item.hint && <span className="font-mono text-[10px] text-avid-muted">{item.hint}</span>}
        </button>
      ))}
    </div>
  );
}

/** Wrapper: right-click area that opens the menu. Callers close over whatever
 *  the menu needs (e.g. the clip under the cursor) — no event plumbing. */
export function ContextArea({
  items,
  children,
}: {
  items: ContextMenuItem[];
  children: (open: (event: React.MouseEvent) => void) => ReactNode;
}) {
  const [pos, setPos] = useState<{ x: number; y: number } | null>(null);

  function open(event: React.MouseEvent): void {
    event.preventDefault();
    setPos({ x: event.clientX, y: event.clientY });
  }

  return (
    <>
      {children(open)}
      {pos && <ContextMenu x={pos.x} y={pos.y} items={items} onClose={() => setPos(null)} />}
    </>
  );
}
