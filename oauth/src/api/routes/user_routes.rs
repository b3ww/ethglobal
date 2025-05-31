use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get},
    Json, Router,
};

use crate::api::models::user::{NewUser, UpdateUser};
use crate::repository::DbPool;
use crate::api::service::{UserService, UserServiceError};

pub fn user_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool)
}

async fn get_users(State(pool): State<DbPool>) -> impl IntoResponse {
    match UserService::get_all_users(&pool) {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get users: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

async fn get_user(Path(id): Path<i32>, State(pool): State<DbPool>) -> impl IntoResponse {
    match UserService::get_user_by_id(id, &pool) {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(UserServiceError::NotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to get user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

async fn create_user(
    State(pool): State<DbPool>,
    Json(new_user): Json<NewUser>,
) -> impl IntoResponse {
    match UserService::create_user(new_user, &pool) {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

async fn update_user(
    Path(id): Path<i32>,
    State(pool): State<DbPool>,
    Json(user): Json<UpdateUser>,
) -> impl IntoResponse {
    match UserService::update_user(id, user, &pool) {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(UserServiceError::NotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to update user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

async fn delete_user(Path(id): Path<i32>, State(pool): State<DbPool>) -> impl IntoResponse {
    match UserService::delete_user(id, &pool) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(UserServiceError::NotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to delete user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}
