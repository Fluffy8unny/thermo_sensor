use serde_derive::Deserialize;
use uuid::Uuid;
use std::fs;


#[derive(Deserialize,Debug,Clone)]
pub struct Config{
   pub db_config : DatabaseConfig,
   pub bt_config : BluetoothConfig
}

#[derive(Deserialize,Debug,Clone)]
pub struct DatabaseConfig{
    pub file_name : String,
    pub backend_ip : String,
    pub backend_port: u16,
}

#[derive(Deserialize,Debug,Clone)]
pub struct BluetoothConfig{
    pub name : String,
    pub service_uuid : Uuid,
}

pub fn parse_config(file_path:&str)->Result<Config,Box<dyn std::error::Error>>{
    let contents =  fs::read_to_string(file_path)?;
    let contents_str = contents.as_str();
    let res = toml::from_str(contents_str)?;
    Ok(res)
}

