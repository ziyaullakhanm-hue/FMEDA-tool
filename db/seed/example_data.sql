-- Example seed data for component types, subtypes and variants
-- This file creates missing columns on `component_variants` (if not present)
-- and inserts a set of commonly-used capacitor/resistor variants used for
-- verification and demo runs.

BEGIN;

-- Ensure extended variant columns exist for calculations
ALTER TABLE IF EXISTS component_variants
    ADD COLUMN IF NOT EXISTS a DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS ea1 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS ea2 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS c2 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS c3 DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS uref_umax_ratio DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS pi_q DOUBLE PRECISION;

-- Insert top-level types (id generated if not present)
INSERT INTO component_types (name, description)
VALUES
  ('Resistor', 'Passive resistor components')
ON CONFLICT (name) DO NOTHING;

INSERT INTO component_types (name, description)
VALUES
  ('Capacitor', 'Passive capacitor components')
ON CONFLICT (name) DO NOTHING;

-- Insert a selection of subtypes (use the type id lookup)
INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Aluminium electrolytic', 'Aluminium electrolytic capacitors'
FROM component_types t
WHERE t.name = 'Capacitor'
ON CONFLICT DO NOTHING;

INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Ceramic', 'Ceramic capacitors'
FROM component_types t
WHERE t.name = 'Capacitor'
ON CONFLICT DO NOTHING;

INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Glass', 'Glass/dielectric capacitors'
FROM component_types t
WHERE t.name = 'Capacitor'
ON CONFLICT DO NOTHING;

INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Metal foil', 'Metal foil capacitors'
FROM component_types t
WHERE t.name = 'Capacitor'
ON CONFLICT DO NOTHING;

INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Mica', 'Mica capacitors'
FROM component_types t
WHERE t.name = 'Capacitor'
ON CONFLICT DO NOTHING;

INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Tantalum electrolytic', 'Tantalum electrolytic capacitors'
FROM component_types t
WHERE t.name = 'Capacitor'
ON CONFLICT DO NOTHING;

INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Thin Film', 'Precision resistors (low TCR)'
FROM component_types t
WHERE t.name = 'Resistor'
ON CONFLICT DO NOTHING;

INSERT INTO component_subtypes (type_id, name, description)
SELECT t.id, 'Thick Film', 'General-purpose resistors'
FROM component_types t
WHERE t.name = 'Resistor'
ON CONFLICT DO NOTHING;

-- Now insert variants (explicit IDs copied from example dataset)

