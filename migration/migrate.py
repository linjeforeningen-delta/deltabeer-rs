# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "argon2-cffi>=25.1.0",
#     "polars>=1.43.2",
# ]
# ///
import secrets
import shutil
import sqlite3
import uuid
from datetime import datetime, timezone
from getpass import getpass
from pathlib import Path

import polars as pl
from argon2.low_level import Type, hash_secret

PROJECT_ROOT = Path(__file__).resolve().parent.parent

TEMPLATE_DB = PROJECT_ROOT / "crates/storage-diesel/data/template.sqlite"
DB_PATH = PROJECT_ROOT / "migration/migration.sqlite"

CSV_PATH = PROJECT_ROOT / "migration/users.csv"

BOOTSTRAP_USERNAME = "evenyt"


def copy_template_db():
    if not TEMPLATE_DB.is_file():
        raise FileNotFoundError(f"Template database not found: {TEMPLATE_DB}")

    if DB_PATH.exists():
        raise FileExistsError(f"Database already exists: {DB_PATH}")

    shutil.copy2(TEMPLATE_DB, DB_PATH)


def open_db():
    conn = sqlite3.connect(DB_PATH)
    conn.execute("PRAGMA foreign_keys = ON")

    enabled = conn.execute("PRAGMA foreign_keys").fetchone()[0]

    if enabled != 1:
        conn.close()
        raise RuntimeError("Foreign keys are not enabled")

    return conn


def verify_schema(conn: sqlite3.Connection):
    required_tables = {
        "users",
        "admins",
        "transactions",
        "admin_tokens",
    }

    rows = conn.execute(
        """
        SELECT name
        FROM sqlite_master
        WHERE type = 'table'
        """
    ).fetchall()

    existing_tables = {row[0] for row in rows}

    missing = required_tables - existing_tables

    if missing:
        raise RuntimeError(f"Database is missing tables: {sorted(missing)}")


def load_users():
    df = pl.read_csv(
        CSV_PATH,
        schema_overrides={
            "id": pl.String,
            "name": pl.String,
            "username": pl.String,
            "program": pl.String,
            "card_number": pl.Int64,
            "birthdate": pl.String,
            "comments": pl.String,
            "balance": pl.Int64,
            "spent": pl.Int64,
            "created_at": pl.Int64,
        },
    )

    return df


def get_bootstrap_user_id(df: pl.DataFrame) -> str:
    user = df.filter(pl.col("username") == BOOTSTRAP_USERNAME)

    if user.height != 1:
        raise RuntimeError(
            f"Expected exactly one user named {BOOTSTRAP_USERNAME!r}, "
            f"found {user.height}"
        )

    return user["id"].item()


def prepare_users(df: pl.DataFrame, bootstrap_user_id: str) -> pl.DataFrame:
    return df.with_columns(
        pl.col("balance").alias("legacy_balance"),
        pl.col("spent").alias("legacy_spent"),
        pl.lit(0, dtype=pl.Int64).alias("balance"),
        pl.lit(0, dtype=pl.Int64).alias("spent"),
        pl.lit(bootstrap_user_id).alias("created_by"),
    )


