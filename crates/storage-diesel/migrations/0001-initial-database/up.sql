-- Your SQL goes here
-- Enable foreign key enforcement (SQLite requires this)
PRAGMA foreign_keys = ON;

-- =========================
-- users
-- =========================
CREATE TABLE users (
                       id TEXT PRIMARY KEY NOT NULL,
                       name TEXT NOT NULL,
                       username TEXT NOT NULL UNIQUE,
                       card_number BIGINT NOT NULL UNIQUE ,
                       role TEXT NOT NULL,
                       birthdate TEXT NOT NULL,              -- YYYY-MM-DD
                       comments TEXT NOT NULL DEFAULT '',
                       balance BIGINT NOT NULL DEFAULT 0,
                       spent BIGINT NOT NULL DEFAULT 0
);

-- =========================
-- admins
-- =========================
CREATE TABLE admins (
                        user_id TEXT PRIMARY KEY NOT NULL,
                        password_hash TEXT NOT NULL,
                        created_at BIGINT NOT NULL,           -- unix timestamp
                        active BOOLEAN NOT NULL DEFAULT TRUE,

                        FOREIGN KEY (user_id)
                            REFERENCES users(id)
                            ON DELETE RESTRICT
);

-- =========================
-- transactions
-- =========================
CREATE TABLE transactions (
                      id TEXT PRIMARY KEY NOT NULL,
                      user_id TEXT NOT NULL,
                      kind TEXT NOT NULL,
                      amount BIGINT NOT NULL,
                      approved_by TEXT,
                      created_at BIGINT NOT NULL,   -- unix timestamp

                      FOREIGN KEY (user_id)
                          REFERENCES users(id)
                          ON DELETE RESTRICT,

                      FOREIGN KEY (approved_by)
                          REFERENCES users(id)
                          ON DELETE RESTRICT,

                      CHECK (
                          kind IN ('topup', 'spend')
                              AND (
                              (kind = 'topup' AND approved_by IS NOT NULL)
                                  OR
                              (kind = 'spend' AND approved_by IS NULL)
                              )
                          )
);


-- =========================
-- admin_tokens
-- =========================
CREATE TABLE admin_tokens (
                              token TEXT PRIMARY KEY NOT NULL,
                              user_id TEXT NOT NULL,
                              expires_at BIGINT NOT NULL,           -- unix timestamp
                              single_use BOOLEAN NOT NULL DEFAULT TRUE,
                              created_at BIGINT NOT NULL,            -- unix timestamp

                              FOREIGN KEY (user_id)
                                  REFERENCES users(id)
                                  ON DELETE RESTRICT
);

-- Prevent deletion of users (immutability)
CREATE TRIGGER prevent_users_delete
    BEFORE DELETE ON users
BEGIN
    SELECT RAISE(ABORT, 'users cannot be deleted');
END;

-- Prevent deletion of transactions (immutability)
CREATE TRIGGER prevent_transaction_delete
    BEFORE DELETE ON transactions
BEGIN
    SELECT RAISE(ABORT, 'transactions are immutable');
END;

-- Ensure approved_by is an active admin for topup transactions
CREATE TRIGGER enforce_active_admin_on_transaction
    BEFORE INSERT ON transactions
    WHEN NEW.kind = 'topup'
BEGIN
    SELECT
        CASE
            WHEN NOT EXISTS (
                SELECT 1
                FROM admins
                WHERE user_id = NEW.approved_by
                  AND active = 1
            )
                THEN RAISE(ABORT, 'approved_by must be an active admin')
            END;
END;