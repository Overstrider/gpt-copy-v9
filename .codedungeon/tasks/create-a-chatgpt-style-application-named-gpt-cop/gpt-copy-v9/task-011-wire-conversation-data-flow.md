# TASK-011: Wire conversation data flow

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 8
Parallel Group: frontend-integration
Owner Role: frontend
Depends On: TASK-009, TASK-010

## Objective
Connect the chat UI to backend conversation and message APIs using TanStack Query.

## Context
- Conversation list, creation, selection, message loading, and message sending must work from the UI.
- Backend validation errors must be surfaced.
- This task prepares the UI for streaming behavior.

## Write Scope
- frontend/src/hooks/use-conversations.ts
- frontend/src/hooks/use-messages.ts
- frontend/src/components/chat-shell.tsx
- frontend/src/components/sidebar.tsx
- frontend/src/components/transcript.tsx
- frontend/src/components/composer.tsx

## Acceptance Criteria
- Sidebar loads persisted conversations and highlights the active conversation.
- New chat creates a conversation and selects it.
- Selecting a conversation loads ordered persisted messages.
- Submitting a prompt can persist a user message through the backend CRUD message route.
- Invalid payload and network errors appear in the UI.

## Verification Commands
- Set-Location frontend; npm run lint
- Set-Location frontend; npm run test

## Risk Notes
- Coordinate query invalidation and optimistic updates to avoid duplicate rendered user messages.

