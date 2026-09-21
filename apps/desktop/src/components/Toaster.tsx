import { useEffect } from "react";
import { Icon } from "@avid/ui";
import { useToastStore } from "../stores/useToastStore";

/**
 * Bottom-right toaster: capped stack, auto-dismiss, action buttons,
 * `role="status"` announcements. Never modal, never blocks playback.
 */
export function Toaster() {
  const toasts = useToastStore((s) => s.toasts);
  const dismiss = useToastStore((s) => s.dismiss);

  useEffect(() => {
    if (toasts.length === 0) return;
    const timer = setTimeout(() => {
      const [oldest] = toasts;
      if (oldest) dismiss(oldest.id);
    }, 6000);
    return () => clearTimeout(timer);
  }, [toasts, dismiss]);

  if (toasts.length === 0) return null;

  return (
    <div
      aria-live="polite"
      className="pointer-events-none fixed bottom-4 right-4 z-50 flex w-80 flex-col gap-2"
    >
      {toasts.map((toast) => (
        <div
          key={toast.id}
          role="status"
          className="pointer-events-auto flex items-start gap-2 rounded-avid-md border border-avid-border bg-avid-panel px-3 py-2 shadow-lg"
        >
          <span
            aria-hidden="true"
            className={`mt-0.5 ${
              toast.kind === "success"
                ? "text-avid-success"
                : toast.kind === "error"
                  ? "text-avid-danger"
                  : "text-avid-secondary"
            }`}
          >
            <Icon
              name={toast.kind === "success" ? "check" : toast.kind === "error" ? "alert" : "info"}
              size={14}
            />
          </span>
          <p className="min-w-0 flex-1 text-xs text-avid-primary">{toast.message}</p>
          {toast.action && (
            <button
              onClick={() => {
                toast.action?.run();
                dismiss(toast.id);
              }}
              className="shrink-0 rounded-avid-sm px-1.5 py-0.5 text-xs font-medium text-avid-accent hover:bg-avid-accent-muted"
            >
              {toast.action.label}
            </button>
          )}
          <button
            onClick={() => dismiss(toast.id)}
            aria-label="Dismiss notification"
            className="shrink-0 rounded-avid-sm p-0.5 text-avid-muted hover:text-avid-primary"
          >
            <Icon name="close" size={13} />
          </button>
        </div>
      ))}
    </div>
  );
}
