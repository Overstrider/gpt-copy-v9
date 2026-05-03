# TASK-013: Harden backend verification

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: test
Wave: 10
Parallel Group: qa-backend
Owner Role: qa
Depends On: TASK-007

## Objective
Ensure backend formatting, build, and tests pass for health, CRUD, validation, CORS, OpenRouter mapping, and streaming.

## Context
- Backend finalization requires formatting, build, and tests.
- Tests must not require real OpenRouter credentials.
- Validation and structured errors are frontend-visible contracts.

## Write Scope
- backend/src/
- backend/tests/
- backend/Cargo.toml

## Acceptance Criteria
- GET /health test passes.
- Conversation and message CRUD integration tests pass against isolated SQLite databases.
- Validation and missing-record tests assert structured JSON errors.
- CORS behavior is covered by a backend test.
- OpenRouter and streaming tests use deterministic fakes.
- cargo fmt, cargo build, and cargo test pass.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo build
- Set-Location backend; cargo test

## Risk Notes
- Do not weaken runtime API key requirements to make tests pass; inject test configuration instead.

