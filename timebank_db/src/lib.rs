use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
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
             date date,
             time_index_begin integer,
             time_index_end integer,
             type_str text,
             remark text
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
    .bind(record.date)
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
