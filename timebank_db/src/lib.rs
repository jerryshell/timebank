use sqlx::Row;

pub async fn init_sqlite_db() -> Result<sqlx::Pool<sqlx::Sqlite>, String> {
    let pool = match sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite:timebank.sqlite?mode=rwc")
        .await
    {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };

    match sqlx::query(
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
        Ok(_) => Ok(pool),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn insert_record(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    record: &timebank_core::Record,
) -> Result<(), String> {
    match sqlx::query(
        "insert or replace into record (date, time_index_begin, time_index_end, type_str, remark) values (?, ?, ?, ?, ?)"
    )
    .bind(record.date.to_string())
    .bind(record.time_index_begin.to_string())
    .bind(record.time_index_end.to_string())
    .bind(record.type_str.to_string())
    .bind(record.remark.to_string())
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn insert_record_list(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    record_list: &[timebank_core::Record],
) -> Result<(), String> {
    for record in record_list.iter() {
        insert_record(pool, record).await?;
    }
    Ok(())
}

fn sqlite_row_to_record(row: &sqlx::sqlite::SqliteRow) -> timebank_core::Record {
    let date = row.get("date");
    let time_index_begin = row.get("time_index_begin");
    let time_index_end = row.get("time_index_end");
    let type_str = row.get("type_str");
    let remark = row.get("remark");
    timebank_core::Record {
        date,
        time_index_begin,
        time_index_end,
        type_str,
        remark,
    }
}

pub async fn get_record_list(
    pool: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<Vec<timebank_core::Record>, String> {
    match sqlx::query("select * from record order by date desc, time_index_end desc")
        .map(|row| sqlite_row_to_record(&row))
        .fetch_all(pool)
        .await
    {
        Ok(record_list) => Ok(record_list),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn search_record(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    date_begin: &str,
    date_end: &str,
) -> Result<Vec<timebank_core::Record>, String> {
    match sqlx::query(
        "select * from record where date between ? and ? order by date desc, time_index_end desc",
    )
    .bind(date_begin)
    .bind(date_end)
    .map(|row| sqlite_row_to_record(&row))
    .fetch_all(pool)
    .await
    {
        Ok(record_list) => Ok(record_list),
        Err(e) => Err(e.to_string()),
    }
}

pub fn db_backup() -> Result<String, String> {
    let duration_since_unix_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("SystemTime::now().duration_since() err")
        .as_secs();
    let db_backup_filename = format!("timebank.{duration_since_unix_epoch}.sqlite");
    if let Err(e) = std::fs::copy("timebank.sqlite", db_backup_filename.clone()) {
        return Err(e.to_string());
    };
    Ok(db_backup_filename)
}
