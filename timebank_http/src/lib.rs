use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use timebank_core::Record;

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

pub async fn record_list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Record>>, String> {
    match timebank_db::get_record_list(&state.pool).await {
        Ok(record_list) => Ok(Json(record_list)),
        Err(e) => Err(e),
    }
}

pub async fn record_search(
    State(state): State<Arc<AppState>>,
    Json(form): Json<SearchForm>,
) -> Result<Json<Vec<Record>>, String> {
    match timebank_db::search_record(&state.pool, &form.date_begin, &form.date_end).await {
        Ok(record_list) => Ok(Json(record_list)),
        Err(e) => Err(e),
    }
}

pub async fn record_create(
    State(state): State<Arc<AppState>>,
    Json(record): Json<Record>,
) -> Result<Json<Vec<Record>>, String> {
    match timebank_core::generate_record_vec(&record) {
        Ok(record_vec) => match timebank_db::insert_record_vec(&state.pool, &record_vec).await {
            Ok(_) => Ok(Json(record_vec)),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
