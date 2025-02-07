use serde::Serialize;
use chrono::{DateTime,Utc};

#[derive(Debug, Clone,Serialize)]
pub struct Reading {
    pub temperature: u16,
    pub humidity: u8,
    pub device_name: DeviceName,
    pub time_stamp: DateTime<Utc>,
}

#[derive(Debug, Clone,Serialize)]
pub struct DeviceName {
    pub name: String,
    pub nickname: Option<String>,
}

pub type ReadingFn = dyn Fn(Vec<u8>, String) -> Result<Reading, Box<dyn std::error::Error>> + Sync;