-- Aluminium electrolytic — non solid electrolyte (LL / GP)
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('c2d8d8d5-d9d1-4ab8-8164-fe4e8f3f6344', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic'), 'non solid electrolyte - LL', 5, 40, 1, 1.36, 0.8, 1, 0.87, 0.5, 0.95, 'LL type', '2025-10-24 18:56:12.605347+05:30'),
  ('6c14e951-4162-4780-884b-b8a571f16d46', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic'), 'non solid electrolyte - GP', 5, 40, 1, 1.36, 0.8, 2, 0.87, 0.5, 0.95, 'GP type', '2025-10-24 18:56:12.605347+05:30')
ON CONFLICT (id) DO NOTHING;

-- Aluminium electrolytic — solid electrolyte
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('65953d6c-1b7b-43f8-be90-260112b7c020', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic'), 'solid electrolyte - LL', 3, 40, 1.9, 3, 0.8, 1, 0.4, 0.14, 0, 'LL type', '2025-10-24 18:56:12.605347+05:30'),
  ('c5d89171-54f6-447a-aabb-7d710bd27c72', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Aluminium electrolytic'), 'solid electrolyte - GP', 3, 40, 1.9, 3, 0.8, 2, 0.4, 0.14, 0, 'GP type', '2025-10-24 18:56:12.605347+05:30')
ON CONFLICT (id) DO NOTHING;

-- Ceramic — HDKI HOC Z5U, Y5V, Y4T
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, notes, created_at)
VALUES
  ('54f040d2-8cc3-4ea0-bab8-c9022a8e981f', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic'), 'HDKI HOC Z5U, Y5V, Y4T - LL', 5, 40, 1, 4, 0.5, 1, 1, 0.35, 'LL type', '2025-10-24 18:56:12.544279+05:30'),
  ('6c50aed4-5729-4f13-9d11-cf2dba332445', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic'), 'HDKI HOC Z5U, Y5V, Y4T - GP', 5, 40, 1, 4, 0.5, 2, 1, 0.35, 'GP type', '2025-10-24 18:56:12.544279+05:30')
ON CONFLICT (id) DO NOTHING;

-- Ceramic — MDKI MOC X7R, X5R
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, notes, created_at)
VALUES
  ('5d165db3-2b2b-44ca-a89f-9a6643300f4c', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic'), 'MDKI MOC X7R, X5R - LL', 2, 40, 1, 4, 0.5, 1, 1, 0.35, 'LL type', '2025-10-24 18:56:12.544279+05:30'),
  ('1b9b98d5-80a1-4ec4-8a50-09e9c5e06b5c', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic'), 'MDKI MOC X7R, X5R - GP', 2, 40, 1, 4, 0.5, 2, 1, 0.35, 'GP type', '2025-10-28 18:56:12.544279+05:30')
ON CONFLICT (id) DO NOTHING;

-- Ceramic — NDKI LOC COG, NPO
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, notes, created_at)
VALUES
  ('ab311e67-2880-43d7-81d7-441f69adbe75', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic'), 'NDKI LOC COG, NPO - LL', 1, 40, 1, 4, 0.5, 1, 1, 0.35, 'LL type', '2025-10-24 18:56:12.544279+05:30'),
  ('84cf4b7a-6a59-4c07-9167-569de9b2b439', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Ceramic'), 'NDKI LOC COG, NPO - GP', 1, 40, 1, 4, 0.5, 2, 1, 0.35, 'GP type', '2025-10-24 18:56:12.544279+05:30')
ON CONFLICT (id) DO NOTHING;

-- Glass — Mica
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('04a3f04e-58d5-4453-8d50-7323628ea8a4', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Glass'), 'Mica - LL', 2, 40, 1.11, 4.33, 0.5, 1, 0.86, 0.27, 0.84, 'LL type', '2025-12-05 14:21:12.619631+05:30'),
  ('f366d5bd-bea2-4bad-9243-6aa5358d01bb', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Glass'), 'Mica - GP', 2, 40, 1.11, 4.33, 0.5, 2, 0.86, 0.27, 0.84, 'GP type', '2025-12-05 14:21:12.619631+05:30')
ON CONFLICT (id) DO NOTHING;

-- Metal foil, Polycarbonate
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('7de86a2a-47b1-4be5-bbb3-0e59b01cac47', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Polycarbonate - LL', 1, 40, 1.5, 4.56, 0.5, 1, 0.998, 0.57, 1.63, 'LL type', '2025-10-24 15:35:23.995835+05:30'),
  ('98622c3f-09cb-46d6-baf1-fd80899033a9', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Polycarbonate - GP', 1, 40, 1.5, 4.56, 0.5, 2, 0.998, 0.57, 1.63, 'GP type', '2025-10-24 15:35:23.995835+05:30')
ON CONFLICT (id) DO NOTHING;

-- Condensed additional variants (some thermal constants unknown)
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('b9b68d19-4ef2-40e5-8696-65ca789b0e85', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Polyethylene terephtalate - LL', 1, 40, 1.29, 4, 0.5, 1, NULL, NULL, NULL, 'LL type', '2025-10-24 15:35:24.021598+05:30'),
  ('da7b9ba2-7be6-4e60-a355-1bd9babcc737', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Polyethylene terephtalate - GP', 1, 40, 1.29, 4, 0.5, 2, NULL, NULL, NULL, 'GP type', '2025-10-24 15:35:24.021598+05:30'),
  ('11aca760-6ef2-4edf-b004-1535860818d0', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Polypropylene - LL', 1, 40, 1.29, 4, 0.5, 1, NULL, NULL, NULL, 'LL type', '2025-10-24 15:35:23.975807+05:30'),
  ('48123ca2-c7de-4e48-9a97-9a58b0db70fd', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Polypropylene - GP', 1, 40, 1.29, 4, 0.5, 2, NULL, NULL, NULL, 'GP type', '2025-10-24 15:35:23.975807+05:30')
ON CONFLICT (id) DO NOTHING;

-- Metalized film
INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('98c978a5-c093-4bfe-a47f-a8586e6ee611', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Acetyl cellulose - LL', 0.7, 40, 1.07, 3.45, 0.5, 1, NULL, NULL, NULL, 'LL type', '2025-10-24 18:56:12.460746+05:30'),
  ('7d807c08-98a0-480e-b61e-8ed2cb8aae83', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Metal foil'), 'Acetyl cellulose - GP', 0.7, 40, 1.07, 3.45, 0.5, 2, NULL, NULL, NULL, 'GP type', '2025-10-24 18:56:12.460746+05:30')
ON CONFLICT (id) DO NOTHING;

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('42310527-a6a3-4fa8-a9f3-df0df80d5f59', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Mica'), 'Mica', 1, 40, 1.12, 2.98, 0.5, 1, 0.86, 0.27, 0.84, 'LL type', '2025-12-05 14:02:44.219846+05:30'),
  ('c0c94714-6fc0-4bad-aed0-2328a28c8d08', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Mica'), 'Mica - GP', 1, 40, 1.12, 2.98, 0.5, 2, 0.86, 0.27, 0.84, 'GP type', '2025-12-05 14:02:44.219846+05:30')
 ON CONFLICT (id) DO NOTHING;

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, c2, c3, uref_umax_ratio, pi_q, a, ea1, ea2, notes, created_at)
VALUES
  ('f4ecd2f7-56a9-4100-9864-5640005ba7e3', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Tantalum electrolytic'), 'non solid electrolyte - LL', 10, 40, 1, 1.05, 0.5, 1, 0.35, 0.54, 0, 'LL type', '2025-10-24 18:56:12.665258+05:30'),
  ('ec955ca9-d2e1-4168-8a61-106be20d0e97', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Capacitor' AND s.name='Tantalum electrolytic'), 'non solid electrolyte - GP', 10, 40, 1, 1.05, 0.5, 2, 0.35, 0.54, 0, 'GP type', '2025-10-24 18:56:12.665258+05:30')
 ON CONFLICT (id) DO NOTHING;

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, notes, created_at)
VALUES
  ('384f1894-3675-4435-a58e-38d306504edd', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Resistor' AND s.name='Thin Film'), '<=100 kΩ', 0.3, 55, 'Carbon film resistor ≤100kΩ', '2025-10-16 18:39:13.527558+05:30'),
  ('541b8492-27f3-4789-aec5-d32ed5430de5', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Resistor' AND s.name='Thin Film'), '>100 kΩ', 1, 55, 'Carbon film resistor >100kΩ', '2025-10-16 18:39:13.527558+05:30')
 ON CONFLICT (id) DO NOTHING;

INSERT INTO component_variants (id, subtype_id, name, ref_fit, ref_temp, notes, created_at)
VALUES
  ('d1f2a3b4-c5d6-47e8-9f01-abcdeffedcba', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Resistor' AND s.name='Thick Film'), '1206 5% 10kΩ', 0.8, 25, 'Thick film 1206 5% 10kΩ', '2025-10-16 18:39:13.527558+05:30'),
  ('e2f3a4b5-c6d7-48f9-0a12-bcdefedcba98', (SELECT s.id FROM component_subtypes s JOIN component_types t ON s.type_id = t.id WHERE t.name='Resistor' AND s.name='Thick Film'), '0805 5% 1kΩ', 0.9, 25, 'Thick film 0805 5% 1kΩ', '2025-10-16 18:39:13.527558+05:30')
 ON CONFLICT (id) DO NOTHING;

COMMIT;

-- End of seed
