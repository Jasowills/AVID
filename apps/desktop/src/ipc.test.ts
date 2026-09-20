import { beforeEach, describe, expect, it, vi } from "vitest";
import { invokeCommand, isTauri, IpcError, setInvoker } from "./lib/ipc";

describe("ipc client", () => {
  beforeEach(() => setInvoker(null));

  it("throws a typed error outside Tauri instead of failing silently", async () => {
    expect(isTauri()).toBe(false);
    const error = await invokeCommand("ping").catch((e: unknown) => e);
    expect(error).toBeInstanceOf(IpcError);
    expect((error as IpcError).code).toBe("NOT_IN_TAURI");
  });

  it("passes commands and camelCase args to the injected invoker", async () => {
    const invoke = vi.fn(async () => ({ version: "test" }));
    setInvoker(invoke);
    const result = await invokeCommand<{ version: string }>("get_app_info", { appHandle: 1 });
    expect(invoke).toHaveBeenCalledWith("get_app_info", { appHandle: 1 });
    expect(result).toEqual({ version: "test" });
  });

  it("wraps invoker failures in INVOKE_FAILED", async () => {
    setInvoker(async () => {
      throw new Error("boom");
    });
    const error = await invokeCommand("ping").catch((e: unknown) => e);
    expect((error as IpcError).code).toBe("INVOKE_FAILED");
  });

  it("preserves backend error codes for UI branching", async () => {
    setInvoker(async () => {
      throw { code: "AVID_TRANSCRIBE_001", message: "No model." };
    });
    const error = (await invokeCommand("ping").catch((e: unknown) => e)) as IpcError;
    expect(error.code).toBe("AVID_TRANSCRIBE_001");
    expect(error.message).toBe("No model.");
  });
});
