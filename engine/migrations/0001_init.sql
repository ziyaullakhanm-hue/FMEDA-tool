-- ensure extensions for UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- define custom type for failure rate
CREATE DOMAIN fit AS NUMERIC;

-- projects table
CREATE TABLE IF NOT EXISTS projects (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  description TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- components table (BOM items)
CREATE TABLE IF NOT EXISTS components (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  manufacturer_part_number TEXT NOT NULL,
  manufacturer TEXT,
  reference_designator TEXT,
  quantity INT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- failure_modes table
CREATE TABLE IF NOT EXISTS failure_modes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  mpn TEXT,
  family TEXT,
  mode TEXT NOT NULL,
  lambda fit NOT NULL,        -- use the custom type defined above
  detection_coverage REAL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- calculations table
CREATE TABLE IF NOT EXISTS calculations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  payload JSONB NOT NULL,
  result JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- indices
CREATE INDEX IF NOT EXISTS idx_components_project ON components(project_id);
CREATE INDEX IF NOT EXISTS idx_failure_modes_mpn ON failure_modes(mpn);
CREATE INDEX IF NOT EXISTS idx_failure_modes_family ON failure_modes(family);
