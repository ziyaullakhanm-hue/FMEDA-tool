#!/usr/bin/env bash
set -euo pipefail

# Simple migration runner for the engine migrations folder.
# Requires: `psql` in PATH and environment variable `DATABASE_URL` set

if [[ -z "${DATABASE_URL-}" ]]; then
  echo "ERROR: DATABASE_URL is not set. Example: postgres://postgres:pass@localhost:5432/safecrate"
  exit 2
fi

if ! command -v psql >/dev/null 2>&1; then
  echo "ERROR: psql not found in PATH. Install PostgreSQL client tools."
  exit 3
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MIG_DIR="$SCRIPT_DIR/../migrations"

echo "Applying migrations from: $MIG_DIR"

shopt -s nullglob
files=("$MIG_DIR"/*.sql)
if [ ${#files[@]} -eq 0 ]; then
  echo "No migration files found in $MIG_DIR"
  exit 0
fi

for f in "${files[@]}"; do
  echo "--- Applying $(basename "$f") ---"
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$f"
done

echo "All migrations applied."