def insert_bootstrap_user(
    conn: sqlite3.Connection,
    df: pl.DataFrame,
    bootstrap_user_id: str,
):
    user = df.filter(pl.col("id") == bootstrap_user_id).row(0, named=True)

    conn.execute(
        """
        INSERT INTO users (
            id,
            name,
            username,
            program,
            card_number,
            birthdate,
            comments,
            balance,
            spent,
            created_at,
            created_by
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
        (
            user["id"],
            user["name"],
            user["username"],
            user["program"],
            user["card_number"],
            user["birthdate"],
            user["comments"],
            user["balance"],
            user["spent"],
            user["created_at"],
            bootstrap_user_id,
        ),
    )


def insert_remaining_users(
    conn: sqlite3.Connection,
    df: pl.DataFrame,
    bootstrap_user_id: str,
):
    remaining = df.filter(pl.col("id") != bootstrap_user_id)

    rows = [
        (
            row["id"],
            row["name"],
            row["username"],
            row["program"],
            row["card_number"],
            row["birthdate"],
            row["comments"],
            row["balance"],
            row["spent"],
            row["created_at"],
            row["created_by"],
        )
        for row in remaining.iter_rows(named=True)
    ]

    conn.executemany(
        """
        INSERT INTO users (
            id,
            name,
            username,
            program,
            card_number,
            birthdate,
            comments,
            balance,
            spent,
            created_at,
            created_by
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
        rows,
    )


def verify_users(
    conn: sqlite3.Connection,
    df: pl.DataFrame,
    bootstrap_user_id: str,
):
    # 1. Row count
    expected_count = df.height

    actual_count = conn.execute("SELECT COUNT(*) FROM users").fetchone()[0]

    if actual_count != expected_count:
        raise RuntimeError(
            f"User count mismatch: expected {expected_count}, got {actual_count}"
        )

    # 2. Balance totals
    expected_balance = df["balance"].sum()
    expected_spent = df["spent"].sum()

    actual_balance, actual_spent = conn.execute(
        """
        SELECT
            SUM(balance),
            SUM(spent)
        FROM users
        """
    ).fetchone()

    if actual_balance != expected_balance:
        raise RuntimeError(
            f"Balance mismatch: expected {expected_balance}, got {actual_balance}"
        )

    if actual_spent != expected_spent:
        raise RuntimeError(
            f"Spent mismatch: expected {expected_spent}, got {actual_spent}"
        )

    # 3. Bootstrap user exists
    bootstrap = conn.execute(
        """
        SELECT id, username, created_by
        FROM users
        WHERE id = ?
        """,
        (bootstrap_user_id,),
    ).fetchone()

    if bootstrap is None:
        raise RuntimeError("Bootstrap user was not inserted")

    bootstrap_id, username, created_by = bootstrap

    if username != BOOTSTRAP_USERNAME:
        raise RuntimeError(f"Bootstrap username mismatch: {username!r}")

    if created_by != bootstrap_user_id:
        raise RuntimeError("Bootstrap user's created_by does not reference itself")

    # 4. Everyone points to bootstrap user
    bad_created_by = conn.execute(
        """
        SELECT COUNT(*)
        FROM users
        WHERE created_by != ?
        """,
        (bootstrap_user_id,),
    ).fetchone()[0]

    if bad_created_by != 0:
        raise RuntimeError(f"{bad_created_by} users have unexpected created_by values")

    # 5. Foreign key integrity
    fk_errors = conn.execute("PRAGMA foreign_key_check").fetchall()

    if fk_errors:
        raise RuntimeError(f"Foreign key violations found: {fk_errors}")

    print("User migration verification passed")
    print(f"  users:   {actual_count}")
    print(f"  balance: {actual_balance}")
    print(f"  spent:   {actual_spent}")


def unix_now() -> int:
    return int(datetime.now(timezone.utc).timestamp())


def uuid7_from_unix(timestamp: int) -> uuid.UUID:
    timestamp_ms = timestamp * 1000

    if not 0 <= timestamp_ms < (1 << 48):
        raise ValueError("Timestamp out of UUIDv7 range")

    rand_a = secrets.randbits(12)
    rand_b = secrets.randbits(62)

    value = (timestamp_ms << 80) | (0x7 << 76) | (rand_a << 64) | (0b10 << 62) | rand_b

    return uuid.UUID(int=value)


def create_bootstrap_admin(
    conn: sqlite3.Connection,
    bootstrap_user_id: str,
    password: str,
):
    granted_at = unix_now()
    admin_id = str(uuid7_from_unix(granted_at))
    password_hash = hash_password(password)

    conn.execute(
        """
        INSERT INTO admins (
            id,
            user_id,
            password_hash,
            granted_at,
            granted_by,
            revoked_at,
            revoked_by
        )
        VALUES (?, ?, ?, ?, ?, NULL, NULL)
        """,
        (
            admin_id,
            bootstrap_user_id,
            password_hash,
            granted_at,
            bootstrap_user_id,
        ),
    )

    return admin_id


def verify_bootstrap_admin(
    conn: sqlite3.Connection,
    bootstrap_user_id: str,
):
    row = conn.execute(
        """
        SELECT
            user_id,
            granted_by,
            revoked_at,
            revoked_by
        FROM admins
        WHERE user_id = ?
          AND revoked_at IS NULL
        """,
        (bootstrap_user_id,),
    ).fetchone()

    if row is None:
        raise RuntimeError("Bootstrap admin was not created")

    user_id, granted_by, revoked_at, revoked_by = row

    if user_id != bootstrap_user_id:
        raise RuntimeError("Admin user_id mismatch")

    if granted_by != bootstrap_user_id:
        raise RuntimeError("Bootstrap admin was not self-granted")

    if revoked_at is not None or revoked_by is not None:
        raise RuntimeError("Bootstrap admin is unexpectedly revoked")


ARGON2_MEMORY_COST = 19_456
ARGON2_TIME_COST = 2
ARGON2_PARALLELISM = 1
ARGON2_HASH_LENGTH = 32
ARGON2_SALT_LENGTH = 16


def hash_password(password: str) -> str:
    salt = secrets.token_bytes(ARGON2_SALT_LENGTH)

    return hash_secret(
        secret=password.encode("utf-8"),
        salt=salt,
        time_cost=ARGON2_TIME_COST,
        memory_cost=ARGON2_MEMORY_COST,
        parallelism=ARGON2_PARALLELISM,
        hash_len=ARGON2_HASH_LENGTH,
        type=Type.ID,
        version=19,
    ).decode("utf-8")


def create_migration_transactions(
    conn: sqlite3.Connection,
    df: pl.DataFrame,
    bootstrap_user_id: str,
):
    migration_time = unix_now()

    for row in df.iter_rows(named=True):
        user_id = row["id"]
        legacy_balance = row["legacy_balance"]
        legacy_spent = row["legacy_spent"]

        total_topup = legacy_balance + legacy_spent

        # ----------------------------------------
        # Topup
        # ----------------------------------------

        if total_topup > 0:
            transaction_id = str(uuid7_from_unix(migration_time))

            conn.execute(
                """
                INSERT INTO transactions (
                    id,
                    user_id,
                    kind,
                    amount,
                    source,
                    approved_by,
                    created_at
                )
                VALUES (?, ?, 'topup', ?, 'migration', ?, ?)
                """,
                (
                    transaction_id,
                    user_id,
                    total_topup,
                    bootstrap_user_id,
                    migration_time,
                ),
            )

        # ----------------------------------------
        # Spend
        # ----------------------------------------

        if legacy_spent > 0:
            transaction_id = str(uuid7_from_unix(migration_time))

            conn.execute(
                """
                INSERT INTO transactions (
                    id,
                    user_id,
                    kind,
                    amount,
                    source,
                    approved_by,
                    created_at
                )
                VALUES (?, ?, 'spend', ?, 'migration', NULL, ?)
                """,
                (
                    transaction_id,
                    user_id,
                    legacy_spent,
                    migration_time,
                ),
            )


def create_migration_transactions(
    conn: sqlite3.Connection,
    df: pl.DataFrame,
    bootstrap_user_id: str,
):
    for row in df.iter_rows(named=True):
        user_id = row["id"]
        legacy_balance = row["legacy_balance"]
        legacy_spent = row["legacy_spent"]

        total_topup = legacy_balance + legacy_spent

        if total_topup > 0:
            created_at = unix_now()
            transaction_id = str(uuid7_from_unix(created_at))

            conn.execute(
                """
                INSERT INTO transactions (
                    id,
                    user_id,
                    kind,
                    amount,
                    source,
                    approved_by,
                    created_at
                )
                VALUES (?, ?, 'topup', ?, 'migration', ?, ?)
                """,
                (
                    transaction_id,
                    user_id,
                    total_topup,
                    bootstrap_user_id,
                    created_at,
                ),
            )

        if legacy_spent > 0:
            created_at = unix_now()
            transaction_id = str(uuid7_from_unix(created_at))

            conn.execute(
                """
                INSERT INTO transactions (
                    id,
                    user_id,
                    kind,
                    amount,
                    source,
                    approved_by,
                    created_at
                )
                VALUES (?, ?, 'spend', ?, 'migration', NULL, ?)
                """,
                (
                    transaction_id,
                    user_id,
                    legacy_spent,
                    created_at,
                ),
            )


def apply_migration_balances(
    conn: sqlite3.Connection,
    df: pl.DataFrame,
):
    for row in df.iter_rows(named=True):
        total_topup = row["legacy_balance"] + row["legacy_spent"]

        if total_topup > 0:
            conn.execute(
                """
                UPDATE users
                SET balance = balance + ?
                WHERE id = ?
                """,
                (
                    total_topup,
                    row["id"],
                ),
            )

        if row["legacy_spent"] > 0:
            conn.execute(
                """
                UPDATE users
                SET
                    balance = balance - ?,
                    spent = spent + ?
                WHERE id = ?
                """,
                (
                    row["legacy_spent"],
                    row["legacy_spent"],
                    row["id"],
                ),
            )


def verify_migration_transactions(
    conn: sqlite3.Connection,
    df: pl.DataFrame,
):
    for row in df.iter_rows(named=True):
        actual = conn.execute(
            """
            SELECT balance, spent
            FROM users
            WHERE id = ?
            """,
            (row["id"],),
        ).fetchone()

        if actual is None:
            raise RuntimeError(f"Missing user {row['username']!r}")

        actual_balance, actual_spent = actual

        if actual_balance != row["legacy_balance"]:
            raise RuntimeError(
                f"{row['username']}: "
                f"balance expected {row['legacy_balance']}, "
                f"got {actual_balance}"
            )

        if actual_spent != row["legacy_spent"]:
            raise RuntimeError(
                f"{row['username']}: "
                f"spent expected {row['legacy_spent']}, "
                f"got {actual_spent}"
            )


def verify_transaction_ledger(
    conn: sqlite3.Connection,
):
    rows = conn.execute(
        """
        SELECT
            u.id,
            u.username,
            u.balance,
            u.spent,
            COALESCE(SUM(
                CASE
                    WHEN t.kind = 'topup' THEN t.amount
                    ELSE 0
                END
            ), 0) AS topups,
            COALESCE(SUM(
                CASE
                    WHEN t.kind = 'spend' THEN t.amount
                    ELSE 0
                END
            ), 0) AS spends
        FROM users u
        LEFT JOIN transactions t
            ON t.user_id = u.id
        GROUP BY u.id
        """
    ).fetchall()

    for (
        user_id,
        username,
        balance,
        spent,
        topups,
        spends,
    ) in rows:
        if balance != topups - spends:
            raise RuntimeError(f"{username}: ledger balance mismatch")

        if spent != spends:
            raise RuntimeError(f"{username}: spent mismatch")


def main():
    password = getpass("Bootstrap admin password: ")
    password_confirm = getpass("Confirm password: ")

    if password != password_confirm:
        raise RuntimeError("Passwords do not match")

    copy_template_db()

    conn = open_db()

    try:
        verify_schema(conn)

        df = load_users()
        bootstrap_user_id = get_bootstrap_user_id(df)

        df = prepare_users(
            df,
            bootstrap_user_id,
        )

        conn.execute("BEGIN")

        insert_bootstrap_user(
            conn,
            df,
            bootstrap_user_id,
        )

        insert_remaining_users(
            conn,
            df,
            bootstrap_user_id,
        )

        admin_id = create_bootstrap_admin(
            conn,
            bootstrap_user_id,
            password,
        )

        verify_bootstrap_admin(
            conn,
            bootstrap_user_id,
        )

        create_migration_transactions(
            conn,
            df,
            bootstrap_user_id,
        )

        apply_migration_balances(
            conn,
            df,
        )

        verify_migration_transactions(
            conn,
            df,
        )

        verify_transaction_ledger(
            conn,
        )

        conn.commit()

        print("Migration committed successfully")
        print(f"Bootstrap admin ID: {admin_id}")

    except Exception:
        conn.rollback()
        raise

    finally:
        conn.close()


if __name__ == "__main__":
    main()
