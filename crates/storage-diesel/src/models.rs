use crate::schema::{admin_tokens, admins, transactions, users, users_with_role};
use diesel::prelude::*;

/// =======================
/// users (row = schema authority)
/// =======================
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = users)]
#[diesel(belongs_to(UserRow, foreign_key = created_by))]
pub struct UserRow {
    pub id: String,
    pub name: String,
    pub username: String,
    pub card_number: i64,
    pub birthdate: String,
    pub comments: String,
    pub balance: i64,
    pub spent: i64,
    pub created_at: i64,
    pub created_by: String,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = users_with_role)]
pub struct UserWithRoleRow {
    pub id: String,
    pub name: String,
    pub username: String,
    pub card_number: i64,
    pub birthdate: String,
    pub comments: String,
    pub balance: i64,
    pub spent: i64,
    pub created_at: i64,
    pub created_by: String,
    pub role: String, // "admin" | "user"
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub name: String,
    pub username: String,
    pub card_number: i64,
    pub birthdate: String,
    pub comments: String,
    pub balance: i64,
    pub spent: i64,
    pub created_at: i64,
    pub created_by: String,
}

/// =======================
/// admins (row = schema authority)
/// =======================
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = admins)]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub struct AdminRow {
    pub id: String,
    pub user_id: String,
    pub password_hash: String,
    pub granted_at: i64,
    pub granted_by: String,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = admins)]
pub struct NewAdminGant {
    pub id: String,
    pub user_id: String,
    pub password_hash: String,
    pub granted_at: i64,
    pub granted_by: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = admins)]
pub struct AdminRevoke {
    pub id: String,
    pub revoked_at: i64,
    pub revoked_by: String,
}

/// =======================
/// transactions (row = schema authority)
/// =======================
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = transactions)]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub struct TransactionRow {
    pub id: String,
    pub user_id: String,
    pub kind: String,
    pub amount: i64,
    pub approved_by: Option<String>,
    pub created_at: i64,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub id: String,
    pub user_id: String,
    pub kind: String,
    pub amount: i64,
    pub approved_by: Option<String>,
    pub created_at: i64,
}

/// =======================
/// admin_tokens (row = schema authority)
/// =======================
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = admin_tokens)]
#[diesel(primary_key(token))]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub struct AdminTokenRow {
    pub token: String,
    pub user_id: String,
    pub expires_at: i64,
    pub single_use: bool,
    pub created_at: i64,
}

#[derive(Insertable)]
#[diesel(table_name = admin_tokens)]
pub struct NewAdminToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: i64,
    pub single_use: bool,
    pub created_at: i64,
}
