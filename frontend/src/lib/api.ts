import { z } from "zod";
import {
  Conversation,
  ConversationListSchema,
  ConversationSchema,
  Message,
  MessageListSchema,
  MessageSchema,
} from "./schemas";

const BASE_URL =
  process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3001";

async function fetchJson<T>(
  schema: z.ZodType<T>,
  input: RequestInfo,
  init?: RequestInit
): Promise<T> {
  const res = await fetch(`${BASE_URL}${input}`, {
    headers: { "Content-Type": "application/json", ...init?.headers },
    ...init,
  });
  if (!res.ok) {
    let errMsg = `HTTP ${res.status}`;
    try {
      const errBody = await res.json();
      if (errBody?.message) errMsg = errBody.message;
    } catch {
      // ignore parse error
    }
    throw new Error(errMsg);
  }
  const data = await res.json();
  return schema.parse(data);
}

// ── Conversations ─────────────────────────────────────────────────────────────

export async function listConversations(): Promise<Conversation[]> {
  return fetchJson(ConversationListSchema, "/api/conversations");
}

export async function createConversation(title?: string): Promise<Conversation> {
  return fetchJson(ConversationSchema, "/api/conversations", {
    method: "POST",
    body: JSON.stringify({ title }),
  });
}

export async function getConversation(id: string): Promise<Conversation> {
  return fetchJson(ConversationSchema, `/api/conversations/${id}`);
}

export async function updateConversation(
  id: string,
  title: string
): Promise<Conversation> {
  return fetchJson(ConversationSchema, `/api/conversations/${id}`, {
    method: "PATCH",
    body: JSON.stringify({ title }),
  });
}

export async function deleteConversation(id: string): Promise<void> {
  const res = await fetch(`${BASE_URL}/api/conversations/${id}`, {
    method: "DELETE",
  });
  if (!res.ok && res.status !== 204) {
    throw new Error(`HTTP ${res.status}`);
  }
}

// ── Messages ──────────────────────────────────────────────────────────────────

export async function listMessages(conversationId: string): Promise<Message[]> {
  return fetchJson(
    MessageListSchema,
    `/api/conversations/${conversationId}/messages`
  );
}

export async function createMessage(
  conversationId: string,
  content: string
): Promise<Message> {
  return fetchJson(
    MessageSchema,
    `/api/conversations/${conversationId}/messages`,
    {
      method: "POST",
      body: JSON.stringify({ content }),
    }
  );
}

export async function deleteMessage(
  conversationId: string,
  messageId: string
): Promise<void> {
  const res = await fetch(
    `${BASE_URL}/api/conversations/${conversationId}/messages/${messageId}`,
    { method: "DELETE" }
  );
  if (!res.ok && res.status !== 204) {
    throw new Error(`HTTP ${res.status}`);
  }
}
