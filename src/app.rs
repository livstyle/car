use makepad_micro_serde::SerJson;
use makepad_widgets::*;
use btleplug::platform::Manager;
use uuid::Uuid;
use anyhow::anyhow;
use lazy_static::lazy_static;

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter, WriteType};
// use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use tokio::runtime::Runtime;

use std::{ops::Deref, sync::{Arc, Mutex}};
use once_cell::sync::Lazy;

use crate::builder::{Builder, thread::SpawnAttach as _, Runtime as _};
/// Only devices whose name contains this string will be tried.
const PERIPHERAL_NAME_MATCH_FILTER: &str = "智能小车蓝牙";
/// UUID of the characteristic for which we should subscribe to notifications.
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x6e400002_b534_f393_67a9_e50e24dccA9e);

// pub struct Config {
//     // peripheral: Option<Box<dyn btleplug::api::Peripheral>>,
//     characteristic: Option<btleplug::api::Characteristic>
// }


// pub static GLOBAL_CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| {
//     Arc::new(Mutex::new(Config { characteristic: None }))
// });


// lazy_static! {
//     static ref GLOBAL_P: Arc<Option<Box<dyn btleplug::api::Peripheral>>> = {
//         Arc::new(None)
//     };
// }

lazy_static! {
    pub static ref MYRUNTIME: Runtime = Builder::new();
}

       
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    
    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                show_bg: true
                width: Fill,
                height: Fill
                
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        // test
                        return mix(#7, #3, self.pos.y);
                    }
                }
                
                body = <ScrollXYView>{
                    flow: Down,
                    spacing:10,
                    // align: {
                    //     x: 0.5,
                    //     y: 0.5
                    // },
                    button1 = <Button> {
                        margin: {
                            top: 10,
                            left: 100,
                        }
                        text: "前进"
                        draw_text:{color:#f00}
                    }
                    left_right = <View> {
                        height: 150
                        margin: {
                            top: 100
                        }
                        button3 = <Button> {
                            margin: {
                                left: 10,
                            }
                            text: "左转"
                            draw_text:{color:#f00}
                        }
                        button4 = <Button> {
                            margin: {
                                left: 150
                            }
                            text: "右转"
                            draw_text:{color:#f00}
                        }
                    }
                    button2 = <Button> {
                        margin: {
                            left: 100
                        }
                        text: "后退"
                        draw_text:{color:#f00}
                    }
                    input1 = <TextInput> {
                        width: 100, height: 30
                        text: "Click to count "
                    }
                    label1 = <Label> {
                        draw_text: {
                            color: #f
                        },
                        text: "Counter: 0"
                    }
                    // button_ble = <Button> {
                    //     margin: {
                    //         top: 10,
                    //         left: 100,
                    //     }
                    //     text: "获取蓝牙"
                    //     draw_text:{color:#f00}
                    // }
                    // label_ble = <Label> {
                    //     draw_text: {
                    //         color: #f
                    //     },
                    //     text: "蓝牙: 0"
                    // }
                }
            }
        }
    }
}  
              
