use chrono::{DateTime, Utc};
use rusqlite::{Connection, Error, MappedRows, Row};
use tokio::sync::mpsc;

use crate::config;
use crate::{DeviceName, Reading};

fn assert_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "create table if not exists readings (
             id          integer primary key,
             time_stamp  text not null,
             device_id   integer not null,
             temperature integer not null,
             humidity    integer not null
         )",
        [],
    )?;

    conn.execute(
        "create table if not exists devices(
            id integer primary key,
            name text not null,
            nickname text,
            UNIQUE(name))",
        [],
    )?;
    Ok(())
}

fn insert_into_table(
    conn: &Connection,
    reading: Reading,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "insert or ignore into devices
              (id,name,nickname) values (NULL,?1,NULL)",
        (reading.device_name.name.clone(),),
    )?;

    conn.execute(
        "insert into readings
             (id,time_stamp,device_id,temperature,humidity) values (NULL,?1, (select id from devices where name=?2) , ?3, ?4)",
        (
            reading.time_stamp,
            reading.device_name.name.clone(),
            reading.temperature,
            reading.humidity,
        ),
    )?;
    Ok(())
}

pub fn update_nickname(
    config: config::DatabaseConfig,
    device_name: &str,
    device_nickname: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    conn.execute(
        "update devices 
                       set nickname =?1
                       where name = ?2",
        [device_nickname, device_name],
    )?;
    Ok(())
}

impl Reading {
    fn from_row(row: &Row<'_>) -> Result<Reading, Error> {
        Ok(Reading {
            time_stamp: row.get(0)?,
            device_name: DeviceName {
                name: row.get(1)?,
                nickname: row.get(2).unwrap_or_default(),
            },
            temperature: row.get(3)?,
            humidity: row.get(4)?,
        })
    }
}

pub fn convert_date_time(time: DateTime<Utc>) -> String {
    time.format("%F %T%.f%:z").to_string()
}

pub fn get_all_devices(
    config: config::DatabaseConfig,
    time_limit: DateTime<Utc>,
) -> Result<Vec<DeviceName>, Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    let time_string = convert_date_time(time_limit);
    let mut stmt = conn.prepare(
        "select distinct d.name,d.nickname
               from readings r inner join devices d on r.device_id = d.id
               where r.time_stamp > datetime(?1)",
    )?;
    let reading_iter = stmt.query_map([time_string], |row| {
        Ok(DeviceName {
            name: row.get(0)?,
            nickname: row.get(1)?,
        })
    })?;

    Ok(extract_data(reading_iter))
}

fn extract_data<T>(itt: MappedRows<'_, impl FnMut(&Row<'_>) -> Result<T, Error>>) -> Vec<T> {
    itt.filter(|maybe_ok| maybe_ok.is_ok())
        .map(|ok| ok.unwrap())
        .collect()
}

pub fn get_all_readings_for_device(
    config: config::DatabaseConfig,
    device_name: &str,
    time_limit: DateTime<Utc>,
) -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    let time_string = convert_date_time(time_limit);
    let mut stmt = conn.prepare("select  r.time_stamp,d.name, d.nickname,r.temperature,r.humidity 
                                                     from  readings r inner join devices d on r.device_id = d.id
                                                     where  d.name =?1 and r.time_stamp > datetime(?2)")?;
    let reading_iter = stmt.query_map([device_name, &time_string], |row| Reading::from_row(row))?;
    Ok(extract_data(reading_iter))
}

pub fn get_newest_readings(
    config: config::DatabaseConfig,
    time_limit: DateTime<Utc>,
) -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let conn = Connection::open(config.file_name)?;
    let time_string = convert_date_time(time_limit);
    let mut stmt =
        conn.prepare("select  r.time_stamp,d.name, d.nickname,r.temperature,r.humidity, max(time_stamp) as latest
                            from  readings r inner join devices d on r.device_id = d.id
                            where r.time_stamp > datetime(?1)
                            group by d.name")?;
    let reading_iter = stmt.query_map([time_string], |row| Reading::from_row(row))?;
    Ok(extract_data(reading_iter))
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
