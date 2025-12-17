mod audit;
mod db;
mod handlers;
mod models;
mod stv;

use axum::http::{
    header::{self, HeaderValue},
    Method,
};
use axum::{
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, net::SocketAddr};
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing_subscriber;

use handlers::{
    get_stv_results,
    participant::{join_wentu, update_preferences},
    wentu::{close_wentu, create_wentu, get_wentu, AppState},
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

    // Configure CORS with specific allowed origins
    let allowed_origins = env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173,http://127.0.0.1:5173".to_string());

    let origins: Vec<_> = allowed_origins
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any);

    // Configure global rate limiting: 100 requests per minute per IP
    let rate_limit_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(600) // 100 per minute = 600ms per request
            .burst_size(10) // Allow bursts of 10
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    let rate_limit_layer = GovernorLayer {
        config: rate_limit_config,
    };

    // Additional stricter rate limiting for write-heavy endpoints
    let write_rate_limit_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(666) // ~1.5 requests per second sustained
            .burst_size(3) // Small burst allowance
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    let write_rate_limit_layer = GovernorLayer {
        config: write_rate_limit_config,
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route(
            "/api/wentu",
            post(create_wentu).layer(write_rate_limit_layer.clone()),
        )
        .route("/api/wentu/:slug", get(get_wentu))
        .route(
            "/api/wentu/:slug/close",
            post(close_wentu).layer(write_rate_limit_layer.clone()),
        )
        .route(
            "/api/wentu/:slug/join",
            post(join_wentu).layer(write_rate_limit_layer.clone()),
        )
        .route(
            "/api/wentu/:slug/preferences",
            post(update_preferences).layer(write_rate_limit_layer),
        )
        .route("/api/wentu/:slug/stv-results", get(get_stv_results))
        // Security headers
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_XSS_PROTECTION,
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(
                "default-src 'self'; \
                 script-src 'self' 'unsafe-inline'; \
                 style-src 'self' 'unsafe-inline'; \
                 img-src 'self' data: https:; \
                 font-src 'self'; \
                 connect-src 'self'; \
                 frame-ancestors 'none';",
            ),
        ))
        .layer(rate_limit_layer)
        .layer(cors)
        .with_state(state);

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    let enable_https = env::var("ENABLE_HTTPS")
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE"))
        .unwrap_or(false);

    if enable_https {
        let cert_path = env::var("TLS_CERT_PATH").map(PathBuf::from).map_err(|_| {
            Error::new(
                ErrorKind::Other,
                "TLS_CERT_PATH is required when ENABLE_HTTPS=true",
            )
        })?;
        let key_path = env::var("TLS_KEY_PATH").map(PathBuf::from).map_err(|_| {
            Error::new(
                ErrorKind::Other,
                "TLS_KEY_PATH is required when ENABLE_HTTPS=true",
            )
        })?;

        let tls_config = RustlsConfig::from_pem_file(cert_path, key_path).await?;
        tracing::info!("Server running with TLS on https://{}", addr);

        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await?;
    } else {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("Server running on http://{}", addr);

        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
    }

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
