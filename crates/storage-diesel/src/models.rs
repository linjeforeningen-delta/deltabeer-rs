use crate::schema::{admin_tokens, admins, transactions, users};
use diesel::prelude::*;

/// =======================
/// users
/// =======================
#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct UserRow {
    pub id: String, // UUID as TEXT
    pub name: String,
    pub username: String,
    pub card_number: i64,
    pub role: String,
    pub birthdate: String, // YYYY-MM-DD
    pub comments: String,
    pub balance: i64,
    pub spent: i64,
}

/// Insert model
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: String,
    pub name: &'a str,
    pub username: &'a str,
    pub card_number: i64,
    pub role: &'a str,
    pub birthdate: String,
    pub comments: &'a str,
    pub balance: i64,
    pub spent: i64,
}

/// =======================
/// admins
/// =======================
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = admins)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub struct AdminRow {
    pub user_id: String, // FK + PK
    pub password_hash: String,
    pub created_at: i64, // unix timestamp
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = admins)]
pub struct NewAdmin<'a> {
    pub user_id: String,
    pub password_hash: &'a str,
    pub created_at: i64,
    pub active: bool,
}

/// =======================
/// transactions
/// =======================
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = transactions)]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
#[diesel(belongs_to(AdminRow, foreign_key = approved_by))]
pub struct TransactionRow {
    pub id: String,
    pub user_id: String,
    pub kind: String, // 'topup' | 'spend'
    pub amount: i64,
    pub approved_by: Option<String>,
    pub created_at: i64,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub id: String,
    pub user_id: String,
    pub kind: &'a str,
    pub amount: i64,
    pub approved_by: Option<String>,
    pub created_at: i64,
}

/// =======================
/// admin_tokens
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
pub struct NewAdminToken<'a> {
    pub token: &'a str,
    pub user_id: &'a str,
    pub expires_at: i64,
    pub single_use: bool,
    pub created_at: i64,
}
