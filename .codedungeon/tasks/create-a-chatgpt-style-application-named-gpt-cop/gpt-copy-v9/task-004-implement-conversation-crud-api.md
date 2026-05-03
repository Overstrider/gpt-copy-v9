# TASK-004: Implement conversation CRUD API

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 4
Parallel Group: backend-api
Owner Role: backend
Depends On: TASK-003

## Objective
Expose validated JSON endpoints for creating, listing, updating, deleting, and reading conversations.

## Context
- Conversation listing and creation are required.
- Prompt asks for conversations CRUD.
- Frontend Zod schemas need stable response and error envelopes.

## Write Scope
- backend/src/routes/conversations.rs
- backend/src/validation.rs
- backend/src/models.rs
- backend/tests/api_conversations.rs

## Acceptance Criteria
- GET /api/conversations lists conversations ordered by latest activity.
- POST /api/conversations validates input and returns the created conversation.
- GET /api/conversations/{conversation_id} returns one conversation or a structured 404 error.
- PATCH /api/conversations/{conversation_id} updates supported metadata such as title.
- DELETE /api/conversations/{conversation_id} deletes the conversation and related messages.
- Invalid payloads return structured JSON validation errors.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo test api_conversations

## Risk Notes
- Keep path names and JSON fields stable for frontend schemas.

