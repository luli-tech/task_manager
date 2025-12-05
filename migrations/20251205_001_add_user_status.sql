-- Add user status fields for admin and active status
ALTER TABLE users ADD COLUMN IF NOT EXISTS is_admin BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE users ADD COLUMN IF NOT EXISTS is_active BOOLEAN NOT NULL DEFAULT true;

-- Create indexes for admin and active status queries
CREATE INDEX IF NOT EXISTS idx_users_is_admin ON users(is_admin);
CREATE INDEX IF NOT EXISTS idx_users_is_active ON users(is_active);

-- Set the first user as admin (if exists)
UPDATE users 
SET is_admin = true 
WHERE id = (SELECT id FROM users ORDER BY created_at ASC LIMIT 1);
