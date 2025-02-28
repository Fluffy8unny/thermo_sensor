use futures::stream::StreamExt;
use std::collections::BTreeSet;
use uuid::Uuid;

use btleplug::api::{
    Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral as _,
    PeripheralProperties, ScanFilter,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio::sync::mpsc;

use crate::config;
use crate::reading::{Reading, ReadingFn};
use config::DeviceSelector;

fn check_name(
    peripheral_properties: Option<PeripheralProperties>,
    comparison: &str,
) -> (bool, String) {
    let name = peripheral_properties
        .and_then(|p| p.local_name)
        .unwrap_or_default();
    (name.contains(comparison), name)
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
///function to select one adapter from a given list of adapters based on config
///
///*`adapter_list` vector of all currentlyt detected adatpers
///*`selector` DeviceSelector instance, that explains how we should select a device from
///adapter_list
///
///returns an adapter if a valid one was found according to the rules in device_selector, otherwise
///an error
async fn select_adapter(
    adapter_list: Vec<Adapter>,
    selector: DeviceSelector,
) -> Result<Adapter, Box<dyn std::error::Error>> {
    if adapter_list.len() == 0 {
        return Err("No adapters found")?;
    }
    match selector {
        DeviceSelector::First => Ok(adapter_list[0].clone()),
        DeviceSelector::NTh { index } => {
            if adapter_list.len() >= index {
                Ok(adapter_list[index - 1].clone())
            } else {
                Err(format!("provided device index{} out of ranger", index))?
            }
        }
        DeviceSelector::Name { substring } => {
            for (adp, info) in adapter_list
                .iter()
                .map(|adapter| (adapter.clone(), adapter.adapter_info()))
            {
                match info.await {
                    Ok(info) => {
                        if info.contains(&substring) {
                            return Ok(adp);
                        }
                    }
                    _ => {}
                }
            }

            Err(format!("name '{}' not found", substring))?
        }
    }
}

async fn read_device_loop(
    characteristic: Characteristic,
    name: String,
    peripheral: &Peripheral,
    cloned_sender: mpsc::UnboundedSender<Reading>,
    reading_fn: &'static ReadingFn,
) -> Result<(), Box<dyn std::error::Error>> {
    peripheral.subscribe(&characteristic).await?;
    let mut notification_stream = peripheral.notifications().await?;
    while let Some(data) = notification_stream.next().await {
        match reading_fn(data.value, name.clone()) {
            Ok(r) => match cloned_sender.send(r) {
                Ok(_) => {}
                Err(_) => {
                    println!("ERROR sending data for device")
                }
            },
            Err(_) => {
                println!("error reading data from {}", name)
            }
        }
    }
    Ok(())
}

async fn poll_devices(
    adapter: Adapter,
    tx: mpsc::UnboundedSender<Reading>,
    device_info: config::BluetoothConfig,
    reading_fn: &'static ReadingFn,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut events = adapter.events().await?;
    adapter.start_scan(ScanFilter::default()).await?;
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

                    if peripheral.is_connected().await.unwrap_or_default() {
                        let _ = peripheral.discover_services().await;
                        if let Some(c) = find_characteristic(
                            peripheral.characteristics(),
                            device_info.service_uuid,
                        ) {
                            let cloned_sender = tx.clone();
                            tokio::spawn(async move {
                                let _ = read_device_loop(
                                    c,
                                    name,
                                    &peripheral,
                                    cloned_sender,
                                    reading_fn,
                                )
                                .await;
                                let _ = peripheral.disconnect().await;
                            });
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
    device_info: config::BluetoothConfig,
    tx: mpsc::UnboundedSender<Reading>,
    reading_fn: &'static ReadingFn,
) -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;

    let adapter = select_adapter(adapter_list, device_info.device_selector.clone()).await?;
    let _ = adapter.stop_scan().await;
    let task = tokio::spawn(async move {
        let _ = poll_devices(adapter, tx, device_info.clone(), reading_fn).await;
    });
    Ok(task)
}
