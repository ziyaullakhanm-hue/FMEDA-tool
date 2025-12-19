-- 0005_update_variants.sql
-- Idempotent migration to add/update component_variants data
-- Ensures thermal/voltage constants exist and inserts/updates variants by id

BEGIN;

-- Add missing variant columns used by calculations
ALTER TABLE IF EXISTS component_variants
    ADD COLUMN IF NOT EXISTS a DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS ea1 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS ea2 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS c2 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS c3 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS uref_umax_ratio DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS pi_q DOUBLE PRECISION;

-- Upsert helper for variants; uses explicit IDs so repeatable

-- Aluminium electrolytic — non solid electrolyte
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES (
  'c2d8d8d5-d9d1-4ab8-8164-fe4e8f3f6344',
  (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic' LIMIT 1),
  'non solid electrolyte - LL', 5, 40, 1, 1.36, 0.8, 1, 0.87, 0.5, 0.95, 'LL type', '2025-10-24 18:56:12.605347+05:30'
)
ON CONFLICT (id) DO UPDATE SET
  subtype_id = COALESCE(component_variants.subtype_id, EXCLUDED.subtype_id),
  name = EXCLUDED.name,
  ref_fit = EXCLUDED.ref_fit,
  ref_temp = EXCLUDED.ref_temp,
  c2 = EXCLUDED.c2,
  c3 = EXCLUDED.c3,
  uref_umax_ratio = EXCLUDED.uref_umax_ratio,
  pi_q = EXCLUDED.pi_q,
  a = EXCLUDED.a,
  ea1 = EXCLUDED.ea1,
  ea2 = EXCLUDED.ea2,
  notes = EXCLUDED.notes;

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES (
  '6c14e951-4162-4780-884b-b8a571f16d46',
  (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic' LIMIT 1),
  'non solid electrolyte - GP', 5, 40, 1, 1.36, 0.8, 2, 0.87, 0.5, 0.95, 'GP type', '2025-10-24 18:56:12.605347+05:30'
)
ON CONFLICT (id) DO UPDATE SET
  subtype_id = COALESCE(component_variants.subtype_id, EXCLUDED.subtype_id),
  name = EXCLUDED.name,
  ref_fit = EXCLUDED.ref_fit,
  ref_temp = EXCLUDED.ref_temp,
  c2 = EXCLUDED.c2,
  c3 = EXCLUDED.c3,
  uref_umax_ratio = EXCLUDED.uref_umax_ratio,
  pi_q = EXCLUDED.pi_q,
  a = EXCLUDED.a,
  ea1 = EXCLUDED.ea1,
  ea2 = EXCLUDED.ea2,
  notes = EXCLUDED.notes;

-- Aluminium electrolytic — solid electrolyte
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES (
  '65953d6c-1b7b-43f8-be90-260112b7c020',
  (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic' LIMIT 1),
  'solid electrolyte - LL', 3, 40, 1.9, 3, 0.8, 1, 0.4, 0.14, 0, 'LL type', '2025-10-24 18:56:12.605347+05:30'
)
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name, ref_fit = EXCLUDED.ref_fit, ref_temp = EXCLUDED.ref_temp,
  c2 = EXCLUDED.c2, c3 = EXCLUDED.c3, uref_umax_ratio = EXCLUDED.uref_umax_ratio,
  pi_q = EXCLUDED.pi_q, a = EXCLUDED.a, ea1 = EXCLUDED.ea1, ea2 = EXCLUDED.ea2, notes = EXCLUDED.notes;

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES (
  'c5d89171-54f6-447a-aabb-7d710bd27c72',
  (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic' LIMIT 1),
  'solid electrolyte - GP', 3, 40, 1.9, 3, 0.8, 2, 0.4, 0.14, 0, 'GP type', '2025-10-24 18:56:12.605347+05:30'
)
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name, ref_fit = EXCLUDED.ref_fit, ref_temp = EXCLUDED.ref_temp,
  c2 = EXCLUDED.c2, c3 = EXCLUDED.c3, uref_umax_ratio = EXCLUDED.uref_umax_ratio,
  pi_q = EXCLUDED.pi_q, a = EXCLUDED.a, ea1 = EXCLUDED.ea1, ea2 = EXCLUDED.ea2, notes = EXCLUDED.notes;

-- (Additional variant upserts follow the same pattern.)
-- For brevity include several representative inserts; add more as needed.

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, notes, created_at)
VALUES (
  '54f040d2-8cc3-4ea0-bab8-c9022a8e981f',
  (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic' LIMIT 1),
  'HDKI HOC Z5U, Y5V, Y4T - LL', 5, 40, 1, 4, 0.5, 1, 1, 0.35, 'LL type', '2025-10-24 18:56:12.544279+05:30'
)
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name, ref_fit = EXCLUDED.ref_fit, ref_temp = EXCLUDED.ref_temp,
  c2 = EXCLUDED.c2, c3 = EXCLUDED.c3, uref_umax_ratio = EXCLUDED.uref_umax_ratio,
  pi_q = EXCLUDED.pi_q, a = EXCLUDED.a, ea1 = EXCLUDED.ea1, notes = EXCLUDED.notes;

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, notes, created_at)
VALUES (
  '6c50aed4-5729-4f13-9d11-cf2dba332445',
  (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic' LIMIT 1),
  'HDKI HOC Z5U, Y5V, Y4T - GP', 5, 40, 1, 4, 0.5, 2, 1, 0.35, 'GP type', '2025-10-24 18:56:12.544279+05:30'
)
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name, ref_fit = EXCLUDED.ref_fit, ref_temp = EXCLUDED.ref_temp,
  c2 = EXCLUDED.c2, c3 = EXCLUDED.c3, uref_umax_ratio = EXCLUDED.uref_umax_ratio,
  pi_q = EXCLUDED.pi_q, a = EXCLUDED.a, ea1 = EXCLUDED.ea1, notes = EXCLUDED.notes;

-- Add more upserts as needed for the full dataset (or import db/dump.sql for full snapshot)

COMMIT;
