import { afterEach, describe, expect, it, vi } from "vitest";

import { streamChat } from "@/lib/stream";

function streamBody(data: string): ReadableStream<Uint8Array> {
  return new ReadableStream({
    start(controller) {
      controller.enqueue(new TextEncoder().encode(data));
      controller.close();
    },
  });
}

describe("streamChat", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("calls onDone when the response closes without a terminal event", async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(
        streamBody('data: {"type":"delta","content":"Hello"}\n\n'),
        { status: 200 }
      )
    );
    vi.stubGlobal("fetch", fetchMock);

    const onDelta = vi.fn();
    const onDone = vi.fn();
    const onError = vi.fn();

    await streamChat("conv-1", "Hi", { onDelta, onDone, onError });

    expect(onDelta).toHaveBeenCalledWith("Hello");
    expect(onDone).toHaveBeenCalledOnce();
    expect(onError).not.toHaveBeenCalled();
  });
});
