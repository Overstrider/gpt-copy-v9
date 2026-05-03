"use client";

import { useEffect, useRef } from "react";
import { Message } from "@/lib/schemas";
import { MessageBubble, StreamingBubble } from "./message";

interface TranscriptProps {
  messages: Message[];
  isLoading: boolean;
  error: Error | null;
  streamContent: string;
  isStreaming: boolean;
}

export function Transcript({
  messages,
  isLoading,
  error,
  streamContent,
  isStreaming,
}: TranscriptProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, streamContent]);

  return (
    <div
      className="flex-1 overflow-y-auto p-4"
      data-testid="transcript"
    >
      {isLoading && (
        <div
          className="text-center text-gray-500 text-sm py-8 animate-pulse"
          data-testid="messages-loading"
        >
          Loading messages…
        </div>
      )}
      {error && !isLoading && (
        <div
          className="text-center text-red-400 text-sm py-4"
          data-testid="messages-error"
        >
          Failed to load messages
        </div>
      )}
      {!isLoading && messages.length === 0 && !isStreaming && (
        <div
          className="text-center text-gray-500 text-sm py-16"
          data-testid="empty-state"
        >
          Send a message to start the conversation
        </div>
      )}
      {messages.map((msg) => (
        <MessageBubble key={msg.id} message={msg} />
      ))}
      {isStreaming && <StreamingBubble content={streamContent} />}
      <div ref={bottomRef} />
    </div>
  );
}
