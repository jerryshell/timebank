use axum::{
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use job_scheduler::{Job, JobScheduler};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use timebank_core::Record;
use tokio::sync::Mutex;
use tracing::{info, warn};

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

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn record_list(
    State(app_state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    match timebank_db::get_record_list(&app_state.lock().await.pool).await {
        Ok(record_list) => (StatusCode::OK, Json(json!(record_list))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}

pub async fn record_search(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(form): Json<SearchForm>,
) -> (StatusCode, Json<Value>) {
    match timebank_db::search_record(
        &app_state.lock().await.pool,
        &form.date_begin,
        &form.date_end,
    )
    .await
    {
        Ok(record_list) => (StatusCode::OK, Json(json!(record_list))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}

pub async fn record_create(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(record): Json<Record>,
) -> (StatusCode, Json<Value>) {
    info!("addr {:#?}", addr);
    let client_ip = addr.ip().to_string();
    info!("client_ip {:#?}", client_ip);

    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    info!("admin_token {:#?}", admin_token);

    let admin_token_from_request = match headers.get("admin_token") {
        Some(value) => value.to_str().unwrap_or(""),
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
            match timebank_db::insert_record_vec(&app_state.lock().await.pool, &record_vec).await {
                Ok(_) => (StatusCode::OK, Json(json!(record_vec))),
                Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
            }
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}

pub async fn db_backup_scheduler_start() {
    let mut sched = JobScheduler::new();

    let cron = "0 0 0 * * * *";

    sched.add(Job::new(cron.parse().expect("cron.parse() err"), || {
        match timebank_db::db_backup() {
            Ok(db_backup_filename) => info!("db_backup_scheduler ok {}", db_backup_filename),
            Err(e) => warn!("db_backup_scheduler err {}", e),
        };
    }));

    info!("db_backup_scheduler_start {}", cron);

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
