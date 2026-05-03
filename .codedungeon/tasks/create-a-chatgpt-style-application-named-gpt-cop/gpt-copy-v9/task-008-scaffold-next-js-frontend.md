# TASK-008: Scaffold Next.js frontend

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 28648a0dde6e0b620aef7c580fdb0ef0cac21c5ea397a79845b8d42f39d90150
PROJECT_RULES_READ: yes

Repo: gpt-copy-v9
Kind: dev
Wave: 2
Parallel Group: frontend-foundation
Owner Role: frontend
Depends On: TASK-001

## Objective
Create the App Router TypeScript Tailwind app with required dependencies and root providers.

## Context
- Frontend must use Next.js App Router, TypeScript, and Tailwind CSS.
- Required libraries include TanStack Query, zod, lucide-react, react-markdown, and remark-gfm.
- The first screen must be the usable chat app, not a landing page.

## Write Scope
- frontend/package.json
- frontend/next.config.ts
- frontend/tsconfig.json
- frontend/tailwind.config.ts
- frontend/postcss.config.js
- frontend/app/layout.tsx
- frontend/app/page.tsx
- frontend/app/globals.css
- frontend/src/providers/query-provider.tsx

## Acceptance Criteria
- Next.js App Router TypeScript app installs successfully.
- Tailwind CSS loads globally.
- TanStack Query provider wraps the app.
- package.json declares zod, lucide-react, react-markdown, remark-gfm, and test tooling scripts.
- The default page renders a chat application shell placeholder.

## Verification Commands
- Set-Location frontend; npm install
- Set-Location frontend; npm run lint
- Set-Location frontend; npm run build

## Risk Notes
- Do not add frontend environment variables for OpenRouter credentials.

