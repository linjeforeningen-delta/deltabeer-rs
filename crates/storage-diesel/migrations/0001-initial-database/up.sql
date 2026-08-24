-- Your SQL goes here
-- Enable foreign key enforcement (SQLite requires this)
PRAGMA foreign_keys = ON;

-- =========================
-- users
-- =========================
CREATE TABLE users
(
    id          TEXT PRIMARY KEY NOT NULL,
    name        TEXT             NOT NULL,
    username    TEXT             NOT NULL UNIQUE,
    program     TEXT             NOT NULL DEFAULT '',
    card_number BIGINT           NOT NULL UNIQUE,
    birthdate   TEXT             NOT NULL, -- YYYY-MM-DD
    comments    TEXT             NOT NULL DEFAULT '',
    balance     BIGINT           NOT NULL,
    spent       BIGINT           NOT NULL,
    created_at  BIGINT           NOT NULL, -- unix timestamp
    created_by  TEXT             NOT NULL,

    FOREIGN KEY (created_by)
        REFERENCES users (id)
        ON DELETE RESTRICT,

    CHECK (length(name) > 0),
    CHECK (length(username) > 0),
    CHECK (card_number >= 0 AND card_number <= 4294967295),
    CHECK (balance >= 0),
    CHECK (spent >= 0)
);

-- Prevent deletion of users
CREATE TRIGGER prevent_users_delete
    BEFORE DELETE
    ON users
BEGIN
    SELECT RAISE(ABORT, 'users cannot be deleted');
END;

-- Prevent modification of immutable user fields
CREATE TRIGGER prevent_users_mutation
    BEFORE UPDATE OF id, birthdate, created_at, created_by
    ON users
BEGIN
    SELECT RAISE(ABORT, 'these user fields are immutable');
END;

-- =========================
-- admins
-- =========================
CREATE TABLE admins
(
    id            TEXT PRIMARY KEY NOT NULL,
    user_id       TEXT             NOT NULL,
    password_hash TEXT             NOT NULL,
    granted_at    BIGINT           NOT NULL,          -- unix timestamp
    granted_by    TEXT             NOT NULL,
    revoked_at    BIGINT           NULL DEFAULT NULL, -- unix timestamp
    revoked_by    TEXT             NULL DEFAULT NULL,

    FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE RESTRICT,
    FOREIGN KEY (granted_by)
        REFERENCES users (id)
        ON DELETE RESTRICT,
    FOREIGN KEY (revoked_by)
        REFERENCES users (id)
        ON DELETE RESTRICT

);

-- Prevent deletion of admin history
CREATE TRIGGER prevent_admin_delete
    BEFORE DELETE
    ON admins
BEGIN
    SELECT RAISE(ABORT, 'admin history cannot be deleted');
END;

-- Prevent modification of admin history
CREATE TRIGGER prevent_admin_grant_update
    BEFORE UPDATE OF id, user_id, granted_at, granted_by
    ON admins
BEGIN
    SELECT RAISE(ABORT, 'admin grant records are immutable');
END;

-- Enforce admin revocation rules
CREATE TRIGGER enforce_admin_revocation_rules
    BEFORE UPDATE OF revoked_at, revoked_by
    ON admins
BEGIN
    -- Rule 0: Revocation metadata is immutable once set
    SELECT CASE
               WHEN OLD.revoked_at IS NOT NULL THEN
                   RAISE(ABORT, 'admin revocation is immutable')
               END;
    -- Rule 1: Revocation must set both fields together
    SELECT CASE
               WHEN (NEW.revoked_at IS NULL) != (NEW.revoked_by IS NULL) THEN
                   RAISE(ABORT, 'revocation must set both revoked_at and revoked_by')
               END;

    -- Rule 2: Admin cannot revoke themselves
    SELECT CASE
               WHEN NEW.revoked_at IS NOT NULL
                   AND NEW.revoked_by = OLD.user_id THEN
                   RAISE(ABORT, 'admin cannot revoke themselves')
               END;
END;

