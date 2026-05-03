import { describe, it, expect } from "vitest";
import {
  ConversationSchema,
  MessageSchema,
  ApiErrorSchema,
  StreamEventSchema,
} from "@/lib/schemas";

describe("ConversationSchema", () => {
  it("parses valid conversation", () => {
    const data = {
      id: "abc",
      title: "Test",
      created_at: "2024-01-01T00:00:00Z",
      updated_at: "2024-01-01T00:00:00Z",
    };
    expect(ConversationSchema.parse(data)).toEqual(data);
  });

  it("rejects missing id", () => {
    expect(() =>
      ConversationSchema.parse({ title: "T", created_at: "x", updated_at: "x" })
    ).toThrow();
  });

  it("rejects missing title", () => {
    expect(() =>
      ConversationSchema.parse({ id: "x", created_at: "x", updated_at: "x" })
    ).toThrow();
  });
});

describe("MessageSchema", () => {
  it("parses valid user message", () => {
    const data = {
      id: "m1",
      conversation_id: "c1",
      role: "user",
      content: "Hello",
      created_at: "2024-01-01T00:00:00Z",
    };
    expect(MessageSchema.parse(data)).toEqual(data);
  });

  it("rejects invalid role", () => {
    expect(() =>
      MessageSchema.parse({
        id: "m1",
        conversation_id: "c1",
        role: "admin",
        content: "Hi",
        created_at: "x",
      })
    ).toThrow();
  });
});

describe("ApiErrorSchema", () => {
  it("parses error with code and message", () => {
    const err = { code: "NOT_FOUND", message: "Not found" };
    expect(ApiErrorSchema.parse(err)).toEqual(err);
  });

  it("rejects missing code", () => {
    expect(() => ApiErrorSchema.parse({ message: "x" })).toThrow();
  });
});

describe("StreamEventSchema", () => {
  it("parses delta event", () => {
    const e = { type: "delta", content: "hello" };
    expect(StreamEventSchema.parse(e)).toEqual(e);
  });

  it("parses done event", () => {
    const e = { type: "done" };
    expect(StreamEventSchema.parse(e)).toEqual(e);
  });

  it("parses error event", () => {
    const e = { type: "error", message: "failed" };
    expect(StreamEventSchema.parse(e)).toEqual(e);
  });

  it("rejects unknown type", () => {
    expect(() => StreamEventSchema.parse({ type: "unknown" })).toThrow();
  });
});
