use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Reading {
    pub temperature: u16,
    pub humidity: u8,
    pub device_name: String,
    pub time_stamp: SystemTime,
}

pub type ReadingFn = dyn Fn(Vec<u8>, String) -> Result<Reading, Box<dyn std::error::Error>> + Sync;
