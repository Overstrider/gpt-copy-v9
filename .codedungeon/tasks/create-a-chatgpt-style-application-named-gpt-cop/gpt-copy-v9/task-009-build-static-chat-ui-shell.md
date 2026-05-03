# TASK-009: Build static chat UI shell

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 3
Parallel Group: frontend-ui
Owner Role: frontend
Depends On: TASK-008

## Objective
Implement the responsive ChatGPT-style layout with sidebar, transcript, composer, loading states, and error states.

## Context
- UI must include sidebar conversations, transcript, composer, loading state, error state, and mobile behavior.
- lucide-react must be used for common icons.
- Layout must remain usable on desktop and mobile.

## Write Scope
- frontend/src/components/chat-shell.tsx
- frontend/src/components/sidebar.tsx
- frontend/src/components/transcript.tsx
- frontend/src/components/message.tsx
- frontend/src/components/composer.tsx
- frontend/src/components/empty-state.tsx
- frontend/app/page.tsx
- frontend/app/globals.css

## Acceptance Criteria
- Desktop layout shows conversation sidebar, transcript, and composer without overlap.
- Mobile layout has a usable sidebar toggle and preserves composer access.
- Conversation, transcript, and composer loading states are visible and stable.
- Error states are visible without clearing existing transcript content.
- Common actions use lucide-react icons.

## Verification Commands
- Set-Location frontend; npm run lint
- Set-Location frontend; npm run build

## Risk Notes
- Constrain heights and scroll regions so transcript content does not hide the composer.

