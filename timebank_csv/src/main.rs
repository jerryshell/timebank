use std::fs;
use timebank_csv::*;
use timebank_db::*;

#[tokio::main]
async fn main() {
    let pool = init_sqlite_db().await.unwrap();

    let readdir = fs::read_dir("csv_data").unwrap();
    let mut csv_path_vec = vec![];
    for entry in readdir {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        match path.extension() {
            None => continue,
            Some(extension) => {
                if extension == "csv" {
                    csv_path_vec.push(path.to_str().unwrap().to_owned())
                }
            }
        }
    }

    for csv_path in csv_path_vec.iter() {
        println!("csv_path {:?}", csv_path);
        let record_vec = generate_record_vec_by_csv_path(csv_path).unwrap();
        insert_record_vec(&pool, &record_vec).await.unwrap()
    }
}
