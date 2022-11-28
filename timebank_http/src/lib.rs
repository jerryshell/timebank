use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use timebank_core::Record;
use tracing::info;

pub struct AppState {
    pub pool: Pool<Sqlite>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchForm {
    #[serde(rename = "dateBegin")]
    date_begin: String,
    #[serde(rename = "dateEnd")]
    date_end: String,
}

pub async fn record_list(State(app_state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    match timebank_db::get_record_list(&app_state.pool).await {
        Ok(record_list) => (StatusCode::OK, Json(json!(record_list))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}

pub async fn record_search(
    State(app_state): State<Arc<AppState>>,
    Json(form): Json<SearchForm>,
) -> (StatusCode, Json<Value>) {
    match timebank_db::search_record(&app_state.pool, &form.date_begin, &form.date_end).await {
        Ok(record_list) => (StatusCode::OK, Json(json!(record_list))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}

pub async fn record_create(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(record): Json<Record>,
) -> (StatusCode, Json<Value>) {
    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    info!("admin_token {:#?}", admin_token);

    let admin_token_from_request = match headers.get("admin_token") {
        Some(value) => value.to_str().unwrap(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "admin_token bank" })),
            )
        }
    };
    info!("admin_token_from_request {:#?}", admin_token_from_request);

    if admin_token != admin_token_from_request {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "admin_token invalid" })),
        );
    }

    match timebank_core::generate_record_vec(&record) {
        Ok(record_vec) => {
            match timebank_db::insert_record_vec(&app_state.pool, &record_vec).await {
                Ok(_) => (StatusCode::OK, Json(json!(record_vec))),
                Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
            }
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}
