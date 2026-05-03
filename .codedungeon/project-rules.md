# Project Rules - gpt-copy-v9

Status: APPROVED

## Sources Reviewed

- README.md
- CLAUDE.md
- prompts/full-v9.txt
- .gitignore

## Architecture And Boundaries

- MUST keep the repository as a monorepo with `backend/` for the Rust API and `frontend/` for the Next.js app.
- MUST implement the backend with Rust 2024, Axum, SQLite, and sqlx.
- MUST implement the frontend with Next.js App Router, TypeScript, and Tailwind CSS.
- MUST keep OpenRouter calls on the server side only.

## Project Rules

- MUST provide `GET /health`, conversation listing, conversation creation, message loading, message sending, and message streaming endpoints.
- MUST persist conversations and messages in SQLite.
- MUST validate request payloads and return structured JSON errors.
- MUST build the chat UI with sidebar conversations, transcript, composer, loading state, error state, and mobile behavior.
- MUST use `lucide-react` for common frontend icons.
- MUST validate API responses with `zod`.
- MUST render assistant markdown safely with `react-markdown` and `remark-gfm`.

## Commands And Verification

- VERIFY backend formatting, build, and tests before finalization.
- VERIFY frontend linting, build, component tests, and the Playwright smoke test before finalization.
- VERIFY the root README lists exact setup, environment, backend, frontend, test, and troubleshooting commands.
- VERIFY CodeDungeon final status only reports COMPLETE when `Verification: PASS` evidence exists.

## Security And Data Rules

- MUST NOT commit real secrets or tokens.
- MUST keep `.env.example` limited to placeholder values.
- MUST keep `.env` and `.env.*` ignored, except `.env.example`.
- MUST default `OPENROUTER_MODEL` to `nvidia/nemotron-3-super-120b-a12b:free`.
- MUST require `OPENROUTER_API_KEY` at runtime without providing a tracked secret.

## Agent Operating Rules

- MUST use `.claude/bin/codedungeon` for CodeDungeon workflow commands in this project.
- MUST preserve the Project Rules envelope in plans, tasks, review reports, phase handoffs, and final reports.
- MUST refresh and compact Project Rules before finalization if generated source files make the source digest stale.
- MUST leave the final PR open for human review and merge.

## Open Questions

- None.
