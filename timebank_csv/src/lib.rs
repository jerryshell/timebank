#[derive(Debug, serde::Deserialize)]
pub struct CsvRow {
    #[serde(rename = "Time")]
    pub time_str: String,
    #[serde(rename = "Type")]
    pub type_str: String,
    #[serde(rename = "Remark")]
    pub remark: String,
}

pub fn generate_record_vec_by_csv_path(
    csv_path: &std::path::PathBuf,
) -> Result<Vec<timebank_core::Record>, String> {
    let csv_filename = csv_path
        .file_name()
        .expect("csv_path.file_name() err")
        .to_str()
        .expect("csv_path.file_name().to_str() err");

    let date_str = csv_filename
        .split('.')
        .next()
        .expect("csv_filename.split().next() err");

    let mut record_vec: Vec<timebank_core::Record> = vec![];

    let mut reader = csv::Reader::from_path(csv_path).expect("csv::Reader::from_path() err");
    for result in reader.deserialize() {
        let csv_row: CsvRow = result.expect("reader.deserialize().result err");

        let Ok(time_index_range) =  timebank_core::hhmmhhmm_to_time_index_range(&csv_row.time_str) else {
            tracing::warn!("invalid csv_row.time_str={}", csv_row.time_str);
            continue;
        };

        let sub_record = timebank_core::Record {
            date: date_str.to_string(),
            time_index_begin: time_index_range.0,
            time_index_end: time_index_range.1,
            type_str: csv_row.type_str,
            remark: csv_row.remark,
        };

        let mut sub_record_vec = timebank_core::generate_record_vec(&sub_record);

        record_vec.append(&mut sub_record_vec)
    }

    Ok(record_vec)
}
