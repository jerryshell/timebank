use std::fs;
use timebank::*;

fn main() {
    let connection = init_sqlite_db().unwrap();

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
    // println!("csv_path_vec {:?}", csv_path_vec);

    for csv_path in csv_path_vec.iter() {
        println!("csv_path {:?}", csv_path);
        insert_by_csv_path(&connection, csv_path).unwrap();
    }
}
