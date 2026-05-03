import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { MessageBubble, StreamingBubble } from "@/components/message";
import { Message } from "@/lib/schemas";

const userMessage: Message = {
  id: "m1",
  conversation_id: "c1",
  role: "user",
  content: "Hello world",
  created_at: "2024-01-01T00:00:00Z",
};

const assistantMessage: Message = {
  id: "m2",
  conversation_id: "c1",
  role: "assistant",
  content: "**Bold** and `code`",
  created_at: "2024-01-01T00:01:00Z",
};

describe("MessageBubble", () => {
  it("renders user message content", () => {
    render(<MessageBubble message={userMessage} />);
    expect(screen.getByText("Hello world")).toBeInTheDocument();
  });

  it("renders assistant message with markdown", () => {
    render(<MessageBubble message={assistantMessage} />);
    // Bold text should be rendered
    expect(screen.getByText("Bold")).toBeInTheDocument();
    expect(screen.getByText("code")).toBeInTheDocument();
  });

  it("does not use dangerouslySetInnerHTML", () => {
    const xssMessage: Message = {
      ...assistantMessage,
      content: "<script>alert('xss')</script>",
    };
    render(<MessageBubble message={xssMessage} />);
    expect(
      document.querySelector("script[data-dangerous]")
    ).not.toBeInTheDocument();
  });
});

describe("StreamingBubble", () => {
  it("renders streaming content", () => {
    render(<StreamingBubble content="Streaming text" />);
    expect(screen.getByTestId("streaming-bubble")).toBeInTheDocument();
    expect(screen.getByText("Streaming text")).toBeInTheDocument();
  });

  it("shows cursor when empty", () => {
    render(<StreamingBubble content="" />);
    expect(screen.getByText("▋")).toBeInTheDocument();
  });
});
