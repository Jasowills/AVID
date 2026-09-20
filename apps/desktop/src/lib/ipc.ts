/**
 * Tauri IPC client (Phase 1.1 wiring, commands land with the backend).
 *
 * Commands are the ONLY mutation path to Rust state (ADR-001). The invoker
 * is injected at startup when running inside the Tauri webview; in the
 * browser it stays unset and calls fail with a typed error — never silently.
 */

export interface IpcErrorInfo {
  /** Backend `code` when the Rust command failed, else the client-side code. */
  code: string;
  message: string;
}

export class IpcError extends Error {
  readonly code: string;

  constructor(code: string, message: string) {
    super(message);
    this.name = "IpcError";
    this.code = code;
  }
}

/** Tauri invoke signature (`cmd`, camelCase args). */
export type InvokeFn = (cmd: string, args?: Record<string, unknown>) => Promise<unknown>;

let invoker: InvokeFn | null = null;

/** Inject the platform invoker (called once at startup under Tauri). Test seam. */
export function setInvoker(fn: InvokeFn | null): void {
  invoker = fn;
}

/** True when running inside the Tauri webview. */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/** Invoke a Rust command. Throws {@link IpcError} outside Tauri. */
export async function invokeCommand<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!invoker) {
    throw new IpcError(
      "NOT_IN_TAURI",
      `Command '${cmd}' needs the Tauri backend (running in the browser).`,
    );
  }
  try {
    return (await invoker(cmd, args)) as T;
  } catch (error) {
    // Tauri rejects with the serialized command error ({code, message}) —
    // preserve the backend code so the UI can branch on it.
    if (typeof error === "object" && error !== null && "code" in error && "message" in error) {
      const { code, message } = error as { code: unknown; message: unknown };
      throw new IpcError(
        typeof code === "string" ? code : "INVOKE_FAILED",
        typeof message === "string" ? message : `Command '${cmd}' failed.`,
      );
    }
    throw new IpcError(
      "INVOKE_FAILED",
      `Command '${cmd}' failed: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
}
