"use client";

import { useState } from "react";
import { Menu, X } from "lucide-react";
import { Sidebar } from "./sidebar";
import { Transcript } from "./transcript";
import { Composer } from "./composer";
import { EmptyState } from "./empty-state";
import {
  useConversations,
  useCreateConversation,
  useDeleteConversation,
} from "@/hooks/use-conversations";
import { useMessages } from "@/hooks/use-messages";
import { useChatStream } from "@/hooks/use-chat-stream";

export function ChatShell() {
  const [activeConversationId, setActiveConversationId] = useState<
    string | null
  >(null);
  const [sidebarOpen, setSidebarOpen] = useState(false);

  const {
    data: conversations = [],
    isLoading: convsLoading,
    error: convsError,
  } = useConversations();

  const createConv = useCreateConversation();
  const deleteConv = useDeleteConversation();

  const {
    data: messages = [],
    isLoading: msgsLoading,
    error: msgsError,
  } = useMessages(activeConversationId);

  const { submit, abort, isStreaming, streamContent, streamError } =
    useChatStream(activeConversationId);

  const handleNewChat = async () => {
    const conv = await createConv.mutateAsync("New Chat");
    setActiveConversationId(conv.id);
    setSidebarOpen(false);
  };

  const handleSelect = (id: string) => {
    setActiveConversationId(id);
    setSidebarOpen(false);
  };

  const handleDelete = async (id: string) => {
    await deleteConv.mutateAsync(id);
    if (activeConversationId === id) {
      setActiveConversationId(null);
    }
  };

  const handleSubmit = (content: string) => {
    submit(content);
  };

  return (
    <div className="flex h-full relative" data-testid="chat-shell">
      {/* Mobile overlay */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 bg-black/50 z-10 md:hidden"
          onClick={() => setSidebarOpen(false)}
          data-testid="sidebar-overlay"
        />
      )}

      {/* Sidebar */}
      <div
        className={`
          fixed md:relative z-20 md:z-auto
          h-full w-64 flex-shrink-0
          transform transition-transform duration-200
          ${sidebarOpen ? "translate-x-0" : "-translate-x-full md:translate-x-0"}
        `}
        data-testid="sidebar-panel"
      >
        <Sidebar
          conversations={conversations}
          activeId={activeConversationId}
          isLoading={convsLoading}
          error={convsError as Error | null}
          onSelect={handleSelect}
          onCreate={handleNewChat}
          onDelete={handleDelete}
        />
      </div>

      {/* Main content */}
      <div className="flex flex-1 flex-col min-w-0">
        {/* Mobile header */}
        <div className="flex items-center gap-2 px-3 py-2 border-b border-gray-700 md:hidden">
          <button
            onClick={() => setSidebarOpen(!sidebarOpen)}
            className="p-1 rounded hover:bg-gray-700 text-gray-400"
            aria-label="Toggle sidebar"
            data-testid="sidebar-toggle"
          >
            {sidebarOpen ? <X size={20} /> : <Menu size={20} />}
          </button>
          <span className="text-sm font-medium text-gray-300">
            {conversations.find((c) => c.id === activeConversationId)?.title ??
              "gpt-copy-v9"}
          </span>
        </div>

        {activeConversationId ? (
          <>
            <Transcript
              messages={messages}
              isLoading={msgsLoading}
              error={msgsError as Error | null}
              streamContent={streamContent}
              isStreaming={isStreaming}
            />
            {streamError && (
              <div
                className="px-4 py-2 text-xs text-red-400 bg-red-900/20 border-t border-red-800"
                data-testid="stream-error"
              >
                Stream error: {streamError}
              </div>
            )}
            <Composer
              onSubmit={handleSubmit}
              onAbort={abort}
              isStreaming={isStreaming}
            />
          </>
        ) : (
          <EmptyState onNewChat={handleNewChat} />
        )}
      </div>
    </div>
  );
}
