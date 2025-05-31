// @generated automatically by Diesel CLI.

diesel::table! {
    issue_comments (id) {
        id -> Int4,
        content -> Text,
        is_draft -> Bool,
        #[max_length = 100]
        github_comment_id -> Nullable<Varchar>,
        issue_id -> Int4,
        author_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    issue_labels (id) {
        id -> Int4,
        issue_id -> Int4,
        label_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    issues (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        status -> Varchar,
        is_draft -> Bool,
        #[max_length = 100]
        github_issue_id -> Nullable<Varchar>,
        github_issue_number -> Nullable<Int4>,
        #[max_length = 255]
        github_issue_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
        closed_at -> Nullable<Timestamp>,
        repository_id -> Int4,
        author_id -> Int4,
        assignee_id -> Nullable<Int4>,
    }
}

diesel::table! {
    labels (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 7]
        color -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        repository_id -> Int4,
        #[max_length = 100]
        github_label_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repositories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        owner -> Varchar,
        #[max_length = 100]
        github_repo_id -> Varchar,
        #[max_length = 255]
        github_repo_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        github_id -> Nullable<Varchar>,
        github_username -> Nullable<Varchar>,
        avatar_url -> Nullable<Varchar>,
        access_token -> Nullable<Varchar>,
    }
}

diesel::joinable!(issue_comments -> issues (issue_id));
diesel::joinable!(issue_comments -> users (author_id));
diesel::joinable!(issue_labels -> issues (issue_id));
diesel::joinable!(issue_labels -> labels (label_id));
diesel::joinable!(issues -> repositories (repository_id));
diesel::joinable!(labels -> repositories (repository_id));

diesel::allow_tables_to_appear_in_same_query!(
    issue_comments,
    issue_labels,
    issues,
    labels,
    repositories,
    users,
);
