use axum::routing::post;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use timebank_http::*;

#[tokio::main]
async fn main() {
    let pool = timebank_db::init_sqlite_db().await.unwrap();

    let shared_state = Arc::new(AppState { pool });

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/record/list", get(get_record_list))
        .route("/record/search", post(search))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
