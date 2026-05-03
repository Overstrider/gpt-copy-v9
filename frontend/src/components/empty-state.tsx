"use client";

import { MessageCircle } from "lucide-react";

interface EmptyStateProps {
  onNewChat: () => void;
}

export function EmptyState({ onNewChat }: EmptyStateProps) {
  return (
    <div
      className="flex flex-1 flex-col items-center justify-center gap-4 text-gray-500"
      data-testid="empty-state-panel"
    >
      <MessageCircle size={48} className="opacity-30" />
      <p className="text-sm">Select a conversation or start a new chat</p>
      <button
        onClick={onNewChat}
        className="px-4 py-2 rounded-lg bg-blue-600 text-white text-sm hover:bg-blue-700 transition"
        data-testid="start-new-chat-btn"
      >
        New Chat
      </button>
    </div>
  );
}
