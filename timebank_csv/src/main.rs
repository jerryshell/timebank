#[tokio::main]
async fn main() {
    let pool = timebank_db::init_sqlite_db()
        .await
        .expect("init_sqlite_db() err");

    tracing_subscriber::fmt::init();

    for entry in glob::glob("csv_data/*.csv").expect("glob() error") {
        match entry {
            Ok(csv_path) => {
                tracing::info!("csv_path={:?}", csv_path);
                let record_list = timebank_csv::generate_record_list_by_csv_path(&csv_path);
                timebank_db::insert_record_list(&pool, &record_list)
                    .await
                    .expect("insert_record_list() err")
            }
            Err(e) => tracing::info!("e={}", e),
        }
    }
}
