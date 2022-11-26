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

pub fn generate_record_vec_by_csv_row(
    date_str: &str,
    time_index_begin: u32,
    time_index_end: u32,
    type_str: &str,
    remark: &str,
) -> Result<Vec<Record>, String> {
    let mut record_vec: Vec<Record> = vec![];
    for time_index in time_index_begin..time_index_end {
        let record = Record {
            date: date_str.to_string(),
            time_index_begin: time_index,
            time_index_end: time_index + 1,
            type_str: type_str.to_string(),
            remark: remark.to_string(),
        };
        record_vec.push(record);
    }

    Ok(record_vec)
}

pub fn generate_record_vec_by_csv_path(csv_path: &str) -> Result<Vec<Record>, String> {
    let csv_filename = csv_path.split('/').last().unwrap();
    // println!("csv_filename {:?}", csv_filename);

    let date_str = csv_filename.split('.').next().unwrap();
    // println!("date_str {:?}", date_str);

    let mut record_vec: Vec<Record> = vec![];

    let mut reader = csv::Reader::from_path(csv_path).unwrap();
    for result in reader.deserialize() {
        let csv_row: CsvRow = result.unwrap();
        // println!(
        //     "{} {} {}",
        //     csv_row.time_str, csv_row.type_str, csv_row.remark
        // );

        let Ok(time_index_range) =  hhmmhhmm_to_time_index_range(&csv_row.time_str) else {
            println!("invalid time_str: {}", csv_row.time_str);
            continue;
        };
        // println!("{:?}", time_index_range);

        let Ok(mut sub_record_vec) = generate_record_vec_by_csv_row(
            date_str,
            time_index_range.0,
            time_index_range.1,
            &csv_row.type_str,
            &csv_row.remark,
        ) else {
            println!("generate_record_vec error {:?}", csv_row);
            continue;
        };

        record_vec.append(&mut sub_record_vec)
    }

    Ok(record_vec)
}
