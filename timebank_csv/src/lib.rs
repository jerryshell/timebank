use serde::Deserialize;
use timebank_core::*;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CsvRow {
    #[serde(rename = "Time")]
    pub time_str: String,
    #[serde(rename = "Type")]
    pub type_str: String,
    #[serde(rename = "Remark")]
    pub remark: String,
}

pub fn generate_record_vec_by_csv_path(csv_path: &str) -> Result<Vec<Record>, String> {
    let csv_filename = csv_path.split('/').last().unwrap();

    let date_str = csv_filename.split('.').next().unwrap();

    let mut record_vec: Vec<Record> = vec![];

    let mut reader = csv::Reader::from_path(csv_path).unwrap();
    for result in reader.deserialize() {
        let csv_row: CsvRow = result.unwrap();

        let Ok(time_index_range) =  hhmmhhmm_to_time_index_range(&csv_row.time_str) else {
            println!("invalid time_str: {}", csv_row.time_str);
            continue;
        };

        let sub_record = Record {
            date: date_str.to_string(),
            time_index_begin: time_index_range.0,
            time_index_end: time_index_range.1,
            type_str: csv_row.type_str,
            remark: csv_row.remark,
        };

        let Ok(mut sub_record_vec) = generate_record_vec(&sub_record) else {
            println!("generate_record_vec error {:?}", sub_record);
            continue;
        };

        record_vec.append(&mut sub_record_vec)
    }

    Ok(record_vec)
}
