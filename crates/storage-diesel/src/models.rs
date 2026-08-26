use crate::schema::{admin_tokens, admins, transactions, users, users_with_role};
use diesel::prelude::*;

/// =======================
/// users (row = schema authority)
/// =======================
#[allow(dead_code)]
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = users)]
#[diesel(belongs_to(UserRow, foreign_key = created_by))]
pub(super) struct UserRow {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) username: String,
    pub(super) program: String,
    pub(super) card_number: i64,
    pub(super) birthdate: String,
    pub(super) comments: String,
    pub(super) balance: i64,
    pub(super) spent: i64,
    pub(super) created_at: i64,
    pub(super) created_by: String,
}

#[allow(dead_code)]
#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = users_with_role)]
pub(super) struct UserWithRoleRow {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) username: String,
    pub(super) program: String,
    pub(super) card_number: i64,
    pub(super) birthdate: String,
    pub(super) comments: String,
    pub(super) balance: i64,
    pub(super) spent: i64,
    pub(super) created_at: i64,
    pub(super) created_by: String,
    pub(super) role: String, // "admin" | "user"
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub(super) struct NewUser {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) username: String,
    pub(super) program: String,
    pub(super) card_number: i64,
    pub(super) birthdate: String,
    pub(super) comments: String,
    pub(super) balance: i64,
    pub(super) spent: i64,
    pub(super) created_at: i64,
    pub(super) created_by: String,
}

/// =======================
/// admins (row = schema authority)
/// =======================
#[allow(dead_code)]
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = admins)]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub(super) struct AdminRow {
    pub(super) id: String,
    pub(super) user_id: String,
    pub(super) password_hash: String,
    pub(super) granted_at: i64,
    pub(super) granted_by: String,
    pub(super) revoked_at: Option<i64>,
    pub(super) revoked_by: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = admins)]
pub(super) struct NewAdminGrant {
    pub(super) id: String,
    pub(super) user_id: String,
    pub(super) password_hash: String,
    pub(super) granted_at: i64,
    pub(super) granted_by: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = admins)]
pub(super) struct AdminRevoke {
    pub(super) id: String,
    pub(super) revoked_at: i64,
    pub(super) revoked_by: String,
}

/// =======================
/// transactions (row = schema authority)
/// =======================
#[allow(dead_code)]
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = transactions)]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub(super) struct TransactionRow {
    pub(super) id: String,
    pub(super) user_id: String,
    pub(super) kind: String,
    pub(super) amount: i64,
    pub(super) source: String,
    pub(super) approved_by: Option<String>,
    pub(super) created_at: i64,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub(super) struct NewTransaction {
    pub(super) id: String,
    pub(super) user_id: String,
    pub(super) kind: String,
    pub(super) amount: i64,
    pub(super) source: String,
    pub(super) approved_by: Option<String>,
    pub(super) created_at: i64,
}

/// =======================
/// admin_tokens (row = schema authority)
/// =======================
#[allow(dead_code)]
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = admin_tokens)]
#[diesel(primary_key(token))]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub(super) struct AdminTokenRow {
    pub(super) token: Vec<u8>,
    pub(super) user_id: String,
    pub(super) expires_at: i64,
    pub(super) single_use: bool,
    pub(super) created_at: i64,
    pub(super) expired: bool,
}

#[derive(Insertable)]
#[diesel(table_name = admin_tokens)]
pub(super) struct NewAdminToken {
    pub(super) token: Vec<u8>,
    pub(super) user_id: String,
    pub(super) expires_at: i64,
    pub(super) single_use: bool,
    pub(super) created_at: i64,
}
