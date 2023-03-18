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
                let record_vec = timebank_csv::generate_record_vec_by_csv_path(&csv_path)
                    .expect("generate_record_vec_by_csv_path() err");
                timebank_db::insert_record_vec(&pool, &record_vec)
                    .await
                    .expect("insert_record_vec() err")
            }
            Err(e) => tracing::info!("e={}", e),
        }
    }
}
