use axum::routing::post;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use timebank_http::*;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    let pool = timebank_db::init_sqlite_db().await.unwrap();

    let app_state = Arc::new(AppState { pool });

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
    info!("port: {}", port);

    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    info!("admin_token: {}", admin_token);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("address on: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
