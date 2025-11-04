use delta_core::domain::User;
use rusqlite::Row;

pub fn row_to_user(row: &Row<'_>) -> rusqlite::Result<User> {
    Ok(User {
        id: row.get("id")?,
        username: row.get("username")?,
        name: row.get("name")?,
        card_number: row.get("card_number")?,
        role: row.get("role")?,
        birthdate: row.get("birthdate")?,
        comments: row.get("comments")?,
        balance: row.get("balance")?,
        spent: row.get("spent")?,
    })
}

// pub fn user_to_row(user: &User) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
//
// }
