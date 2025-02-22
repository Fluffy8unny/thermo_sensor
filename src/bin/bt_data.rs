use chrono::Utc;
use thermo_sensor::{
    parse_config, start_bluetooth_thread, start_database_thread, DeviceName, Reading,
};
use tokio::sync::mpsc;

///Implementation of a function that extracts temperature and huimidity from the BT response
///
///This is the implementation that takes the raw response of tghe bluetooth device and 
///parses it. If you want to use a different device you'll probably have to change it.
///For some godforsaken reason the temperature is encoded in the concatinated bytes 3 and 4
///where the resulting uint16 represents the temperature*10 e.g. a result of 255 means a 
///temeperature of 25.5 centigrade.
///
///* `data` bluetooth response, a vector of raw data
///* `name` identifier of the device
///
///* returns a reading instance containting the temperature and humidity that was encoded in the
///result. Time is set as the current timestamp, not the time of the sending....because honestly it 
///doens't matter for our purpose.
fn extract_temp_and_humidity(
    data: Vec<u8>,
    name: String,
) -> Result<Reading, Box<dyn std::error::Error>> {
    //for some reason once in a blue moon the data is too short.
    //I have no idea why this happens
    //plz send help
    if data.len() < 5 {
        return Err(Box::from(format!("data corrupted, this happens once every few hours.\n Got {:?}. \n If you have any idea why this happens contact me please.", data)));
    }
    let t = u16::from_le_bytes(data[3..5].try_into()?);
    let h = data[5];
    let now = Utc::now();
    Ok(Reading {
        temperature: t,
        humidity: h,
        device_name: DeviceName {
            name: name.to_string(),
            nickname: None,
        },
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
    adapter_tasks.await?; //loop is running
    database_task.await?;
    Ok(())
}
