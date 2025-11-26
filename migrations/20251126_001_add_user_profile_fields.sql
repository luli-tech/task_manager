-- Add user profile fields for personalization
ALTER TABLE users ADD COLUMN IF NOT EXISTS bio TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS theme VARCHAR(50) NOT NULL DEFAULT 'light';

-- Create index for theme for potential filtering
CREATE INDEX IF NOT EXISTS idx_users_theme ON users(theme);
