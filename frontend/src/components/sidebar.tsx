"use client";

import { MessageSquare, Plus, Trash2 } from "lucide-react";
import { Conversation } from "@/lib/schemas";

interface SidebarProps {
  conversations: Conversation[];
  activeId: string | null;
  isLoading: boolean;
  error: Error | null;
  onSelect: (id: string) => void;
  onCreate: () => void;
  onDelete: (id: string) => void;
}

export function Sidebar({
  conversations,
  activeId,
  isLoading,
  error,
  onSelect,
  onCreate,
  onDelete,
}: SidebarProps) {
  return (
    <aside
      className="flex h-full flex-col bg-gray-900"
      data-testid="sidebar"
    >
      <div className="flex items-center justify-between p-3 border-b border-gray-700">
        <span className="font-semibold text-sm text-gray-200">Conversations</span>
        <button
          onClick={onCreate}
          className="p-1.5 rounded hover:bg-gray-700 text-gray-400 hover:text-white transition"
          aria-label="New chat"
          data-testid="new-chat-btn"
        >
          <Plus size={18} />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto py-1">
        {isLoading && (
          <div
            className="px-3 py-2 text-sm text-gray-500 animate-pulse"
            data-testid="conversations-loading"
          >
            Loading…
          </div>
        )}
        {error && (
          <div
            className="px-3 py-2 text-sm text-red-400"
            data-testid="conversations-error"
          >
            Failed to load conversations
          </div>
        )}
        {!isLoading && !error && conversations.length === 0 && (
          <div className="px-3 py-2 text-sm text-gray-500">No conversations yet</div>
        )}
        {conversations.map((conv) => (
          <div
            key={conv.id}
            className={`group flex items-center gap-2 px-3 py-2 cursor-pointer rounded mx-1 my-0.5 text-sm transition ${
              conv.id === activeId
                ? "bg-gray-700 text-white"
                : "text-gray-300 hover:bg-gray-800"
            }`}
            onClick={() => onSelect(conv.id)}
            data-testid={`conversation-item-${conv.id}`}
          >
            <MessageSquare size={14} className="flex-shrink-0 text-gray-400" />
            <span className="flex-1 truncate">{conv.title}</span>
            <button
              onClick={(e) => {
                e.stopPropagation();
                onDelete(conv.id);
              }}
              className="opacity-0 group-hover:opacity-100 p-0.5 rounded hover:text-red-400 transition"
              aria-label={`Delete ${conv.title}`}
              data-testid={`delete-conv-${conv.id}`}
            >
              <Trash2 size={13} />
            </button>
          </div>
        ))}
      </div>
    </aside>
  );
}
