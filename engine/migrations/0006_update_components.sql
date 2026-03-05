-- 0006_update_components.sql
-- Adds missing columns to `components` to match `engine::models::Component`

BEGIN;

ALTER TABLE components
  ADD COLUMN IF NOT EXISTS component_type TEXT,
  ADD COLUMN IF NOT EXISTS base_fit NUMERIC,
  ADD COLUMN IF NOT EXISTS quality_factor NUMERIC,
  ADD COLUMN IF NOT EXISTS resistor_type TEXT,
  ADD COLUMN IF NOT EXISTS subtype_id UUID REFERENCES component_subtypes(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS operating_voltage DOUBLE PRECISION,
  ADD COLUMN IF NOT EXISTS rated_voltage DOUBLE PRECISION;

COMMIT;

-- NOTE: This migration is idempotent. After applying, you may want to
-- backfill `component_type`, `subtype_id`, or `variant_id` for existing
-- rows using your application logic or SQL `UPDATE` statements.
