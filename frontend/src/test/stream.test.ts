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

  it("calls onError with the HTTP error response message", async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(JSON.stringify({ code: "OPENROUTER_ERROR", message: "Upstream failed" }), {
        status: 502,
        headers: { "Content-Type": "application/json" },
      })
    );
    vi.stubGlobal("fetch", fetchMock);

    const onDelta = vi.fn();
    const onDone = vi.fn();
    const onError = vi.fn();

    await streamChat("conv-1", "Hi", { onDelta, onDone, onError });

    expect(onError).toHaveBeenCalledWith("Upstream failed");
    expect(onDelta).not.toHaveBeenCalled();
    expect(onDone).not.toHaveBeenCalled();
  });

  it("calls onError for explicit SSE error events", async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(
        streamBody('data: {"type":"error","message":"upstream failed"}\n\n'),
        { status: 200 }
      )
    );
    vi.stubGlobal("fetch", fetchMock);

    const onDelta = vi.fn();
    const onDone = vi.fn();
    const onError = vi.fn();

    await streamChat("conv-1", "Hi", { onDelta, onDone, onError });

    expect(onError).toHaveBeenCalledWith("upstream failed");
    expect(onDelta).not.toHaveBeenCalled();
    expect(onDone).not.toHaveBeenCalled();
  });

  it("calls onDone for explicit SSE done events", async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(streamBody('data: {"type":"done"}\n\n'), { status: 200 })
    );
    vi.stubGlobal("fetch", fetchMock);

    const onDelta = vi.fn();
    const onDone = vi.fn();
    const onError = vi.fn();

    await streamChat("conv-1", "Hi", { onDelta, onDone, onError });

    expect(onDone).toHaveBeenCalledOnce();
    expect(onDelta).not.toHaveBeenCalled();
    expect(onError).not.toHaveBeenCalled();
  });

  it("cancels the stream reader after a terminal event", async () => {
    const cancel = vi.fn();
    const body = new ReadableStream<Uint8Array>({
      start(controller) {
        controller.enqueue(
          new TextEncoder().encode('data: {"type":"done"}\n\n')
        );
      },
      cancel,
    });
    const fetchMock = vi
      .fn()
      .mockResolvedValue(new Response(body, { status: 200 }));
    vi.stubGlobal("fetch", fetchMock);

    await streamChat("conv-1", "Hi", {
      onDelta: vi.fn(),
      onDone: vi.fn(),
      onError: vi.fn(),
    });

    expect(cancel).toHaveBeenCalledOnce();
  });
});
