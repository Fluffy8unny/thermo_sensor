mod config;
use config::parse_config;

mod bluetooth;
use bluetooth::start_bluetooth_thread;

mod reading;
use reading::Reading;

mod database;
use database::start_database_thread;

use std::time::SystemTime;
use tokio::sync::mpsc;

fn extract_temp_and_humidity(
    data: Vec<u8>,
    name: String,
) -> Result<Reading, Box<dyn std::error::Error>> {
    let t = u16::from_le_bytes(data[3..5].try_into()?);
    let h = data[5];
    let now = SystemTime::now();
    Ok(Reading {
        temperature: t,
        humidity: h,
        device_name: name,
        time_stamp: now,
    })
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "config.toml";
    let config = parse_config(file_name)?;

    let (tx, rx) = mpsc::unbounded_channel::<Reading>();
    let adapter_tasks = start_bluetooth_thread(config.bt_config, tx, &extract_temp_and_humidity)
        .await
        .unwrap();
    let database_task = start_database_thread(config.db_config, rx).await.unwrap();
    let _ = adapter_tasks.await; //loop is running
    let _ = database_task.await;
    Ok(())
}
