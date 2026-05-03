import { z } from "zod";

export const ConversationSchema = z.object({
  id: z.string(),
  title: z.string(),
  created_at: z.string(),
  updated_at: z.string(),
});

export const ConversationListSchema = z.array(ConversationSchema);

export const MessageSchema = z.object({
  id: z.string(),
  conversation_id: z.string(),
  role: z.enum(["user", "assistant", "system"]),
  content: z.string(),
  created_at: z.string(),
});

export const MessageListSchema = z.array(MessageSchema);

export const ApiErrorSchema = z.object({
  code: z.string(),
  message: z.string(),
});

// Stream event schemas
export const StreamDeltaEventSchema = z.object({
  type: z.literal("delta"),
  content: z.string(),
});

export const StreamDoneEventSchema = z.object({
  type: z.literal("done"),
});

export const StreamErrorEventSchema = z.object({
  type: z.literal("error"),
  message: z.string(),
});

export const StreamEventSchema = z.discriminatedUnion("type", [
  StreamDeltaEventSchema,
  StreamDoneEventSchema,
  StreamErrorEventSchema,
]);

export type Conversation = z.infer<typeof ConversationSchema>;
export type Message = z.infer<typeof MessageSchema>;
export type ApiError = z.infer<typeof ApiErrorSchema>;
export type StreamEvent = z.infer<typeof StreamEventSchema>;
