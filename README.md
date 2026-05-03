# gpt-copy-v9

A ChatGPT-style application built with Rust (Axum) backend and Next.js frontend.

## Architecture

- **backend/**: Rust Axum REST + streaming API with SQLite persistence
- **frontend/**: Next.js App Router TypeScript UI with TanStack Query and Tailwind CSS

## Initial Setup

```bash
# Clone the repository
git clone https://github.com/Overstrider/gpt-copy-v9.git
cd gpt-copy-v9

# Copy and configure environment
cp .env.example .env
# Edit .env with your OPENROUTER_API_KEY
```

## Environment

| Variable | Description | Default |
|----------|-------------|---------|
| `OPENROUTER_API_KEY` | Required for live chat. Automated tests use mocks. | — |
| `OPENROUTER_MODEL` | Model to use via OpenRouter | `nvidia/nemotron-3-super-120b-a12b:free` |
| `DATABASE_URL` | SQLite database path | `sqlite:./gpt-copy-v9.db` |

> **Note**: Automated tests use mocked or deterministic OpenRouter behavior. Live chat requires a local `OPENROUTER_API_KEY`.

## Backend

```bash
cd backend

# Check formatting
cargo fmt -- --check

# Build
cargo build

# Run (requires .env with OPENROUTER_API_KEY)
cargo run

# Run tests (no API key required)
cargo test
```

## Frontend

```bash
cd frontend

# Install dependencies
npm install

# Development server
npm run dev

# Lint
npm run lint

# Type-check + build
npm run build

# Unit/component tests
npm run test

# End-to-end smoke test (no API key required)
npm run test:e2e
```

## Testing

- Backend tests use isolated in-memory SQLite databases and fake OpenRouter responses.
- Frontend unit tests use Vitest + React Testing Library with mocked API calls.
- E2E tests use Playwright with route interception — no live OpenRouter calls.

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `OPENROUTER_API_KEY not set` | Copy `.env.example` to `.env` and add your key |
| `cargo: command not found` | Install Rust via `rustup.rs` |
| `npm: command not found` | Install Node.js 18+ |
| SQLite errors | Delete `*.db` files and restart the backend |
| CORS errors | Ensure backend is running on port 3001 and frontend on 3000 |
