mod api;
mod github;

use github::bot::Bot;

use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = api::repository::establish_connection();

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = api::create_routes(pool).layer(cors);

    let server_task = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("Erreur d'ouverture du port 3000");

        tracing::info!(
            "🚀 Serveur en écoute sur {}",
            listener.local_addr().unwrap()
        );

        axum::serve(listener, app)
            .await
            .expect("Erreur lors du démarrage du serveur");
    });

    tokio::join!(server_task);
}
