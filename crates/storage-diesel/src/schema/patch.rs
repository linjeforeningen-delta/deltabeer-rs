use super::generated::*;

diesel::table! {
    users_with_role (id) {
        id -> Text,
        name -> Text,
        username -> Text,
        program -> Text,
        card_number -> BigInt,
        birthdate -> Text,
        comments -> Text,
        balance -> BigInt,
        spent -> BigInt,
        created_at -> BigInt,
        created_by -> Text,
        role -> Text,
    }
}

diesel::joinable!(admins -> users (user_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(users, users_with_role,);
diesel::allow_tables_to_appear_in_same_query!(admins, users_with_role,);
diesel::allow_tables_to_appear_in_same_query!(transactions, users_with_role,);
diesel::allow_tables_to_appear_in_same_query!(admin_tokens, users_with_role,);
