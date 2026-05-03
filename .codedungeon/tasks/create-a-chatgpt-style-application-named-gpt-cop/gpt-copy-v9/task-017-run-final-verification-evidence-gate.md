# TASK-017: Run final verification evidence gate

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: test
Wave: 13
Parallel Group: final-verification
Owner Role: qa
Depends On: TASK-016

## Objective
Collect PASS evidence for all required backend, frontend, docs, secret hygiene, and CodeDungeon completion checks.

## Context
- CodeDungeon final status can report COMPLETE only with Verification: PASS evidence.
- Project rules digest must be preserved.
- No tracked secrets or frontend OpenRouter key exposure are allowed.

## Write Scope
- .codedungeon/
- README.md

## Acceptance Criteria
- Backend cargo fmt --check passes.
- Backend cargo build passes.
- Backend cargo test passes.
- Frontend npm run lint passes.
- Frontend npm run build passes.
- Frontend npm run test passes.
- Frontend npm run test:e2e passes.
- README command audit and secret hygiene checks pass.
- Final handoff records Verification: PASS with command evidence before COMPLETE is reported.

## Verification Commands
- Set-Location backend; cargo fmt -- --check
- Set-Location backend; cargo build
- Set-Location backend; cargo test
- Set-Location frontend; npm run lint
- Set-Location frontend; npm run build
- Set-Location frontend; npm run test
- Set-Location frontend; npm run test:e2e
- git check-ignore .env .env.local
- git grep -n "OPENROUTER_API_KEY" -- .
- git grep -n "nvidia/nemotron-3-super-120b-a12b:free" -- .
- Get-Content README.md

## Risk Notes
- Do not mark COMPLETE if any verification command fails, is skipped, or lacks captured evidence.

