use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_client_ip::ClientIp;
use job_scheduler::{Job, JobScheduler};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};
use std::{collections::HashMap, sync::Arc, time::Duration};
use timebank_core::Record;
use tokio::sync::Mutex;
use tracing::{info, instrument, warn};

#[derive(Debug)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub ip_to_admin_token_error_count_map: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchForm {
    #[serde(rename = "dateBegin")]
    date_begin: String,
    #[serde(rename = "dateEnd")]
    date_end: String,
}

#[instrument]
pub async fn health() -> StatusCode {
    StatusCode::OK
}

#[instrument]
pub async fn record_list(
    State(app_state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    match timebank_db::get_record_list(&app_state.lock().await.pool).await {
        Ok(record_list) => (StatusCode::OK, Json(json!(record_list))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}

#[instrument]
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

#[instrument]
pub async fn record_create(
    ClientIp(client_ip): ClientIp,
    headers: HeaderMap,
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(record): Json<Record>,
) -> (StatusCode, Json<Value>) {
    let client_ip = client_ip.to_string();

    let mut app_state = app_state.lock().await;

    let ip_to_admin_token_error_count_map = &mut app_state.ip_to_admin_token_error_count_map;

    let admin_token_error_count = ip_to_admin_token_error_count_map
        .get(&client_ip)
        .unwrap_or(&0);
    info!(admin_token_error_count);

    if *admin_token_error_count >= 3 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "message": format!("This ip has been banned: {client_ip}")
            })),
        );
    }

    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    info!(admin_token);

    let admin_token_from_request = match headers.get("admin_token") {
        Some(value) => value.to_str().unwrap_or(""),
        None => {
            ip_to_admin_token_error_count_map.insert(client_ip, *admin_token_error_count + 1);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "admin_token bank" })),
            );
        }
    };
    info!(admin_token_from_request);

    if admin_token != admin_token_from_request {
        ip_to_admin_token_error_count_map.insert(client_ip, *admin_token_error_count + 1);
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

#[instrument]
pub async fn db_backup_scheduler_start() {
    let mut sched = JobScheduler::new();

    let cron = "0 0 0 * * * *";

    sched.add(Job::new(cron.parse().expect("cron.parse() err"), || {
        match timebank_db::db_backup() {
            Ok(db_backup_filename) => info!(
                "db_backup_scheduler ok db_backup_filename={}",
                db_backup_filename
            ),
            Err(e) => warn!("db_backup_scheduler err e={}", e),
        };
    }));

    info!(cron);

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
