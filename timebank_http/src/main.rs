#[tokio::main]
async fn main() {
    // init tracing
    let file_appender = tracing_appender::rolling::daily("./", "timebank_http.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    // admin token
    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    tracing::info!(admin_token);

    // init db
    let db_pool = timebank_db::init_sqlite_db()
        .await
        .expect("timebank_db::init_sqlite_db() err");

    // init db bakcup scheduler
    std::thread::spawn(timebank_http::db_backup_scheduler_start);

    // init app state
    let app_state = timebank_http::SharedState::default();

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
        .route(
            "/record/delete/:id",
            axum::routing::delete(timebank_http::record_delete_by_id),
        )
        .route("/state", axum::routing::get(timebank_http::state))
        .layer(cors)
        .layer(axum::Extension(db_pool))
        .with_state(app_state);

    // init ip addr
    let ip_addr = std::env::var("IP_ADDR")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)));

    // init port
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20000);
    tracing::info!("port {}", port);

    // init socket addr
    let socket_addr = std::net::SocketAddr::new(ip_addr, port);
    tracing::info!("socket_addr {}", socket_addr);

    // run app
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .expect("axum serve err");
}
