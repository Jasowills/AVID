/**
 * Tauri IPC client (Phase 1.1 wiring, commands land with the backend).
 *
 * Commands are the ONLY mutation path to Rust state (ADR-001). The invoker
 * is injected at startup when running inside the Tauri webview; in the
 * browser it stays unset and calls fail with a typed error — never silently.
 */

export interface IpcErrorInfo {
  code: "NOT_IN_TAURI" | "INVOKE_FAILED";
  message: string;
}

export class IpcError extends Error {
  readonly code: IpcErrorInfo["code"];

  constructor(code: IpcErrorInfo["code"], message: string) {
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
    throw new IpcError(
      "INVOKE_FAILED",
      `Command '${cmd}' failed: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
}
