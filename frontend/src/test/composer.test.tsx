import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { Composer } from "@/components/composer";

function renderComposer(props = {}) {
  return render(
    <Composer
      onSubmit={vi.fn()}
      isStreaming={false}
      {...props}
    />
  );
}

describe("Composer", () => {
  it("renders textarea and send button", () => {
    renderComposer();
    expect(screen.getByTestId("composer-textarea")).toBeInTheDocument();
    expect(screen.getByTestId("composer-send")).toBeInTheDocument();
  });

  it("send button disabled when empty", () => {
    renderComposer();
    expect(screen.getByTestId("composer-send")).toBeDisabled();
  });

  it("send button enabled when text entered", () => {
    renderComposer();
    const ta = screen.getByTestId("composer-textarea");
    fireEvent.change(ta, { target: { value: "Hello" } });
    expect(screen.getByTestId("composer-send")).not.toBeDisabled();
  });

  it("calls onSubmit with trimmed content", () => {
    const onSubmit = vi.fn();
    renderComposer({ onSubmit });
    const ta = screen.getByTestId("composer-textarea");
    fireEvent.change(ta, { target: { value: "  Hello  " } });
    fireEvent.click(screen.getByTestId("composer-send"));
    expect(onSubmit).toHaveBeenCalledWith("Hello");
  });

  it("does not submit empty content", () => {
    const onSubmit = vi.fn();
    renderComposer({ onSubmit });
    fireEvent.click(screen.getByTestId("composer-send"));
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("shows stop button when streaming", () => {
    renderComposer({ isStreaming: true });
    expect(screen.getByTestId("composer-stop")).toBeInTheDocument();
    expect(screen.queryByTestId("composer-send")).not.toBeInTheDocument();
  });

  it("submits on Enter key", () => {
    const onSubmit = vi.fn();
    renderComposer({ onSubmit });
    const ta = screen.getByTestId("composer-textarea");
    fireEvent.change(ta, { target: { value: "Hello" } });
    fireEvent.keyDown(ta, { key: "Enter", shiftKey: false });
    expect(onSubmit).toHaveBeenCalledWith("Hello");
  });

  it("does not submit on Shift+Enter", () => {
    const onSubmit = vi.fn();
    renderComposer({ onSubmit });
    const ta = screen.getByTestId("composer-textarea");
    fireEvent.change(ta, { target: { value: "Hello" } });
    fireEvent.keyDown(ta, { key: "Enter", shiftKey: true });
    expect(onSubmit).not.toHaveBeenCalled();
  });
});
