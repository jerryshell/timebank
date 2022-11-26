use chrono::NaiveDate;
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CsvRow {
    #[serde(rename = "Time")]
    pub time_str: String,
    #[serde(rename = "Type")]
    pub type_str: String,
    #[serde(rename = "Remark")]
    pub remark: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Record {
    #[serde(rename = "date")]
    pub date: NaiveDate,
    #[serde(rename = "timeIndexBegin")]
    pub time_index_begin: usize,
    #[serde(rename = "timeIndexEnd")]
    pub time_index_end: usize,
    #[serde(rename = "type")]
    pub type_str: String,
    #[serde(rename = "remark")]
    pub remark: String,
}

pub fn hhmm_to_time_index(hhmm: &str) -> Result<usize, String> {
    let hhmm_split: Vec<&str> = hhmm.split(':').collect();
    if hhmm_split.len() != 2 {
        return Err(format!("invalid(1) hhmm: {}", hhmm));
    }

    let Some(hh) = hhmm_split.first() else {
        return Err(format!("invalid(2) hhmm: {}", hhmm));
    };

    let Ok(hh_usize) = hh.parse::<usize>() else {
        return Err(format!("invalid(3) hhmm: {}", hhmm));
    };

    let mut time_index = hh_usize * 2;

    let Some(mm) = hhmm_split.last() else {
        return Err(format!("invalid(4) hhmm: {}", hhmm));
    };

    if *mm == "30" {
        time_index += 1;
    }

    Ok(time_index)
}

pub fn hhmmhhmm_to_time_index_range(hhmmhhmm: &str) -> Result<(usize, usize), String> {
    let hhmmhhmm_split: Vec<&str> = hhmmhhmm.split('-').collect();
    if hhmmhhmm_split.len() != 2 {
        return Err(format!("invalid(1) hhmmhhmm: {}", hhmmhhmm));
    }

    let Some(hhmm1) = hhmmhhmm_split.first() else {
        return Err(format!("invalid(2) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm1 {:#?}", hhmm1);

    let Ok(hhmm1_time_index) = hhmm_to_time_index(hhmm1) else {
        return Err(format!("invalid(3) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm1_time_index {:#?}", hhmm1_time_index);

    let Some(hhmm2) = hhmmhhmm_split.last() else {
        return Err(format!("invalid(4) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm2 {:#?}", hhmm2);

    let Ok(hhmm2_time_index) = hhmm_to_time_index(hhmm2) else {
        return Err(format!("invalid(5) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm2_time_index {:#?}", hhmm2_time_index);

    Ok((hhmm1_time_index, hhmm2_time_index))
}

pub fn generate_record_vec_by_csv_row(
    date_str: &str,
    time_index_begin: usize,
    time_index_end: usize,
    type_str: &str,
    remark: &str,
) -> Result<Vec<Record>, String> {
    let mut record_vec: Vec<Record> = vec![];

    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();

    for time_index in time_index_begin..time_index_end {
        let record = Record {
            date,
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

pub fn init_sqlite_db() -> Result<Connection, String> {
    let connection = match Connection::open("timebank.sqlite") {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    if let Err(e) = connection.execute(
        "create table if not exists record (
             date date,
             time_index_begin integer,
             time_index_end integer,
             type_str text,
             remark text
         )",
        [],
    ) {
        return Err(e.to_string());
    };

    Ok(connection)
}

pub fn insert_record(connection: &Connection, record: &Record) -> Result<(), String> {
    match connection.execute(
        "insert or replace into record (date, time_index_begin, time_index_end, type_str, remark) values (?1, ?2, ?3, ?4, ?5)",
        [&record.date.to_string(), &record.time_index_begin.to_string(), &record.time_index_end.to_string(), &record.type_str, &record.remark],
    ) {
        Ok(_) => Ok(()),
        Err(_) => return Err(format!("insert error {:?}", record)),
    }
}

pub fn insert_record_vec(connection: &Connection, record_vec: &[Record]) -> Result<(), String> {
    for record in record_vec.iter() {
        insert_record(connection, record)?;
    }
    Ok(())
}

pub fn insert_by_csv_path(connection: &Connection, csv_path: &str) -> Result<(), String> {
    let record_vec = generate_record_vec_by_csv_path(csv_path)?;
    insert_record_vec(connection, &record_vec)
}
