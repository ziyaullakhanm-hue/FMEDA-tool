param(
    [string]$DatabaseUrl = $env:DATABASE_URL
)

if (-not $DatabaseUrl) {
    Write-Host "DATABASE_URL not set. Example: postgres://safeuser:safepass@localhost:5432/safecrate"
    exit 1
}

Write-Host "Running migrations and seed against: $DatabaseUrl"

# Apply engine migrations (adjust paths if you keep migrations elsewhere)
psql $DatabaseUrl -f engine/migrations/0001_init.sql
psql $DatabaseUrl -f engine/migrations/0002_add_fmed_tables.sql
psql $DatabaseUrl -f engine/migrations/0003_add_mission_profiles.sql
psql $DatabaseUrl -f engine/migrations/0004_add_component_hierarchy.sql

# Apply repo-level migrations (if any)
if (Test-Path migrations) {
    Get-ChildItem migrations -Filter *.sql | Sort-Object Name | ForEach-Object {
        Write-Host "Applying migration: $($_.FullName)"
        psql $DatabaseUrl -f $_.FullName
    }
}

# Load seed (example data)
if (Test-Path db/seed/example_data.sql) {
    Write-Host "Importing example seed: db/seed/example_data.sql"
    psql $DatabaseUrl -f db/seed/example_data.sql
} else {
    Write-Host "No seed file found at db/seed/example_data.sql"
}

Write-Host "Database import complete."