-- =========================
-- transactions
-- =========================
CREATE TABLE transactions
(
    id          TEXT PRIMARY KEY NOT NULL,
    user_id     TEXT             NOT NULL,
    kind        TEXT             NOT NULL,
    amount      BIGINT           NOT NULL,
    source      TEXT             NOT NULL DEFAULT 'live',
    approved_by TEXT,
    created_at  BIGINT           NOT NULL, -- unix timestamp

    FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE RESTRICT,

    FOREIGN KEY (approved_by)
        REFERENCES users (id)
        ON DELETE RESTRICT,

    CHECK (
        kind IN ('topup', 'spend')
            AND (
            (kind = 'topup' AND approved_by IS NOT NULL)
                OR
            (kind = 'spend' AND approved_by IS NULL)
            )
        ),
    CHECK (amount >= 0),
    CHECK (source IN ('live', 'migration', 'adjustment'))
);

-- Prevent deletion of transaction history
CREATE TRIGGER prevent_transaction_delete
    BEFORE DELETE
    ON transactions
BEGIN
    SELECT RAISE(ABORT, 'transactions are immutable');
END;

-- Prevent modification of transaction history
CREATE TRIGGER prevent_transaction_update
    BEFORE UPDATE
    ON transactions
BEGIN
    SELECT RAISE(ABORT, 'transactions are immutable');
END;

-- =========================
-- admin_tokens
-- =========================
CREATE TABLE admin_tokens
(
    token      BLOB PRIMARY KEY NOT NULL,
    user_id    TEXT             NOT NULL,
    expires_at BIGINT           NOT NULL, -- unix timestamp
    single_use BOOLEAN          NOT NULL,
    created_at BIGINT           NOT NULL, -- unix timestamp
    expired    BOOLEAN          NOT NULL DEFAULT FALSE,

    FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE RESTRICT
);

-- Prevent deletion of admin tokens
CREATE TRIGGER prevent_admin_token_delete
    BEFORE DELETE
    ON admin_tokens
BEGIN
    SELECT RAISE(ABORT, 'admin tokens cannot be deleted');
END;

-- Prevent modification of admin token identity
CREATE TRIGGER prevent_admin_token_identity_update
    BEFORE UPDATE OF token, user_id, created_at
    ON admin_tokens
BEGIN
    SELECT RAISE(ABORT, 'token identity is immutable');
END;

-- Prevent extending admin token expiry
CREATE TRIGGER prevent_admin_token_expiry_extension
    BEFORE UPDATE OF expires_at
    ON admin_tokens
    WHEN NEW.expires_at > OLD.expires_at
BEGIN
    SELECT RAISE(ABORT, 'token expiry cannot be extended');
END;

-- Prevent reactivating expired token
CREATE TRIGGER prevent_reactivating_expired_token
    Before UPDATE OF expired
    ON admin_tokens
    WHEN (NEW.expired = FALSE AND OLD.expired = TRUE)
BEGIN
    SELECT RAISE(ABORT, 'expired tokens cannot be reactivated');
END;

-- =========================
-- misc
-- =========================

-- Ensure approved_by is a currently active admin for topup transactions
CREATE TRIGGER enforce_active_admin_on_transaction
    BEFORE INSERT
    ON transactions
    WHEN NEW.kind = 'topup'
BEGIN
    SELECT CASE
               WHEN NEW.approved_by IS NULL THEN
                   RAISE(ABORT, 'topup transactions require approval')
               WHEN NOT EXISTS (SELECT 1
                                FROM admins
                                WHERE user_id = NEW.approved_by
                                  AND revoked_at IS NULL) THEN
                   RAISE(ABORT, 'approved_by must be an active admin')
               END;
END;

-- Ensure only one active admin per user
CREATE UNIQUE INDEX uniq_active_admin_per_user
    ON admins (user_id)
    WHERE revoked_at IS NULL;


-- Create a view of users with role
CREATE VIEW users_with_role AS
SELECT users.*,
       CASE
           WHEN EXISTS (SELECT 1
                        FROM admins
                        WHERE admins.user_id = users.id
                          AND admins.revoked_at IS NULL)
               THEN 'admin'
           ELSE 'user'
           END AS role
FROM users;
