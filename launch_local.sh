#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

for tool in cargo pnpm; do
  command -v "$tool" >/dev/null || {
    echo "$tool was not found in PATH. Install it before starting the local servers."
    exit 1
  }
done

[ -f backend/.env ] || cp backend/.env.example backend/.env

if command -v pg_isready >/dev/null && pg_isready -q -h localhost -p 5432; then
  echo "Using the native Postgres on localhost:5432."
else
  command -v docker >/dev/null || {
    echo "No Postgres answers on localhost:5432 and docker was not found. Start Postgres or install docker."
    exit 1
  }
  docker-compose up -d --wait postgres
fi
(cd frontend && pnpm install --frozen-lockfile)

if command -v watchexec >/dev/null; then
  backend_cmd=(watchexec -r -w crates -w Cargo.toml -- cargo run -p osl_api)
else
  backend_cmd=(cargo run -p osl_api)
fi

trap 'trap - EXIT; kill 0' EXIT INT TERM

(cd backend && "${backend_cmd[@]}") &
(cd frontend && pnpm dev) &
wait
