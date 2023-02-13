#[tokio::main]
async fn main() {
    // init db
    let pool = timebank_db::init_sqlite_db()
        .await
        .expect("timebank_db::init_sqlite_db() err");

    // init tracing
    tracing_subscriber::fmt::init();

    // init db bakcup scheduler
    tokio::spawn(async { timebank_http::db_backup_scheduler_start().await });

    // init app state
    let app_state = std::sync::Arc::new(tokio::sync::Mutex::new(timebank_http::AppState {
        pool,
        ip_to_admin_token_error_count_map: std::collections::HashMap::new(),
    }));

    // cors
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // init route
    let app = axum::Router::new()
        .route("/health", axum::routing::get(timebank_http::health))
        .route(
            "/record/list",
            axum::routing::get(timebank_http::record_list),
        )
        .route(
            "/record/search",
            axum::routing::post(timebank_http::record_search),
        )
        .route(
            "/record/create",
            axum::routing::post(timebank_http::record_create),
        )
        .with_state(app_state)
        .layer(cors);

    // init port
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    tracing::info!(port);

    // admin token
    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    tracing::info!(admin_token);

    // run app
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("addr={}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .expect("axum::Server::bind().serve() err");
}
