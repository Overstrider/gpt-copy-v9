"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { createMessage, deleteMessage, listMessages } from "@/lib/api";

export const messagesKey = (conversationId: string) =>
  ["messages", conversationId] as const;

export function useMessages(conversationId: string | null) {
  return useQuery({
    queryKey: messagesKey(conversationId ?? ""),
    queryFn: () => listMessages(conversationId!),
    enabled: !!conversationId,
  });
}

export function useCreateMessage(conversationId: string) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (content: string) => createMessage(conversationId, content),
    onSuccess: () =>
      qc.invalidateQueries({ queryKey: messagesKey(conversationId) }),
  });
}

export function useDeleteMessage(conversationId: string) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (messageId: string) =>
      deleteMessage(conversationId, messageId),
    onSuccess: () =>
      qc.invalidateQueries({ queryKey: messagesKey(conversationId) }),
  });
}
