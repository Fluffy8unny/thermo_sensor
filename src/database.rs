use rusqlite::Connection;
use tokio::sync::mpsc;

use std::time::UNIX_EPOCH;

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
    let time_serializeable = reading
        .time_stamp
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    conn.execute(
        "insert into readings (id,time_stamp,device_name,temperature,humidity) values (NULL,?1, ?2, ?3, ?4)",
        (
            time_serializeable,
            reading.device_name,
            reading.temperature,
            reading.humidity,
        ),
    )?;

    Ok(())
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
