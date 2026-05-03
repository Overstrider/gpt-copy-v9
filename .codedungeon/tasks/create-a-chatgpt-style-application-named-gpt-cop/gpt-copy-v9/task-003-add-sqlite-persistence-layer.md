# TASK-003: Add SQLite persistence layer

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 3
Parallel Group: backend-data
Owner Role: backend
Depends On: TASK-002

## Objective
Implement sqlx migrations, database setup, models, and repository methods for conversations and messages.

## Context
- Conversations and messages must persist in SQLite.
- Transcript ordering must be stable.
- Backend tests need isolated temporary databases.

## Write Scope
- backend/migrations/
- backend/src/db.rs
- backend/src/models.rs
- backend/src/repository.rs
- backend/tests/support.rs
- backend/tests/repository.rs

## Acceptance Criteria
- Migrations create conversations and messages tables with stable IDs, roles, content, timestamps, and foreign keys.
- Database initialization runs migrations at startup.
- Repository functions create, list, update, and delete conversations.
- Repository functions insert, list, and delete messages for a conversation in chronological order.
- Repository tests cover persistence behavior using isolated SQLite databases.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo test repository

## Risk Notes
- Use sqlx-supported SQLite types consistently and avoid a shared test database file.

