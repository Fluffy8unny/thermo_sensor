mod config;
use config::parse_config;

mod bluetooth;
use bluetooth::start_bluetooth_thread;

mod reading;
use reading::Reading;
fn extract_temp_and_humidity(data: Vec<u8>) -> Result<Reading,Box<dyn std::error::Error>> {
    let t = u16::from_le_bytes(data[3..5].try_into()?);
    let h = data[5];
    Ok(Reading {
        temperature: t,
        humidity: h,
    })
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "config.toml";
    let device_info = parse_config(file_name)?;
    
    let adapter_tasks = start_bluetooth_thread(device_info,&extract_temp_and_humidity).await.unwrap();
    let _ = adapter_tasks.await; //loop is running
    Ok(())
}
