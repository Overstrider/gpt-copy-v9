"use client";

import { useCallback, useRef, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { streamChat } from "@/lib/stream";
import { messagesKey } from "./use-messages";

export type StreamState = "idle" | "streaming" | "error";

export function useChatStream(conversationId: string | null) {
  const [streamState, setStreamState] = useState<StreamState>("idle");
  const [streamContent, setStreamContent] = useState<string>("");
  const [streamError, setStreamError] = useState<string | null>(null);
  const abortRef = useRef<AbortController | null>(null);
  const qc = useQueryClient();

  const submit = useCallback(
    async (content: string) => {
      if (!conversationId) return;
      if (streamState === "streaming") return;
      if (!content.trim()) return;

      abortRef.current = new AbortController();
      setStreamState("streaming");
      setStreamContent("");
      setStreamError(null);

      await streamChat(
        conversationId,
        content,
        {
          onDelta: (delta) => setStreamContent((prev) => prev + delta),
          onDone: () => {
            setStreamState("idle");
            // Refresh messages to show persisted assistant reply
            qc.invalidateQueries({
              queryKey: messagesKey(conversationId),
            });
            setStreamContent("");
          },
          onError: (msg) => {
            setStreamState("error");
            setStreamError(msg);
          },
        },
        abortRef.current.signal
      );
    },
    [conversationId, streamState, qc]
  );

  const abort = useCallback(() => {
    abortRef.current?.abort();
    setStreamState("idle");
  }, []);

  return {
    submit,
    abort,
    streamState,
    streamContent,
    streamError,
    isStreaming: streamState === "streaming",
  };
}
