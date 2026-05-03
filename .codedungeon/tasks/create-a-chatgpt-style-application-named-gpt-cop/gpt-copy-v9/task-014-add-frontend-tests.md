# TASK-014: Add frontend tests

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: test
Wave: 10
Parallel Group: qa-frontend
Owner Role: qa
Depends On: TASK-012

## Objective
Verify UI components, API schema validation, markdown rendering, and mobile behavior.

## Context
- Frontend verification requires linting, build, and component tests.
- UI behavior must include sidebar, transcript, composer, loading, error, and mobile states.
- API responses must be validated with zod.

## Write Scope
- frontend/vitest.config.ts
- frontend/src/test/
- frontend/src/**/*.test.ts
- frontend/src/**/*.test.tsx
- frontend/package.json

## Acceptance Criteria
- Component tests cover sidebar list, selection, loading, and error states.
- Component tests cover transcript empty, loading, markdown, and error states.
- Component tests cover composer empty-submit and pending-submit behavior.
- Zod schema tests reject malformed backend responses.
- npm scripts expose lint, build, and test commands.

## Verification Commands
- Set-Location frontend; npm run lint
- Set-Location frontend; npm run build
- Set-Location frontend; npm run test

## Risk Notes
- Use backend-like fixtures for schema tests to detect contract drift.

