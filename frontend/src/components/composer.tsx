"use client";

import { KeyboardEvent, useRef, useState } from "react";
import { SendHorizonal, Square } from "lucide-react";

interface ComposerProps {
  onSubmit: (content: string) => void;
  onAbort?: () => void;
  isStreaming: boolean;
  disabled?: boolean;
}

export function Composer({ onSubmit, onAbort, isStreaming, disabled }: ComposerProps) {
  const [value, setValue] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const handleSubmit = () => {
    const trimmed = value.trim();
    if (!trimmed || isStreaming || disabled) return;
    onSubmit(trimmed);
    setValue("");
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  const handleInput = () => {
    const el = textareaRef.current;
    if (!el) return;
    el.style.height = "auto";
    el.style.height = `${Math.min(el.scrollHeight, 200)}px`;
  };

  return (
    <div
      className="border-t border-gray-700 bg-gray-900 p-3"
      data-testid="composer"
    >
      <div className="flex items-end gap-2 max-w-4xl mx-auto">
        <textarea
          ref={textareaRef}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={handleKeyDown}
          onInput={handleInput}
          rows={1}
          placeholder={isStreaming ? "Waiting for response…" : "Send a message…"}
          disabled={disabled}
          className="flex-1 resize-none rounded-xl bg-gray-800 px-4 py-2.5 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:opacity-50 min-h-[40px] max-h-[200px]"
          data-testid="composer-textarea"
        />
        {isStreaming ? (
          <button
            onClick={onAbort}
            className="p-2.5 rounded-xl bg-red-600 hover:bg-red-700 text-white transition flex-shrink-0"
            aria-label="Stop streaming"
            data-testid="composer-stop"
          >
            <Square size={16} />
          </button>
        ) : (
          <button
            onClick={handleSubmit}
            disabled={!value.trim() || disabled}
            className="p-2.5 rounded-xl bg-blue-600 hover:bg-blue-700 text-white transition flex-shrink-0 disabled:opacity-40 disabled:cursor-not-allowed"
            aria-label="Send message"
            data-testid="composer-send"
          >
            <SendHorizonal size={16} />
          </button>
        )}
      </div>
    </div>
  );
}
