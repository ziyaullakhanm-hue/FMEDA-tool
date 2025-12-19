# React + TypeScript + Vite

This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh

## React Compiler

The React Compiler is currently not compatible with SWC. See [this issue](https://github.com/vitejs/vite-plugin-react/issues/428) for tracking the progress.

## Expanding the ESLint configuration

If you are developing a production application, we recommend updating the configuration to enable type-aware lint rules:

```js
export default defineConfig([
  globalIgnores(['dist']),
  {
    *** Begin README

    # SafeCrate — Setup & Run

    This repository contains a Vite + React frontend and a Rust backend (engine) that performs FMEDA/FIT calculations (SN29500). The instructions below walk through cloning the repo, installing prerequisites, setting up the Postgres database, running migrations, starting the backend, and starting the frontend on Windows (PowerShell).

    ## Prerequisites
    - Git
    - Rust toolchain (rustup + cargo). Recommended: stable toolchain. https://rustup.rs/
    - Node.js (LTS) and `npm` (or Yarn). Recommended: Node 18+.
    - PostgreSQL server and `psql` client.
    - (Optional) `sqlx-cli` if you prefer sqlx migrations automation.

    ## 1) Clone the repository

    ```powershell
    git clone https://github.com/<your-org>/SafeCrate.git
    cd SafeCrate
    ```

    ## 2) Database setup

    1. Create a Postgres database and a user (example):

    ```powershell
    # run in PowerShell (replace names/passwords)
    psql -U postgres -c "CREATE DATABASE safecrate;"
    psql -U postgres -c "CREATE USER safeuser WITH PASSWORD 'safepass';"
    psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE safecrate TO safeuser;"
    ```

    2. Apply SQL migrations. The repo contains SQL under `db/migrations/` (and `migrations/`). You can apply them with `psql`:

    ```powershell
    # from repo root
    psql "postgres://safeuser:safepass@localhost:5432/safecrate" -f db/migrations/0001_init.sql
    psql "postgres://safeuser:safepass@localhost:5432/safecrate" -f db/migrations/0002_add_fmed_tables.sql
    psql "postgres://safeuser:safepass@localhost:5432/safecrate" -f db/migrations/0003_add_mission_profiles.sql
    psql "postgres://safeuser:safepass@localhost:5432/safecrate" -f migrations/0004_add_component_hierarchy.sql

    Alternatively, run the provided import script which applies migrations and loads the example seed:

    ```powershell
    # set the DATABASE_URL environment variable first (example)
    $env:DATABASE_URL = 'postgres://safeuser:safepass@localhost:5432/safecrate'
    # from repository root
    .\scripts\import-db.ps1
    ```

    Or import the single-file SQL dump directly:

    ```powershell
    psql "postgres://safeuser:safepass@localhost:5432/safecrate" -f db/dump.sql
    ```

    Or start a reproducible local Postgres with Docker Compose (recommended for one-command onboarding):

    ```powershell
    # from repository root
    docker compose up -d

    # check logs while initialization runs
    docker compose logs -f db

    # to stop and remove container (keep volume):
    docker compose down

    # to remove container and data volume (start fresh next time):
    docker compose down -v
    ```

    The `docker-compose.yml` starts Postgres with the user `safeuser` / `safepass` and DB `safecrate`, and mounts `db/dump.sql` into `/docker-entrypoint-initdb.d/` so the dump is applied automatically on first container start.

## Updating the database schema or data (how to share DB changes with cloners)

When you change the local database (schema or data) and want collaborators who clone the repository to receive those changes, follow one of these reproducible approaches.

- Preferred: create a new migration under `engine/migrations/` (increment the sequence, e.g. `0006_my_change.sql`). Make the migration idempotent so it can be applied multiple times safely. Example pattern:

    ```sql
    -- engine/migrations/0006_update_some_variants.sql
    BEGIN;

    ALTER TABLE IF EXISTS component_variants ADD COLUMN IF NOT EXISTS some_col DOUBLE PRECISION;

    INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, a, ea1, ea2)
    VALUES ('...-uuid', (SELECT id FROM component_subtypes WHERE name='...'), 'variant name', 1.0, 40, 0.9, 0.5, 0.7)
    ON CONFLICT (id) DO UPDATE SET
      ref_fit = EXCLUDED.ref_fit,
      a = EXCLUDED.a,
      ea1 = EXCLUDED.ea1,
      ea2 = EXCLUDED.ea2;

    COMMIT;
    ```

    - Commit the new migration to Git and push. Collaborators who run migrations (or the import script) will pick it up.

- Snapshot / single-file dump (useful for full dataset snapshots):
  - Export locally with `pg_dump` and commit `db/dump.sql` or `db/dump.custom` (binary). Example:

    ```powershell
    # plain SQL dump (committed as db/dump.sql)
    pg_dump -U safeuser -d safecrate -f db/dump.sql

    # recommended: custom format (smaller) - commit only if you want to track binary file
    pg_dump -U safeuser -d safecrate -Fc -f db/dump.custom
    ```

  - Note: Docker `docker-entrypoint-initdb.d` runs init scripts only when the DB volume is empty. A cloner using Docker Compose must remove the `safecrate_pgdata` volume (`docker compose down -v`) to force reinitialization and import the updated dump.

- Import script: we provide `scripts/import-db.ps1` which applies migrations and the example seed. Encourage collaborators to run this after pulling new migrations.

Practical collaborator steps (after you commit & push migrations / dump):

1. Pull the repository updates:

    ```powershell
    git pull origin main
    ```

2. If you use migrations (recommended), run:

    ```powershell
    $env:DATABASE_URL = 'postgres://safeuser:safepass@localhost:5432/safecrate'
    .\scripts\import-db.ps1
    ```

   Or apply a single migration file directly:

    ```powershell
    psql $env:DATABASE_URL -f engine/migrations/0006_update_some_variants.sql
    ```

3. If you rely on the single-file dump, either run `psql -f db/dump.sql` or (if using Docker) recreate the DB volume so the init script runs:

    ```powershell
    docker compose down -v
    docker compose up -d
    ```

Verification commands

```powershell
# list recent migrations (files on disk)
ls engine\migrations | Sort-Object Name

# confirm variant rows exist in DB
psql $env:DATABASE_URL -c "SELECT id, name, a, ea1, ea2, c2, c3, uref_umax_ratio FROM component_variants ORDER BY created_at DESC LIMIT 20;"
```

Best practices
- Use idempotent migrations: `INSERT ... ON CONFLICT` or guarded `UPDATE` so migration re-run is safe.
- Prefer small, focused migrations for auditability rather than replacing the dump every time.
- If you must change the seed/dump, update `db/dump.sql` and also add a migration that brings existing DBs in sync for developers who don't want to reinit volumes.
- Document any breaking schema changes in a migration README note so collaborators can prepare.

If you want, I can:
- Add a migration template file `engine/migrations/TEMPLATE_idempotent.sql` (I can add it now), and/or
- Add `scripts/export-db.ps1` to produce `db/component_variants_data.sql` or `db/dump.sql` automatically.
    ```

    Adjust filenames/order if you have more migrations. If you use `sqlx-cli`, follow its workflow (note: ensure `DATABASE_URL` env var is set when running sqlx).

    ## 3) Backend (Rust engine)

    1. Configure `DATABASE_URL` in PowerShell (example):

    ```powershell
    $env:DATABASE_URL = 'postgres://safeuser:safepass@localhost:5432/safecrate'
    ```

    2. Build and run the backend (engine):

    ```powershell
    cd engine
    cargo build
    cargo run
    ```

    Notes:
    - The backend uses `sqlx` to access the DB. If `DATABASE_URL` is not set or the DB is unreachable, the program will run a local demo calculation (useful for validating FIT math without DB access).
    - You can run `cargo run --release` for a release build.

    ## 4) Frontend (Vite + React)

    From the repo root:

    1. Install dependencies:

    ```powershell
    npm install
    ```

    2. Start the dev server (Vite):

    ```powershell
    npm run dev
    ```

    The frontend dev server runs by default at `http://localhost:5173/`. If the frontend needs to talk to the backend API, edit the frontend config or use environment variables (e.g., set `VITE_API_URL`) as appropriate.

    To build the frontend for production:

    ```powershell
    npm run build
    npm run preview
    ```

    ## 5) Running end-to-end locally

    1. Ensure Postgres is running and migrations applied.
    2. Set `DATABASE_URL` as shown above.
    3. Start backend in one terminal (PowerShell):

    ```powershell
    cd engine
    cargo run
    ```

    4. Start frontend in another terminal:

    ```powershell
    npm run dev
    ```

    Open `http://localhost:5173/` in your browser and use the frontend; backend interactive CLI can also be used directly by running the engine.

    ## 6) Debugging & verification tips
    - If the backend fails with `No such host is known.`, check `DATABASE_URL` hostname and DNS reachability (use `nslookup`, `Test-NetConnection -Port 5432`).
    - The engine prints detailed calculation debug lines for SN29500 (per-segment PiT, PiU details, and final formula). Use those to verify manual calculations.
    - If a variant's constants differ from your expected values, update `component_variants` table (see README_VERIFY_FIT.md for example SQL).

    ## 7) Running tests / building CI
    - There are currently no unit tests included for FIT calculations. You can add tests under `engine/tests/` and run `cargo test`.

    ## 8) Contributing
    - Create branches for features/fixes, add tests for core calculation functions, and open PRs back to the main repo.

    ---

    If you want I can also:
    - Add a `Makefile` or PowerShell script that runs the full setup (DB + migrations + backend + frontend) for dev convenience.
    - Add a small unit test in `engine` that reproduces the sample capacitor/resistor FIT check and asserts values.

    Let me know which you'd like next.

    *** End README
