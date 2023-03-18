#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SearchForm {
    #[serde(rename = "dateBegin")]
    date_begin: String,
    #[serde(rename = "dateEnd")]
    date_end: String,
}

pub async fn health() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

pub async fn record_list(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    match timebank_db::get_record_list(&db_pool).await {
        Ok(record_list) => (
            axum::http::StatusCode::OK,
            axum::Json(serde_json::json!(record_list)),
        ),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({ "message": e })),
        ),
    }
}

pub async fn record_search(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    axum::Json(form): axum::Json<SearchForm>,
) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    match timebank_db::search_record(&db_pool, &form.date_begin, &form.date_end).await {
        Ok(record_list) => (
            axum::http::StatusCode::OK,
            axum::Json(serde_json::json!(record_list)),
        ),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({ "message": e })),
        ),
    }
}

pub async fn record_create(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    axum::Extension(mut ip_to_admin_token_error_count_map): axum::Extension<
        std::collections::HashMap<String, usize>,
    >,
    axum::extract::ConnectInfo(connect_info): axum::extract::ConnectInfo<std::net::SocketAddr>,
    request_header_map: axum::http::HeaderMap,
    axum::Json(record): axum::Json<timebank_core::Record>,
) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    tracing::info!("request_header_map {:?}", request_header_map);

    let header_value_iter = request_header_map.get_all("x-forwarded-for").iter();
    let client_ip = match header_value_iter.last() {
        None => connect_info.ip().to_string(),
        Some(header_value) => match header_value.to_str() {
            Ok(header_value_str) => header_value_str.to_string(),
            Err(_) => connect_info.ip().to_string(),
        },
    };
    tracing::info!("client_ip {client_ip}");

    let admin_token_error_count = ip_to_admin_token_error_count_map
        .get(&client_ip)
        .unwrap_or(&0);
    tracing::info!("admin_token_error_count {admin_token_error_count}");

    if *admin_token_error_count >= 3 {
        return (
            axum::http::StatusCode::FORBIDDEN,
            axum::Json(serde_json::json!({
                "message": format!("This ip has been banned: {client_ip}")
            })),
        );
    }

    let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or("admin_token".to_string());
    tracing::info!("admin_token {admin_token}");

    let admin_token_from_request = match request_header_map.get("admin_token") {
        Some(value) => value.to_str().unwrap_or(""),
        None => {
            ip_to_admin_token_error_count_map.insert(client_ip, *admin_token_error_count + 1);
            return (
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({ "message": "admin_token bank" })),
            );
        }
    };
    tracing::info!("admin_token_from_request {admin_token_from_request}");

    if admin_token != admin_token_from_request {
        ip_to_admin_token_error_count_map.insert(client_ip, *admin_token_error_count + 1);
        return (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({ "message": "admin_token invalid" })),
        );
    }

    let record_vec = timebank_core::generate_record_vec(&record);
    match timebank_db::insert_record_vec(&db_pool, &record_vec).await {
        Ok(_) => (
            axum::http::StatusCode::OK,
            axum::Json(serde_json::json!(record_vec)),
        ),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({ "message": e })),
        ),
    }
}

pub async fn db_backup_scheduler_start() {
    let mut sched = job_scheduler::JobScheduler::new();

    let cron = "0 0 0 * * * *";

    sched.add(job_scheduler::Job::new(
        cron.parse().expect("cron.parse() err"),
        || {
            match timebank_db::db_backup() {
                Ok(db_backup_filename) => tracing::info!(
                    "db_backup_scheduler ok db_backup_filename={}",
                    db_backup_filename
                ),
                Err(e) => tracing::warn!("db_backup_scheduler err e={}", e),
            };
        },
    ));

    tracing::info!("cron {cron}");

    loop {
        sched.tick();

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
