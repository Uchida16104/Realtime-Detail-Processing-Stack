# Realtime Detail Processing Stack

This repository is a deployable monorepo scaffold for:

- Frontend: Vite + Vue + Tailwind CSS + HTMX + hyperscript + Alpine.js + Mermaid
- Backend: Rust (Axum)
- External processors: Python3, C, C++, Java, Zig, Mojo
- Deployment: Vercel for frontend, Render for backend
- Output artifacts: downloadable logs under `backend/logs`

The runtime path is designed so the application stays operational even when a secondary language toolchain is missing: Rust falls back to an internal implementation and returns a structured result instead of failing hard.

## Layout

- `frontend/`
- `backend/`
- `plugins/`
- `diagrams/`
- `render.yaml`
- `vercel.json`

## Local run

Backend:

```bash
cd backend
cargo run
```

Frontend:

```bash
cd frontend
npm install
npm run dev
```

Set `VITE_BACKEND_URL=http://localhost:8080` for local development.

## Deployment model

Frontend goes to Vercel. Backend goes to Render as a Docker web service. That split matches Vercel's project/environment model and Render's support for Dockerfile-based services and environment variables. citeturn583868search0turn583868search1turn583868search3turn583868search5turn583868search8

## Notes

- "Realtime" is implemented with SSE.
- Network, file, and security processing are normalized into a single event stream.
- The Mermaid diagram is rendered client-side.
