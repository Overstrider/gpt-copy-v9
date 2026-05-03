import { StreamEvent, StreamEventSchema } from "./schemas";
import { API_BASE_URL } from "./config";

export type StreamCallbacks = {
  onDelta: (content: string) => void;
  onDone: () => void;
  onError: (message: string) => void;
};

export async function streamChat(
  conversationId: string,
  content: string,
  callbacks: StreamCallbacks,
  signal?: AbortSignal
): Promise<void> {
  const res = await fetch(
    `${API_BASE_URL}/api/conversations/${conversationId}/stream`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ content }),
      signal,
    }
  );

  if (!res.ok) {
    let msg = `HTTP ${res.status}`;
    try {
      const body = await res.json();
      if (body?.message) msg = body.message;
    } catch {
      // ignore
    }
    callbacks.onError(msg);
    return;
  }

  const reader = res.body?.getReader();
  if (!reader) {
    callbacks.onError("No response body");
    return;
  }

  const decoder = new TextDecoder();
  let buffer = "";
  let doneFired = false;
  const fireDone = () => {
    if (!doneFired) {
      doneFired = true;
      callbacks.onDone();
    }
  };

  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      fireDone();
      break;
    }
    buffer += decoder.decode(value, { stream: true });

    const lines = buffer.split("\n");
    buffer = lines.pop() ?? "";

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed.startsWith("data: ")) continue;
      const data = trimmed.slice(6);
      try {
        const parsed = JSON.parse(data) as unknown;
        const event = StreamEventSchema.safeParse(parsed);
        if (!event.success) continue;
        handleStreamEvent(event.data, { ...callbacks, onDone: fireDone });
        if (event.data.type === "done" || event.data.type === "error") {
          return;
        }
      } catch {
        // skip malformed
      }
    }
  }
}

function handleStreamEvent(event: StreamEvent, callbacks: StreamCallbacks) {
  switch (event.type) {
    case "delta":
      callbacks.onDelta(event.content);
      break;
    case "done":
      callbacks.onDone();
      break;
    case "error":
      callbacks.onError(event.message);
      break;
  }
}
