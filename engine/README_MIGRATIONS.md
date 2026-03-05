**Engine Migrations — Quick Guide**

This document explains how to apply the SQL migration files in `engine/migrations`.

Prerequisites
- PostgreSQL server accessible
- `psql` client installed and available on PATH
- `DATABASE_URL` environment variable set to a valid Postgres connection URI, for example:

  postgres://postgres:pass@localhost:5432/safecrate

Apply migrations (Linux / macOS / WSL)
```bash
cd engine
export DATABASE_URL='postgres://postgres:pass@localhost:5432/safecrate'
./scripts/run_migrations.sh
```

Apply migrations (Windows PowerShell)
```powershell
cd engine
$env:DATABASE_URL = 'postgres://postgres:pass@localhost:5432/safecrate'
.\scripts\run_migrations.ps1
```

Notes
- Migration scripts are idempotent and ordered by filename.
- `0004_add_component_hierarchy.sql` creates `component_types`, `component_subtypes`, and `component_variants` tables — ensure it runs before any migration that references these tables.
- After applying migrations you may want to seed variant data (see `0005_update_variants.sql`) or restore `db/dump.sql` for a full snapshot.

Troubleshooting
- If `psql` reports permission errors, check database user and password in `DATABASE_URL`.
- To apply a single migration manually: `psql "$DATABASE_URL" -f engine/migrations/0006_update_components.sql`
