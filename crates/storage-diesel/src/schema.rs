// @generated automatically by Diesel CLI.

diesel::table! {
    admin_tokens (token) {
        token -> Text,
        user_id -> Text,
        expires_at -> BigInt,
        single_use -> Bool,
        created_at -> BigInt,
    }
}

diesel::table! {
    admins (user_id) {
        user_id -> Text,
        password_hash -> Text,
        created_at -> BigInt,
        active -> Bool,
    }
}

diesel::table! {
    transactions (id) {
        id -> Text,
        user_id -> Text,
        kind -> Text,
        amount -> BigInt,
        approved_by -> Nullable<Text>,
        created_at -> BigInt,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        username -> Text,
        card_number -> BigInt,
        role -> Text,
        birthdate -> Text,
        comments -> Text,
        balance -> BigInt,
        spent -> BigInt,
    }
}

diesel::joinable!(admin_tokens -> users (user_id));
diesel::joinable!(admins -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(admin_tokens, admins, transactions, users,);
