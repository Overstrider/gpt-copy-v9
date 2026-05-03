import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { Transcript } from "@/components/transcript";
import { Message } from "@/lib/schemas";

const messages: Message[] = [
  {
    id: "m1",
    conversation_id: "c1",
    role: "user",
    content: "Hello",
    created_at: "2024-01-01T00:00:00Z",
  },
  {
    id: "m2",
    conversation_id: "c1",
    role: "assistant",
    content: "Hi there!",
    created_at: "2024-01-01T00:01:00Z",
  },
];

describe("Transcript", () => {
  it("renders messages", () => {
    render(
      <Transcript
        messages={messages}
        isLoading={false}
        error={null}
        streamContent=""
        isStreaming={false}
      />
    );
    expect(screen.getByText("Hello")).toBeInTheDocument();
    expect(screen.getByText("Hi there!")).toBeInTheDocument();
  });

  it("shows loading state", () => {
    render(
      <Transcript
        messages={[]}
        isLoading={true}
        error={null}
        streamContent=""
        isStreaming={false}
      />
    );
    expect(screen.getByTestId("messages-loading")).toBeInTheDocument();
  });

  it("shows error state without clearing messages", () => {
    render(
      <Transcript
        messages={messages}
        isLoading={false}
        error={new Error("Failed")}
        streamContent=""
        isStreaming={false}
      />
    );
    expect(screen.getByTestId("messages-error")).toBeInTheDocument();
    // Messages should still be visible
    expect(screen.getByText("Hello")).toBeInTheDocument();
  });

  it("shows empty state when no messages", () => {
    render(
      <Transcript
        messages={[]}
        isLoading={false}
        error={null}
        streamContent=""
        isStreaming={false}
      />
    );
    expect(screen.getByTestId("empty-state")).toBeInTheDocument();
  });

  it("shows streaming bubble when streaming", () => {
    render(
      <Transcript
        messages={messages}
        isLoading={false}
        error={null}
        streamContent="Typing..."
        isStreaming={true}
      />
    );
    expect(screen.getByTestId("streaming-bubble")).toBeInTheDocument();
  });
});
