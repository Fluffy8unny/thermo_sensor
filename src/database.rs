use rusqlite::Connection;
use tokio::sync::mpsc;

use chrono::{DateTime, Utc};

use crate::config;
use crate::Reading;

fn assert_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "create table if not exists readings (
             id          integer primary key,
             time_stamp  integer not null,
             device_name text not null,
             temperature integer not null,
             humidity    integer not null
         )",
        [],
    )?;
    Ok(())
}

fn insert_into_table(
    conn: &Connection,
    reading: Reading,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "insert into readings (id,time_stamp,device_name,temperature,humidity) values (NULL,?1, ?2, ?3, ?4)",
        (
            reading.time_stamp,
            reading.device_name,
            reading.temperature,
            reading.humidity,
        ),
    )?;

    Ok(())
}

//select distinct device_name from readings where time_stamp > 0
//select  * from readings where time_stamp > 0 and device_name ="TP357S (B88E)"
//select *, max(time_stamp) as latest from readings group by device_name
struct DeviceName {
    name: String,
}
pub fn get_all_devices(
    config: config::DatabaseConfig,
    _time_limit: DateTime<Utc>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    let mut stmt =
        conn.prepare("select distinct device_name from readings where time_stamp > 0")?;
    let reading_iter = stmt.query_map([], |row| Ok(DeviceName { name: row.get(0)? }))?;
    let res: Vec<String> = reading_iter
        .filter(|maybe_ok| maybe_ok.is_ok())
        .map(|ok| ok.unwrap().name)
        .collect();

    Ok(res)
}

pub fn get_all_readings_for_device(
    config: config::DatabaseConfig,
    device_name: &str,
    _time_limit: DateTime<Utc>,
) -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    let mut stmt = conn.prepare("select  * from readings where  device_name =?1")?;
    let reading_iter = stmt.query_map([device_name], |row| {
        Ok(Reading {
            time_stamp: row.get(1)?,
            device_name: row.get(2)?,
            temperature: row.get(3)?,
            humidity: row.get(4)?,
        })
    })?;
    let res: Vec<Reading> = reading_iter
        .filter(|maybe_ok| maybe_ok.is_ok())
        .map(|ok| ok.unwrap())
        .collect();
    Ok(res)
}

pub fn get_newest_readings(
    config: config::DatabaseConfig,
    _time_limit: DateTime<Utc>,
) -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    let mut stmt =
        conn.prepare("select *, max(time_stamp) as latest from readings group by device_name")?;
    let reading_iter = stmt.query_map([], |row| {
        Ok(Reading {
            time_stamp: row.get(1)?,
            device_name: row.get(2)?,
            temperature: row.get(3)?,
            humidity: row.get(4)?,
        })
    })?;
    let res: Vec<Reading> = reading_iter
        .filter(|maybe_ok| maybe_ok.is_ok())
        .map(|ok| ok.unwrap())
        .collect();
    Ok(res)
}

async fn receive_data(
    config: config::DatabaseConfig,
    mut rx: mpsc::UnboundedReceiver<Reading>,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    assert_table(&conn)?;

    loop {
        match rx.recv().await {
            None => return Err("connecting between threads fucked".into()),
            Some(reading) => {
                insert_into_table(&conn, reading)?;
            }
        }
    }
}

pub async fn start_database_thread(
    config: config::DatabaseConfig,
    rx: mpsc::UnboundedReceiver<Reading>,
) -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error + 'static>> {
    Ok(tokio::spawn(async move {
        let _ = receive_data(config, rx).await;
    }))
}
