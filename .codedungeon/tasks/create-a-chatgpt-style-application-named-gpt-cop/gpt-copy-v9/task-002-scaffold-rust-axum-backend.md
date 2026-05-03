# TASK-002: Scaffold Rust Axum backend

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 2
Parallel Group: backend-foundation
Owner Role: backend
Depends On: TASK-001

## Objective
Create the Rust 2024 Axum service shell with configuration, app state, CORS, structured error envelope, and GET /health.

## Context
- Backend must use Rust 2024, Axum, SQLite, and sqlx.
- GET /health is required.
- Validation errors and internal failures must return structured JSON errors.
- CORS is required for the frontend development origin.

## Write Scope
- backend/Cargo.toml
- backend/src/main.rs
- backend/src/lib.rs
- backend/src/config.rs
- backend/src/error.rs
- backend/src/state.rs
- backend/src/routes/mod.rs
- backend/src/routes/health.rs

## Acceptance Criteria
- Cargo.toml uses edition 2024 and declares axum, tokio, tower-http, serde, thiserror, tracing, sqlx with sqlite support, and an HTTP client dependency.
- GET /health returns HTTP 200 with a JSON status payload.
- App errors serialize as a stable JSON envelope with code and message fields.
- CORS allows the local Next.js development origin.
- Configuration requires OPENROUTER_API_KEY for runtime chat behavior and defaults OPENROUTER_MODEL to nvidia/nemotron-3-super-120b-a12b:free when unset.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo check

## Risk Notes
- Tests should be able to inject configuration without requiring a real OpenRouter key.

