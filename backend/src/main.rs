mod models;
mod handlers;
mod db;
mod stv;

use axum::{
    routing::{get, post},
    Router,
};
use std::env;
use tracing_subscriber;
use tower_http::cors::CorsLayer;

use handlers::{
    wentu::{create_wentu, get_wentu, close_wentu, AppState},
    participant::{join_wentu, update_preferences},
    get_stv_results,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load env vars
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://wentu:wentu@localhost:5432/wentu".to_string());

    // Create database pool
    let pool = db::create_pool(&database_url).await?;

    // Run migrations
    db::run_migrations(&pool).await?;

    let state = AppState { db: pool };

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/wentu", post(create_wentu))
        .route("/api/wentu/:slug", get(get_wentu))
        .route("/api/wentu/:slug/close", post(close_wentu))
        .route("/api/wentu/:slug/join", post(join_wentu))
        .route("/api/wentu/:slug/preferences", post(update_preferences))
        .route("/api/wentu/:slug/stv-results", get(get_stv_results))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
