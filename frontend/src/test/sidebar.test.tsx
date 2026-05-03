import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { Sidebar } from "@/components/sidebar";
import { Conversation } from "@/lib/schemas";

const mockConversations: Conversation[] = [
  {
    id: "c1",
    title: "First Chat",
    created_at: "2024-01-01T00:00:00Z",
    updated_at: "2024-01-01T00:00:00Z",
  },
  {
    id: "c2",
    title: "Second Chat",
    created_at: "2024-01-01T00:01:00Z",
    updated_at: "2024-01-01T00:01:00Z",
  },
];

function renderSidebar(overrides = {}) {
  const props = {
    conversations: mockConversations,
    activeId: null,
    isLoading: false,
    error: null,
    onSelect: vi.fn(),
    onCreate: vi.fn(),
    onDelete: vi.fn(),
    ...overrides,
  };
  return render(<Sidebar {...props} />);
}

describe("Sidebar", () => {
  it("renders conversation list", () => {
    renderSidebar();
    expect(screen.getByText("First Chat")).toBeInTheDocument();
    expect(screen.getByText("Second Chat")).toBeInTheDocument();
  });

  it("shows loading state", () => {
    renderSidebar({ conversations: [], isLoading: true });
    expect(screen.getByTestId("conversations-loading")).toBeInTheDocument();
  });

  it("shows error state", () => {
    renderSidebar({
      conversations: [],
      error: new Error("Network error"),
    });
    expect(screen.getByTestId("conversations-error")).toBeInTheDocument();
  });

  it("highlights active conversation", () => {
    renderSidebar({ activeId: "c1" });
    const item = screen.getByTestId("conversation-item-c1");
    expect(item.className).toContain("bg-gray-700");
  });

  it("calls onSelect when clicked", () => {
    const onSelect = vi.fn();
    renderSidebar({ onSelect });
    fireEvent.click(screen.getByText("First Chat"));
    expect(onSelect).toHaveBeenCalledWith("c1");
  });

  it("calls onCreate when new chat button clicked", () => {
    const onCreate = vi.fn();
    renderSidebar({ onCreate });
    fireEvent.click(screen.getByTestId("new-chat-btn"));
    expect(onCreate).toHaveBeenCalled();
  });
});
