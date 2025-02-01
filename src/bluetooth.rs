use btleplug::api::{
    Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral,
    PeripheralProperties, ScanFilter,
};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use uuid::Uuid;
use std::collections::BTreeSet;

use crate::reading::Reading;
use crate::config;

fn check_name(
    peripheral_properties: Option<PeripheralProperties>,
    comparison: &str,
) -> (bool, String) {
    let name = peripheral_properties
        .and_then(|p| p.local_name)
        .map(|local_name| format!("Name: {local_name}"))
        .unwrap_or_default();
    let is_interessting = name.contains(comparison);
    (is_interessting, name)
}

fn find_characteristic(
    characteristics: BTreeSet<Characteristic>,
    characteristic_uuid: Uuid,
) -> Option<Characteristic> {
    for characteristic in characteristics {
        if characteristic.uuid == characteristic_uuid
            && characteristic.properties.contains(CharPropFlags::NOTIFY)
        {
            return Some(characteristic);
        }
    }
    None
}

fn select_adapter(adapter_list: Vec<Adapter>) -> Adapter {
    adapter_list[0].clone()
}

async fn poll_devices(
    adapter: Adapter,
    device_info: config::DeviceInfo,
    reading_fn: &'static (dyn Fn(Vec<u8>) -> Result<Reading, Box<dyn std::error::Error>> + Sync),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut events = adapter.events().await?;
    //let _ = adapter.stop_scan().await;
    //return Ok(());
    adapter.start_scan(ScanFilter::default()).await?;
    println!("START SCANNING");
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral = adapter.peripheral(&id).await?;
                let properties = peripheral.properties().await?;
                let (is_interessting, name) = check_name(properties, device_info.name.as_str());

                if is_interessting {
                    if !peripheral.is_connected().await.unwrap_or_default() {
                        let _ = peripheral.connect().await.unwrap_or_default();
                    }

                    println!("{} IS CONNECTED", name);
                    if peripheral.is_connected().await.unwrap_or_default() {
                        let _ = peripheral.discover_services().await;

                        match find_characteristic(
                            peripheral.characteristics(),
                            device_info.service_uuid,
                        ) {
                            Some(c) => {
                                tokio::spawn(async move {
                                    match peripheral.subscribe(&c).await {
                                        Ok(_) => {
                                            let notification_stream_result =
                                                peripheral.notifications().await;
                                            if let Ok(mut notification_stream) =
                                                notification_stream_result
                                            {
                                                while let Some(data) =
                                                    notification_stream.next().await
                                                {
                                                    match reading_fn(data.value) {
                                                        Ok(Reading {
                                                            temperature,
                                                            humidity,
                                                        }) => {
                                                            let t_f: f32 = <u16 as Into<f32>>::into(
                                                                temperature,
                                                            ) / 10_f32;
                                                            println!(
                                                                "{} {} {}",
                                                                name, t_f, humidity
                                                            );
                                                        }

                                                        Err(_) => {
                                                            println!(
                                                                "error reading data from {}",
                                                                name
                                                            )
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }

                                    let _ = peripheral.disconnect().await;
                                });
                            }
                            None => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn start_bluetooth_thread(
    device_info: config::DeviceInfo,
    reading_fn: &'static (dyn Fn(Vec<u8>) -> Result<Reading, Box<dyn std::error::Error>> + Sync),
) -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;

    let adapter = select_adapter(adapter_list);
    let _ = adapter.stop_scan().await;
    let task = tokio::spawn(async move {
        let _ = poll_devices(adapter, device_info.clone(), reading_fn).await;
    });
    Ok(task)
}
