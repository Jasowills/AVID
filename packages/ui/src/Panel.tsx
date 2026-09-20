import type { HTMLAttributes, ReactNode } from "react";

export interface PanelProps extends HTMLAttributes<HTMLElement> {
  title?: string;
  actions?: ReactNode;
  children: ReactNode;
}

/** Card/panel container with optional header row. */
export function Panel({ title, actions, children, className = "", ...rest }: PanelProps) {
  return (
    <section
      className={`rounded-avid-lg border border-avid-border bg-avid-panel p-4 ${className}`}
      {...rest}
    >
      {(title ?? actions) && (
        <header className="mb-3 flex items-center justify-between">
          {title ? <h2 className="text-sm font-semibold text-avid-primary">{title}</h2> : <span />}
          {actions}
        </header>
      )}
      {children}
    </section>
  );
}
