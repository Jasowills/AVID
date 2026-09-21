import { create } from "zustand";

export interface ToastAction {
  label: string;
  run: () => void;
}

export interface Toast {
  id: number;
  kind: "success" | "error" | "info";
  message: string;
  action?: ToastAction;
}

interface ToastState {
  toasts: Toast[];
  push: (toast: Omit<Toast, "id">) => void;
  dismiss: (id: number) => void;
}

let nextId = 1;
const MAX_TOASTS = 4;

/**
 * Token-native toast store (part-2 §§35–36): bottom-right, capped,
 * dismissible, action-capable, never modal. Success confirmations carry
 * the next action (Undo, Open file); errors stay inline where the user
 * must act — toasts never replace error states.
 */
export const useToastStore = create<ToastState>()((set) => ({
  toasts: [],
  push: (toast) =>
    set((state) => ({
      toasts: [...state.toasts, { ...toast, id: nextId++ }].slice(-MAX_TOASTS),
    })),
  dismiss: (id) => set((state) => ({ toasts: state.toasts.filter((t) => t.id !== id) })),
}));

/** Convenience: success toast with an optional next action. */
export function toastSuccess(message: string, action?: ToastAction): void {
  useToastStore.getState().push({ kind: "success", message, action });
}
