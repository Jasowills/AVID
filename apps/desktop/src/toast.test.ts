import { describe, expect, it } from "vitest";
import { toastSuccess, useToastStore } from "./stores/useToastStore";
import { act } from "react";

describe("toast store", () => {
  it("caps the stack and dismisses by id", () => {
    act(() => {
      useToastStore.setState({ toasts: [] });
      for (let i = 0; i < 6; i++) toastSuccess(`done ${i}`);
    });
    const { toasts, dismiss } = useToastStore.getState();
    expect(toasts).toHaveLength(4);
    expect(toasts[0]?.message).toBe("done 2");
    act(() => dismiss(toasts[0]?.id ?? -1));
    expect(useToastStore.getState().toasts).toHaveLength(3);
  });
});
