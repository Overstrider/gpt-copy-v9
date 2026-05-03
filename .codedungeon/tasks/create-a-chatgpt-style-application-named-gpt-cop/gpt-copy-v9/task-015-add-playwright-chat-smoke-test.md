# TASK-015: Add Playwright chat smoke test

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: test
Wave: 11
Parallel Group: qa-e2e
Owner Role: qa
Depends On: TASK-012, TASK-014

## Objective
Verify a deterministic chat path from app load through conversation creation, prompt submission, streamed assistant output, and mobile sidebar behavior.

## Context
- Frontend verification requires a Playwright smoke test.
- Smoke tests should avoid real OpenRouter calls.
- The first viewport should be the chat application shell.

## Write Scope
- frontend/playwright.config.ts
- frontend/e2e/chat-smoke.spec.ts
- frontend/package.json

## Acceptance Criteria
- Playwright opens the app and finds the chat shell, sidebar, transcript, and composer.
- The test creates or selects a conversation and sends a user prompt.
- The test observes deterministic assistant output from a mocked backend stream or deterministic test backend mode.
- The test verifies one mobile viewport sidebar interaction.
- npm scripts expose a test:e2e command that does not require OPENROUTER_API_KEY.

## Verification Commands
- Set-Location frontend; npm run test:e2e

## Risk Notes
- Use Playwright route interception or a deterministic backend test mode rather than live OpenRouter calls.

