use axum::routing::post;
use axum::{routing::get, Router};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use timebank_http::*;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, instrument};

#[instrument]
#[tokio::main]
async fn main() {
    let pool = timebank_db::init_sqlite_db()
        .await
        .expect("timebank_db::init_sqlite_db() err");

    let app_state = Arc::new(Mutex::new(AppState {
        pool,
        ip_to_admin_token_error_count_map: HashMap::new(),
    }));

    tracing_subscriber::fmt::init();

    tokio::spawn(async { db_backup_scheduler_start().await });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/record/list", get(record_list))
        .route("/record/search", post(record_search))
        .route("/record/create", post(record_create))
        .with_state(app_state)
        .layer(cors);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    info!(port);

    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    info!(admin_token);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("addr={}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("axum::Server::bind().serve() err");
}
