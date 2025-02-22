use chrono::{DateTime, Utc};
use serde::Serialize;

///class that represents a single measurement of temerature/humidity at a given point in time
///
/// *`temerature` the temerature at the time of measurement
/// *`humidity`  the humidity at the time of measurement
/// *`device_name` somewhat unique identifier of the device...somewha
/// *`time_stamp` timestamp when the measurement was taken
#[derive(Debug, Clone, Serialize)]
pub struct Reading {
    /// the temerature at the time of measurement
    pub temperature: u16,
    /// the humidity at the time of measurement
    pub humidity: u8,
    /// somewhat unique identifier of the device...somewhat
    pub device_name: DeviceName,
    /// timestamp when the measurement was taken
    pub time_stamp: DateTime<Utc>,
}

///Simple struct that is used to link a device to a description(nickname)
/// *`name` identifier of the device
/// *`nickname` a description of the device
#[derive(Debug, Clone, Serialize)]
pub struct DeviceName {
    pub name: String,
    pub nickname: Option<String>,
}

pub type ReadingFn = dyn Fn(Vec<u8>, String) -> Result<Reading, Box<dyn std::error::Error>> + Sync;
