use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::models::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub github_id: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub github_id: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub github_id: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub id: i64,
    pub login: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}
