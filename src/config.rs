use serde_derive::Deserialize;
use uuid::Uuid;
use std::fs;

#[derive(Deserialize,Debug,Clone)]
pub struct DeviceInfo{
    pub name : String,
    pub service_uuid : Uuid,
}

pub fn parse_config(file_path:&str)->Result<DeviceInfo,Box<dyn std::error::Error>>{
    let contents =  fs::read_to_string(file_path)?;
    let contents_str = contents.as_str();
    let res = toml::from_str(contents_str)?;
    Ok(res)
}

