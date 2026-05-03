# TASK-012: Add streaming composer and markdown transcript

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 9
Parallel Group: frontend-streaming
Owner Role: frontend
Depends On: TASK-011

## Objective
Stream assistant responses into the transcript and render assistant markdown safely.

## Context
- Message streaming is required.
- Assistant markdown must use react-markdown and remark-gfm.
- The composer is the primary user workflow.

## Write Scope
- frontend/src/hooks/use-chat-stream.ts
- frontend/src/components/composer.tsx
- frontend/src/components/transcript.tsx
- frontend/src/components/message.tsx
- frontend/src/lib/stream.ts

## Acceptance Criteria
- Composer rejects empty prompts and disables duplicate submission during active streams.
- Submitting a prompt streams assistant deltas into the transcript.
- Completed assistant messages remain visible after query refresh.
- Assistant content renders with react-markdown and remark-gfm without raw HTML rendering.
- Stream errors are shown and do not erase existing conversation content.
- Keyboard submit and multiline input behave predictably.

## Verification Commands
- Set-Location frontend; npm run lint
- Set-Location frontend; npm run test
- Set-Location frontend; npm run build

## Risk Notes
- Do not use dangerouslySetInnerHTML or raw HTML markdown plugins for assistant content.