app_main!(App); 
 
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] counter: usize,
}
 
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        //println!("{}", std::mem::size_of::<LiveNode2>());
        /*makepad_draw::live_design(cx);
        makepad_widgets::base::live_design(cx);
        makepad_widgets::theme_desktop_dark::live_design(cx);
        makepad_widgets::label::live_design(cx);
        makepad_widgets::view::live_design(cx);
        makepad_widgets::button::live_design(cx);
        makepad_widgets::window::live_design(cx);
        makepad_widgets::scroll_bar::live_design(cx);
        makepad_widgets::scroll_bars::live_design(cx);
        makepad_widgets::root::live_design(cx);*/
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
        
        if self.ui.button(id!(button1)).clicked(&actions) {
            log!("BUTTON 前进: {}", self.counter); 
            // self.counter += 1;
            // let label = self.ui.label(id!(label1));
            // label.set_text_and_redraw(cx,&format!("Counter: {}", self.counter));
            //log!("TOTAL : {}",TrackingHeap.total());

            let th = std::thread::Builder::new()
                .name(String::from("发送数据线程"))
                .spawn(move || {
                    // debug!("callng discover under MYRUNTIME in thread id: {:?}", thread::current().id());
                    match MYRUNTIME.block_on(async { ble(1).await }) {
                        Ok(_) => { println!("数据发送成功");},
                        Err(e) => {println!("数据发送失败: {:?}", e)}
                    }
                    // debug!("exiting 发送数据线程: {:?}", thread::current().id());
            }).unwrap();
            th.join().unwrap();
        }

        if self.ui.button(id!(button3)).clicked(&actions) {
            log!("BUTTON 左转"); 
            let th = std::thread::Builder::new()
                .name(String::from("发送数据线程"))
                .spawn(move || {
                    // debug!("callng discover under MYRUNTIME in thread id: {:?}", thread::current().id());
                    match MYRUNTIME.block_on(async { ble(2).await }) {
                        Ok(_) => { println!("数据发送成功");},
                        Err(e) => {println!("数据发送失败: {:?}", e)}
                    }
                    // debug!("exiting 发送数据线程: {:?}", thread::current().id());
            }).unwrap();
            th.join().unwrap();
        }

        if self.ui.button(id!(button4)).clicked(&actions) {
            log!("BUTTON 右转"); 
            let th = std::thread::Builder::new()
                .name(String::from("发送数据线程"))
                .spawn(move || {
                    // debug!("callng discover under MYRUNTIME in thread id: {:?}", thread::current().id());
                    match MYRUNTIME.block_on(async { ble(3).await }) {
                        Ok(_) => { println!("数据发送成功");},
                        Err(e) => {println!("数据发送失败: {:?}", e)}
                    }
                    // debug!("exiting 发送数据线程: {:?}", thread::current().id());
            }).unwrap();
            th.join().unwrap();
        }

        if self.ui.button(id!(button2)).clicked(&actions) {
            log!("BUTTON 后退"); 
            let th = std::thread::Builder::new()
                .name(String::from("发送数据线程"))
                .spawn(move || {
                    // debug!("callng discover under MYRUNTIME in thread id: {:?}", thread::current().id());
                    match MYRUNTIME.block_on(async { ble(4).await }) {
                        Ok(_) => { println!("数据发送成功");},
                        Err(e) => {println!("数据发送失败: {:?}", e)}
                    }
                    // debug!("exiting 发送数据线程: {:?}", thread::current().id());
            }).unwrap();
            th.join().unwrap();
        }

        // if self.ui.button(id!(button_ble)).clicked(&actions) {
        //     log!("BUTTON ble {}", self.counter); 
        //     let label = self.ui.label(id!(label_ble));
        //     // let result = block_on(future);
        //     let rt = Runtime::new().unwrap();
        //     let fut = async move {
        //         let res = init_ble().await;
        //         label.set_text_and_redraw(cx, "蓝牙连接成功");
        //     };
        //     rt.block_on(fut);
        // }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 


// #[tokio::main]
async fn ble(num: u8) -> Result<(), anyhow::Error> {
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
        time::sleep(Duration::from_millis(500)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            println!("->>> BLE peripheral devices were not found, sorry. Exiting...");
            return Err(anyhow!("Scan failed"));
        } else {
            // All peripheral devices in range.
            let mut is_send = false;
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
                            let r = peripheral.write(&characteristic, &vec![num], WriteType::WithoutResponse).await;
                            println!("Write result: {:?}", r);
                            is_send = true;
                            // let config = GLOBAL_CONFIG.lock().unwrap();
                            // // config.peripheral = Some(Box::new(peripheral.clone()));
                            // let global_p = GLOBAL_P.unwrap();
                            // *global_p = Box::new(peripheral.clone());
                            // config.characteristic = Some(characteristic.clone());
                            break;
                        }
                        // println!("Disconnecting from peripheral {:?}...", local_name);
                        // peripheral.disconnect().await?;
                    }
                } else {
                    println!("Skipping unknown peripheral {:?}", peripheral.address());
                    continue;
                }
            }
            if is_send {
                return Ok(());
            } else {
                return Err(anyhow!("Scan completed not found in peripherals"));
            }
        }
    }
    Ok(())
}