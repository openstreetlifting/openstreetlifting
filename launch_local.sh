#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

for tool in docker cargo pnpm; do
  command -v "$tool" >/dev/null || {
    echo "$tool was not found in PATH. Install it before starting the local servers."
    exit 1
  }
done

if [ ! -f backend/.env ]; then
  echo "backend/.env is missing. Run: cp backend/.env.example backend/.env"
  exit 1
fi

docker-compose up -d --wait postgres

if command -v cargo-watch >/dev/null; then
  backend_cmd=(cargo watch -x "run -p osl_api")
else
  echo "Backend reload is disabled. To enable it, run: cargo install cargo-watch"
  backend_cmd=(cargo run -p osl_api)
fi

trap 'trap - EXIT; kill 0' EXIT INT TERM

(cd backend && "${backend_cmd[@]}") &
(cd frontend && pnpm dev) &
wait
