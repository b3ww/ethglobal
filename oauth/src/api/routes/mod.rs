mod user_routes;
mod oauth_routes;

use axum::{
    routing::{get, Router},
};

use crate::repository::DbPool;

pub fn create_routes(pool: DbPool) -> Router {
    Router::new()
        .nest("/api/users", user_routes::user_routes(pool.clone()))
        .nest("/api/auth", oauth_routes::oauth_routes(pool.clone()))
        .route("/", get(|| async { "Hello, World!" }))
}
