use axum::{
    extract::{Query, State},
    http::{StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize};

use crate::repository::DbPool;
use crate::api::service::{OAuthService, OAuthServiceError};

pub fn oauth_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/github", get(github_auth))
        .route("/github/callback", get(github_callback))
        .route("/me", get(get_current_user))
        .with_state(pool)
}

async fn github_auth() -> impl IntoResponse {
    match OAuthService::get_authorization_url() {
        Ok((auth_url, csrf_token)) => {
            let cookie = Cookie::build(("csrf_token", csrf_token.secret().clone()))
                .path("/")
                .http_only(true)
                .build();

            (CookieJar::new().add(cookie), Redirect::to(&auth_url)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get authorization URL: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to initialize GitHub authentication",
            )
                .into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

async fn github_callback(
    Query(params): Query<CallbackParams>,
    jar: CookieJar,
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    // Get CSRF token from cookie
    let csrf_token = match jar.get("csrf_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Missing CSRF token cookie",
            )
                .into_response();
        }
    };

    // Exchange code for token
    let access_token = match OAuthService::exchange_code_for_token(params.code, csrf_token).await {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Failed to exchange code for token: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to authenticate with GitHub",
            )
                .into_response();
        }
    };

    // Authenticate user
    match OAuthService::authenticate_github_user(access_token, &pool).await {
        Ok(user) => {
            // Set auth cookie
            let auth_cookie = Cookie::build(("auth_token", user.id.to_string()))
                .path("/")
                .http_only(true)
                .build();

            // Redirect to frontend with success
            (
                CookieJar::new().add(auth_cookie).remove(Cookie::from("csrf_token")),
                Redirect::to("/auth/success"),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to authenticate user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to authenticate user",
            )
                .into_response()
        }
    }
}

async fn get_current_user(
    jar: CookieJar,
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    // Get user ID from auth cookie
    let user_id = match jar.get("auth_token") {
        Some(cookie) => match cookie.value().parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                return (StatusCode::UNAUTHORIZED, "Invalid auth token").into_response();
            }
        },
        None => {
            return (StatusCode::UNAUTHORIZED, "Not authenticated").into_response();
        }
    };

    // Get user from database
    use crate::api::service::{UserService, UserServiceError};
    match UserService::get_user_by_id(user_id, &pool) {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(UserServiceError::NotFound) => {
            (StatusCode::UNAUTHORIZED, "User not found").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get user information",
            )
                .into_response()
        }
    }
}
