use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use std::sync::Arc;

struct AppState {
    pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let pool = timebank_db::init_sqlite_db().await.unwrap();

    let shared_state = Arc::new(AppState { pool });

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/record/list", get(get_record_list))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_record_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match timebank_db::get_record_list(&state.pool).await {
        Ok(record_list) => Json(record_list),
        Err(_) => Json(vec![]),
    }
}
