use serde_derive::Deserialize;
use std::fs;
use uuid::Uuid;

///strcut that represents the whole system configuration
///
/// *`db_config` config for the database binary
/// *`bt_config`  the humidity at the time of measurement
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub db_config: DatabaseConfig,
    pub bt_config: BluetoothConfig,
}

///struct that represents the the configuration of the database binary
///
///*`file_name` the SQLite database lives in a file on the hdd. this the path to said file
///* `backend_ip` ip parameter userd to limit which requests are handled. Check out the actix-web
///   documentation for more info
///* `backend_port` port on which the server is listening
#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub file_name: String,
    pub backend_ip: String,
    pub backend_port: u16,
}


#[derive(Deserialize, Debug, Clone)]
pub enum DeviceSelector{
    First,
    NTh{index:usize},
    Name{substring:String},
}
///
///struict that represents all configuration required for the bluetooth probe
///this mainly means that it contains filters for which devices are interessting for us
///and information on how to poll them
///
///*`name` substring that needs to be present in the device's name field. This is usually the
///designation of the device e.g. TP357S will accept TP357S (abcd), TP357SXXXXXXx
///*`service_uuid` uuid of the service, that will be polled. Only devices that provide said service
///are accepted
#[derive(Deserialize, Debug, Clone)]
pub struct BluetoothConfig {
    pub name: String,
    pub service_uuid: Uuid,
    pub device_selector: DeviceSelector,
}

pub fn parse_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let contents_str = contents.as_str();
    let res = toml::from_str(contents_str)?;
    Ok(res)
}
