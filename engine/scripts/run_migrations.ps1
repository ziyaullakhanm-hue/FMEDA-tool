<#
Simple PowerShell migration runner for engine/migrations.
Requires: `psql` in PATH and environment variable `DATABASE_URL` set.
#>

if (-not $env:DATABASE_URL) {
    Write-Error "DATABASE_URL is not set. Example: 'postgres://postgres:pass@localhost:5432/safecrate'"
    exit 2
}

if (-not (Get-Command psql -ErrorAction SilentlyContinue)) {
    Write-Error "psql not found in PATH. Install PostgreSQL client tools."
    exit 3
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$migDir = Join-Path $scriptDir "..\migrations"

Write-Host "Applying migrations from: $migDir"

$files = Get-ChildItem -Path $migDir -Filter *.sql | Sort-Object Name
if ($files.Count -eq 0) {
    Write-Host "No migration files found in $migDir"
    exit 0
}

foreach ($f in $files) {
    Write-Host "--- Applying $($f.Name) ---"
    & psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -f $f.FullName
    if ($LASTEXITCODE -ne 0) {
        Write-Error "psql failed applying $($f.Name)"
        exit $LASTEXITCODE
    }
}

Write-Host "All migrations applied."
