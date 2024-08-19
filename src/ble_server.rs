// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter, WriteType};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

/// Only devices whose name contains this string will be tried.
const PERIPHERAL_NAME_MATCH_FILTER: &str = "智能小车蓝牙";
/// UUID of the characteristic for which we should subscribe to notifications.
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x6e400002_b534_f393_67a9_e50e24dccA9e);

use crate::GLOBAL_CONFIG;

async fn init_ble() -> Result<(), Box<dyn Error>> {
    // pretty_env_logger::init();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        println!("No Bluetooth adapters found");
        return Err(anyhow!("No Bluetooth adapters found"));
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await?;
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            println!("->>> BLE peripheral devices were not found, sorry. Exiting...");
            return Err(anyhow!("Scan failed"));
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                println!(
                    "Peripheral {:?} is connected: {:?}",
                    &local_name, is_connected
                );
                // Check if it's the peripheral we want.
                if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
                    println!("Found matching peripheral {:?}...", &local_name);
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            eprintln!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    let is_connected = peripheral.is_connected().await?;
                    println!(
                        "Now connected ({:?}) to peripheral {:?}.",
                        is_connected, &local_name
                    );
                    if is_connected {
                        println!("Discover peripheral {:?} services...", local_name);
                        peripheral.discover_services().await?;
                        for characteristic in peripheral.characteristics() {
                            println!("Checking characteristic {:?}", characteristic);
                            let r = peripheral.write(&characteristic, &vec![0x11], WriteType::WithoutResponse).await;
                            println!("Write result: {:?}", r);
                            let config = GLOBAL_CONFIG.lock().unwrap();
                            config.peripheral = peripheral.clone();
                            config.characteristic = characteristic.clone();
                        }
                        // println!("Disconnecting from peripheral {:?}...", local_name);
                        // peripheral.disconnect().await?;
                    }
                } else {
                    println!("Skipping unknown peripheral {:?}", peripheral);

                }
            }
        }
    }
    Ok(())
}