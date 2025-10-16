-- 0004_add_component_hierarchy.sql
-- Adds structured component hierarchy: Type → Subtype → Variant
-- Each Variant stores refFIT (base reliability) and ref_temp (reference temperature)
-- Compatible with mission-profile-based temperature scaling

BEGIN;

-- ============================================================
-- 1️⃣ Component Types (Top Level)
-- ============================================================
CREATE TABLE IF NOT EXISTS component_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- ============================================================
-- 2️⃣ Component Subtypes (Linked to Type)
-- ============================================================
CREATE TABLE IF NOT EXISTS component_subtypes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_id UUID NOT NULL REFERENCES component_types(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(type_id, name)
);

-- ============================================================
-- 3️⃣ Component Variants (Linked to Subtype)
-- ============================================================
CREATE TABLE IF NOT EXISTS component_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subtype_id UUID NOT NULL REFERENCES component_subtypes(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    ref_fit DOUBLE PRECISION NOT NULL,         -- Base FIT value (failures per 1e9h)
    ref_temp DOUBLE PRECISION NOT NULL,        -- Reference temperature (°C)
    notes TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(subtype_id, name)
);

-- ============================================================
-- 4️⃣ Example Data (Optional — Can be removed for production)
-- ============================================================
INSERT INTO component_types (name, description)
VALUES
    ('Resistor', 'Passive resistor components'),
    ('Capacitor', 'Passive capacitor components');

INSERT INTO component_subtypes (type_id, name, description)
SELECT id, 'Thin Film', 'Precision resistors (low TCR)' FROM component_types WHERE name = 'Resistor';

INSERT INTO component_subtypes (type_id, name, description)
SELECT id, 'Thick Film', 'General-purpose resistors' FROM component_types WHERE name = 'Resistor';

INSERT INTO component_variants (subtype_id, name, ref_fit, ref_temp, notes)
SELECT id, '0603 1% 100Ω', 0.30, 40, 'Base FIT @ 40°C' FROM component_subtypes WHERE name = 'Thin Film';

INSERT INTO component_variants (subtype_id, name, ref_fit, ref_temp, notes)
SELECT id, '1206 5% 10kΩ', 0.80, 25, 'Base FIT @ 25°C' FROM component_subtypes WHERE name = 'Thick Film';

-- ============================================================
-- 5️⃣ Link existing components to variant
-- ============================================================
ALTER TABLE components
ADD COLUMN IF NOT EXISTS variant_id UUID REFERENCES component_variants(id) ON DELETE RESTRICT;

-- Optional: update existing components with correct variant_id after seeding variants
-- Example:
-- UPDATE components c
-- SET variant_id = v.id
-- FROM component_variants v
-- WHERE c.manufacturer_part_number = '0603 1% 100Ω' AND v.name = '0603 1% 100Ω';

COMMIT;
