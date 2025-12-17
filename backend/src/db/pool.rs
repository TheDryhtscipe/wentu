use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tracing::info;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20); // Default to 20

    let min_connections: u32 = env::var("DATABASE_MIN_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5); // Default to 5

    let acquire_timeout_ms: u64 = env::var("DATABASE_ACQUIRE_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5_000);

    let idle_timeout_secs: u64 = env::var("DATABASE_IDLE_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(600);

    let max_lifetime_secs: u64 = env::var("DATABASE_MAX_LIFETIME_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_800);

    info!(
        "Configuring database pool (min={} max={} acquire_timeout_ms={} idle_timeout_secs={} max_lifetime_secs={})",
        min_connections, max_connections, acquire_timeout_ms, idle_timeout_secs, max_lifetime_secs
    );

    PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_millis(acquire_timeout_ms))
        .idle_timeout(Duration::from_secs(idle_timeout_secs)) // Default 10 minutes
        .max_lifetime(Duration::from_secs(max_lifetime_secs)) // Default 30 minutes
        .connect(database_url)
        .await
}
