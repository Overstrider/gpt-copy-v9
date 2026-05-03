# TASK-005: Implement message CRUD API

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 5
Parallel Group: backend-api
Owner Role: backend
Depends On: TASK-003, TASK-004

## Objective
Expose validated JSON endpoints for loading, sending, and deleting persisted conversation messages.

## Context
- Message loading and message sending are required.
- Messages must persist in SQLite.
- This non-streamed send path supports deterministic CRUD tests and fallback UI behavior.

## Write Scope
- backend/src/routes/messages.rs
- backend/src/validation.rs
- backend/src/models.rs
- backend/tests/api_messages.rs

## Acceptance Criteria
- GET /api/conversations/{conversation_id}/messages returns ordered persisted messages.
- POST /api/conversations/{conversation_id}/messages validates content and persists a user message without calling OpenRouter.
- DELETE /api/conversations/{conversation_id}/messages/{message_id} deletes a message when it belongs to the conversation.
- Missing conversations and invalid message IDs return structured JSON errors.
- Integration tests cover create conversation, send message, list messages, and delete message.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo test api_messages

## Risk Notes
- Do not call OpenRouter from CRUD-only message routes.

