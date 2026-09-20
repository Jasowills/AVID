import type { ReactNode } from "react";

export interface EmptyStateProps {
  title: string;
  body: string;
  actions?: ReactNode;
}

/** Guidance-first empty state — never a bare blank panel (AGENTS §57). */
export function EmptyState({ title, body, actions }: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center gap-2 rounded-avid-lg border border-dashed border-avid-border-strong px-6 py-10 text-center">
      <h3 className="text-sm font-semibold text-avid-primary">{title}</h3>
      <p className="max-w-sm text-sm text-avid-secondary">{body}</p>
      {actions && <div className="mt-2 flex flex-wrap justify-center gap-2">{actions}</div>}
    </div>
  );
}
