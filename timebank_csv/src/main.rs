use glob::glob;
use timebank_csv::*;
use timebank_db::*;
use tracing::{info, instrument};

#[instrument]
#[tokio::main]
async fn main() {
    let pool = init_sqlite_db().await.expect("init_sqlite_db() err");

    tracing_subscriber::fmt::init();

    for entry in glob("csv_data/*.csv").expect("glob() error") {
        match entry {
            Ok(csv_path) => {
                info!("csv_path={:?}", csv_path);
                let record_vec = generate_record_vec_by_csv_path(&csv_path)
                    .expect("generate_record_vec_by_csv_path() err");
                insert_record_vec(&pool, &record_vec)
                    .await
                    .expect("insert_record_vec() err")
            }
            Err(e) => info!("e={}", e),
        }
    }
}
