-- Mission profiles table
CREATE TABLE IF NOT EXISTS mission_profiles (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  description TEXT,
  temp_tau_profile JSONB NOT NULL, -- stores array of (temperature, tau) pairs
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Add index for fast lookup
CREATE INDEX IF NOT EXISTS idx_mission_profiles_name ON mission_profiles(name);

-- Add foreign key to components table if not already present
ALTER TABLE components ADD COLUMN IF NOT EXISTS mission_profile_id UUID REFERENCES mission_profiles(id);
