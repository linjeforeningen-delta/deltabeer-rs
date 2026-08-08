// @generated automatically by Diesel CLI.

diesel::table! {
    admin_tokens (token) {
        token -> Binary,
        user_id -> Text,
        expires_at -> BigInt,
        single_use -> Bool,
        created_at -> BigInt,
        expired -> Bool,
    }
}

diesel::table! {
    admins (id) {
        id -> Text,
        user_id -> Text,
        password_hash -> Text,
        granted_at -> BigInt,
        granted_by -> Text,
        revoked_at -> Nullable<BigInt>,
        revoked_by -> Nullable<Text>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Text,
        user_id -> Text,
        kind -> Text,
        amount -> BigInt,
        source -> Text,
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
        birthdate -> Text,
        comments -> Text,
        balance -> BigInt,
        spent -> BigInt,
        created_at -> BigInt,
        created_by -> Text,
    }
}

diesel::joinable!(admin_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(admin_tokens, admins, transactions, users,);
