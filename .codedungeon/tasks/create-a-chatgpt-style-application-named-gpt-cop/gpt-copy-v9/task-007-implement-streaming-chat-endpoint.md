# TASK-007: Implement streaming chat endpoint

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 6
Parallel Group: backend-streaming
Owner Role: backend
Depends On: TASK-005, TASK-006

## Objective
Stream assistant output from OpenRouter through the backend while persisting user and assistant messages consistently.

## Context
- Message streaming endpoint is required.
- Frontend needs incremental assistant deltas, completion, and error events.
- Persistence must remain consistent on upstream failure.

## Write Scope
- backend/src/routes/chat.rs
- backend/src/openrouter.rs
- backend/src/repository.rs
- backend/tests/streaming.rs

## Acceptance Criteria
- POST /api/conversations/{conversation_id}/stream validates non-empty user content and conversation existence.
- Endpoint emits documented delta, done, and error events using one stable stream format.
- The user message is persisted once before the model call.
- The assistant message is persisted exactly once after a successful completed stream.
- Streaming tests use fake OpenRouter events and verify event order plus persisted database state.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo test streaming

## Risk Notes
- Buffer assistant deltas server-side and commit one final assistant message to avoid duplicate rows.

