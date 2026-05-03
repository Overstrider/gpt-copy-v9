# TASK-001: Create monorepo foundation

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 1
Parallel Group: foundation
Owner Role: docs
Depends On: -

## Objective
Establish the root monorepo layout, environment hygiene, and placeholder documentation.

## Context
- Repository must remain a monorepo with backend/ and frontend/.
- Tracked environment examples must contain placeholders only.
- OPENROUTER_MODEL must default to nvidia/nemotron-3-super-120b-a12b:free.

## Write Scope
- .gitignore
- .env.example
- README.md
- backend/
- frontend/

## Acceptance Criteria
- backend/ and frontend/ directories exist as the Rust API and Next.js app workspaces.
- .gitignore ignores .env and .env.* while allowing .env.example.
- .env.example includes placeholder OPENROUTER_API_KEY and OPENROUTER_MODEL=nvidia/nemotron-3-super-120b-a12b:free.
- README.md identifies the project as gpt-copy-v9 and includes initial setup, environment, backend, frontend, test, and troubleshooting headings.

## Verification Commands
- Get-ChildItem -Force
- Get-Content .gitignore
- Get-Content .env.example
- Get-Content README.md

## Risk Notes
- Do not add real tokens, local SQLite database files, dependency folders, or generated build output.

