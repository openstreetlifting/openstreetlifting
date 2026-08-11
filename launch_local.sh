#!/usr/bin/env bash
# Postgres runs in a container so it matches CI. The backend and frontend run
# natively, because a bind mounted target directory turns a 15 second
# incremental build into a cold one.
set -euo pipefail

cd "$(dirname "$0")"

for tool in docker cargo pnpm; do
  command -v "$tool" >/dev/null || { echo "$tool is required but not installed"; exit 1; }
done

if [ ! -f backend/.env ]; then
  echo "backend/.env is missing. Copy backend/.env.example and fill it in."
  exit 1
fi

# --wait blocks on the healthcheck, so migrations cannot race the database.
docker compose up -d --wait postgres

if command -v cargo-watch >/dev/null; then
  backend_cmd=(cargo watch -x "run -p osl_api")
else
  echo "cargo-watch not found, running without reload. cargo install cargo-watch"
  backend_cmd=(cargo run -p osl_api)
fi

# Kill the whole process group on exit so neither server is orphaned.
trap 'trap - EXIT; kill 0' EXIT INT TERM

(cd backend && "${backend_cmd[@]}") &
(cd frontend && pnpm dev) &
wait
