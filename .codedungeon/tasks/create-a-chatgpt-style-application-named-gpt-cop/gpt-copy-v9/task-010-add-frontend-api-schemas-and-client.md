# TASK-010: Add frontend API schemas and client

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 7
Parallel Group: frontend-data
Owner Role: frontend
Depends On: TASK-004, TASK-005, TASK-007, TASK-008

## Objective
Implement Zod-validated API helpers and stream parsing for the backend contracts.

## Context
- API responses must be validated with zod.
- Frontend should talk only to the Rust backend.
- Stream parsing must match the backend event format.

## Write Scope
- frontend/src/lib/schemas.ts
- frontend/src/lib/api.ts
- frontend/src/lib/stream.ts
- frontend/src/hooks/use-conversations.ts
- frontend/src/hooks/use-messages.ts

## Acceptance Criteria
- Zod schemas validate conversation, message, list, and structured error envelopes.
- API helpers centralize base URL configuration, fetch behavior, error parsing, and response validation.
- Stream helper parses delta, done, and error events from the backend format.
- Hooks expose TanStack Query queries and mutations for conversations and messages.
- No frontend code accepts or references an OpenRouter API key.

## Verification Commands
- Set-Location frontend; npm run lint
- Set-Location frontend; npm run test

## Risk Notes
- Schema fixtures should mirror backend integration test responses to reduce contract drift.

