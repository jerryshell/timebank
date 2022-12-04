use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteRow},
    Pool, Row, Sqlite,
};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};
use timebank_core::*;

pub async fn init_sqlite_db() -> Result<Pool<Sqlite>, String> {
    let pool = match SqlitePoolOptions::new()
        .connect("sqlite:timebank.sqlite?mode=rwc")
        .await
    {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    if let Err(e) = sqlx::query(
        "create table if not exists record (
             date text,
             time_index_begin integer,
             time_index_end integer,
             type_str text,
             remark text,
             primary key(date, time_index_begin, time_index_end)
         )",
    )
    .execute(&pool)
    .await
    {
        return Err(e.to_string());
    };

    Ok(pool)
}

pub async fn insert_record(pool: &Pool<Sqlite>, record: &Record) -> Result<(), String> {
    if let Err(e) = sqlx::query(
        "insert or replace into record (date, time_index_begin, time_index_end, type_str, remark) values (?, ?, ?, ?, ?)"
    )
    .bind(record.date.to_string())
    .bind(record.time_index_begin.to_string())
    .bind(record.time_index_end.to_string())
    .bind(record.type_str.to_string())
    .bind(record.remark.to_string())
    .execute(pool).await {
        return Err(e.to_string());
    };
    Ok(())
}

pub async fn insert_record_vec(pool: &Pool<Sqlite>, record_vec: &[Record]) -> Result<(), String> {
    for record in record_vec.iter() {
        insert_record(pool, record).await?;
    }
    Ok(())
}

fn sqlite_row_to_record(row: SqliteRow) -> Record {
    let date = row.get("date");
    let time_index_begin = row.get("time_index_begin");
    let time_index_end = row.get("time_index_end");
    let type_str = row.get("type_str");
    let remark = row.get("remark");
    Record {
        date,
        time_index_begin,
        time_index_end,
        type_str,
        remark,
    }
}

pub async fn get_record_list(pool: &Pool<Sqlite>) -> Result<Vec<Record>, String> {
    match sqlx::query("select * from record order by date desc, time_index_end desc")
        .map(sqlite_row_to_record)
        .fetch_all(pool)
        .await
    {
        Ok(record_list) => Ok(record_list),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn search_record(
    pool: &Pool<Sqlite>,
    date_begin: &str,
    date_end: &str,
) -> Result<Vec<Record>, String> {
    match sqlx::query(
        "select * from record where date between ? and ? order by date desc, time_index_end desc",
    )
    .bind(date_begin)
    .bind(date_end)
    .map(sqlite_row_to_record)
    .fetch_all(pool)
    .await
    {
        Ok(record_list) => Ok(record_list),
        Err(e) => Err(e.to_string()),
    }
}

pub fn db_backup() -> Result<String, String> {
    let duration_since_unix_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime::now().duration_since() err")
        .as_secs();
    let db_backup_filename = format!("timebank.{duration_since_unix_epoch}.sqlite");
    if let Err(e) = fs::copy("timebank.sqlite", db_backup_filename.clone()) {
        return Err(e.to_string());
    };
    Ok(db_backup_filename)
}
