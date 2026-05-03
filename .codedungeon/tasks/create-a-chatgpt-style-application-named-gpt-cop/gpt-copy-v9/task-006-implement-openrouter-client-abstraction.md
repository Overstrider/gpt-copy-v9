# TASK-006: Implement OpenRouter client abstraction

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 4
Parallel Group: backend-api
Owner Role: backend
Depends On: TASK-002

## Objective
Create a backend-only OpenRouter integration that can be replaced with deterministic fakes in tests.

## Context
- OpenRouter calls must stay server-side only.
- OPENROUTER_API_KEY is required at runtime.
- Automated tests must not require live provider credentials.

## Write Scope
- backend/src/openrouter.rs
- backend/src/config.rs
- backend/tests/openrouter.rs

## Acceptance Criteria
- OpenRouter requests include model, messages, authorization, and JSON headers.
- OPENROUTER_MODEL defaults to nvidia/nemotron-3-super-120b-a12b:free when unset.
- Provider errors map to structured backend errors without leaking secrets.
- The client is injectable or trait-backed so route tests can use deterministic fake responses.
- No frontend file references OPENROUTER_API_KEY.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo test openrouter

## Risk Notes
- Do not log authorization headers or upstream details that may contain sensitive information.

