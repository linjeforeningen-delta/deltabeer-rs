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

diesel::table! {
    users_with_role (id) {
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
        role -> Text,
    }
}

diesel::joinable!(admin_tokens -> users (user_id));
diesel::joinable!(admins -> users (user_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin_tokens,
    admins,
    transactions,
    users,
    users_with_role,
);
