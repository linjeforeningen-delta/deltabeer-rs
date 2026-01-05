-- Disable foreign key enforcement temporarily for teardown
PRAGMA foreign_keys = OFF;

-- =========================
-- drop triggers
-- =========================

DROP TRIGGER IF EXISTS prevent_users_mutation;
DROP TRIGGER IF EXISTS prevent_users_delete;

DROP TRIGGER IF EXISTS prevent_transaction_delete;
DROP TRIGGER IF EXISTS prevent_transaction_update;

DROP TRIGGER IF EXISTS prevent_admin_delete;
DROP TRIGGER IF EXISTS prevent_admin_grant_update;
DROP TRIGGER IF EXISTS enforce_admin_revocation_rules;

DROP TRIGGER IF EXISTS prevent_admin_token_delete;
DROP TRIGGER IF EXISTS prevent_admin_token_identity_update;
DROP TRIGGER IF EXISTS prevent_admin_token_expiry_extension;
DROP TRIGGER IF EXISTS prevent_reactivating_expired_token;

DROP TRIGGER IF EXISTS enforce_active_admin_on_transaction;

-- =========================
-- drop indexes
-- =========================

DROP INDEX IF EXISTS uniq_active_admin_per_user;

-- =========================
-- drop views
-- =========================

DROP VIEW IF EXISTS users_with_role;


-- =========================
-- drop tables (children first)
-- =========================

DROP TABLE IF EXISTS admin_tokens;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS admins;
DROP TABLE IF EXISTS users;

-- Re-enable foreign key enforcement
PRAGMA foreign_keys = ON;
