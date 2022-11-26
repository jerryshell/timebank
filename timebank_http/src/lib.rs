use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

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

pub async fn get_record_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match timebank_db::get_record_list(&state.pool).await {
        Ok(record_list) => Json(record_list),
        Err(_) => Json(vec![]),
    }
}

pub async fn search(
    State(state): State<Arc<AppState>>,
    Json(form): Json<SearchForm>,
) -> impl IntoResponse {
    match timebank_db::search_record(&state.pool, &form.date_begin, &form.date_end).await {
        Ok(record_list) => Json(record_list),
        Err(_) => Json(vec![]),
    }
}
