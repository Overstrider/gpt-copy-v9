# TASK-016: Complete README and environment docs

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 12
Parallel Group: docs-final
Owner Role: docs
Depends On: TASK-013, TASK-015

## Objective
Document exact setup, environment, backend, frontend, test, and troubleshooting commands.

## Context
- README must list exact setup, environment, backend, frontend, test, and troubleshooting commands.
- .env.example must remain placeholder-only.
- OpenRouter API key must be required at runtime without providing a tracked secret.

## Write Scope
- README.md
- .env.example
- .gitignore

## Acceptance Criteria
- README names gpt-copy-v9 and explains backend and frontend responsibilities.
- README lists exact setup, environment, backend run/test, frontend install/dev/lint/build/test/e2e, and troubleshooting commands.
- .env.example values are placeholders only and include the required OpenRouter model default.
- .env and .env.* are ignored except .env.example.
- README explains that automated tests use mocked or deterministic OpenRouter behavior and live OpenRouter requires a local OPENROUTER_API_KEY.

## Verification Commands
- Get-Content README.md
- Get-Content .env.example
- git check-ignore .env .env.local

## Risk Notes
- Keep README commands synchronized with actual package scripts and Cargo commands.

