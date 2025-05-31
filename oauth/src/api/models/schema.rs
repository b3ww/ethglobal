// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        github_id -> Nullable<Int8>,
        github_username -> Nullable<Varchar>,
        avatar_url -> Nullable<Varchar>,
        access_token -> Nullable<Varchar>,
    }
}
