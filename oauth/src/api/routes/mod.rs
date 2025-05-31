mod oauth_routes;

use axum::{
    routing::{get, Router},
};

use crate::repository::DbPool;

pub fn create_routes(pool: DbPool) -> Router {
    Router::new()
        .nest("/api/auth", oauth_routes::oauth_routes(pool.clone()))
        .route("/", get(|| async { "Hello, World!" }))
}
