-- Disable foreign key checks temporarily to allow clean teardown
PRAGMA foreign_keys = OFF;

-- =========================
-- triggers
-- =========================
DROP TRIGGER IF EXISTS enforce_active_admin_on_transaction;
DROP TRIGGER IF EXISTS prevent_transaction_delete;
DROP TRIGGER IF EXISTS prevent_users_delete;

-- =========================
-- tables (drop children first)
-- =========================
DROP TABLE IF EXISTS admin_tokens;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS admins;
DROP TABLE IF EXISTS users;

-- Re-enable foreign key checks
PRAGMA foreign_keys = ON;
