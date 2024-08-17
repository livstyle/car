use makepad_micro_serde::SerJson;
use makepad_widgets::*;
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;

use tokio::runtime::Runtime;
use std::error::Error;
use std::time::Duration;
use tokio::time;
       
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
                    button_ble = <Button> {
                        margin: {
                            top: 10,
                            left: 100,
                        }
                        text: "获取蓝牙"
                        draw_text:{color:#f00}
                    }
                    label_ble = <Label> {
                        draw_text: {
                            color: #f
                        },
                        text: "蓝牙: 0"
                    }
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
            log!("BUTTON jk {}", self.counter); 
            self.counter += 1;
            let label = self.ui.label(id!(label1));
            label.set_text_and_redraw(cx,&format!("Counter: {}", self.counter));
            //log!("TOTAL : {}",TrackingHeap.total());
            
        }
        if self.ui.button(id!(button_ble)).clicked(&actions) {
            log!("BUTTON ble {}", self.counter); 
            let label = self.ui.label(id!(label_ble));
            // let result = block_on(future);
            let rt = Runtime::new().unwrap();
            let fut = async move {
                let manager = Manager::new().await.unwrap();
                let adapter_list = manager.adapters().await.unwrap();
                if adapter_list.is_empty() {
                    println!("No Bluetooth adapters found");
                    label.set_text_and_redraw(cx, "蓝牙连接失败");
                } else {
                    println!("蓝牙连接成功    : ==> {:?}", adapter_list);
                    label.set_text_and_redraw(cx, "蓝牙连接成功");
                    time::sleep(Duration::from_secs(2)).await;
                    for adapter in adapter_list.iter() {
                        println!("Starting scan on {}...", adapter.adapter_info().await.unwrap());
                        adapter
                            .start_scan(ScanFilter::default())
                            .await
                            .expect("Can't scan BLE adapter for connected devices...");
                        time::sleep(Duration::from_secs(2)).await;
                        let peripherals = adapter.peripherals().await.unwrap();
                        if peripherals.is_empty() {
                            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
                        } else {
                            // All peripheral devices in range
                            for peripheral in peripherals.iter() {
                                let properties = peripheral.properties().await.unwrap();
                                let is_connected = peripheral.is_connected().await.unwrap();
                                let local_name = properties
                                    .unwrap()
                                    .local_name
                                    .unwrap_or(String::from("(peripheral name unknown)"));
                                println!(
                                    "Peripheral {:?} is connected: {:?}",
                                    local_name, is_connected
                                );
                                if !is_connected {
                                    println!("Connecting to peripheral {:?}...", &local_name);
                                    if let Err(err) = peripheral.connect().await {
                                        eprintln!("Error connecting to peripheral, skipping: {}", err);
                                        continue;
                                    }
                                }
                                let is_connected = peripheral.is_connected().await.unwrap();
                                println!(
                                    "Now connected ({:?}) to peripheral {:?}...",
                                    is_connected, &local_name
                                );
                                peripheral.discover_services().await.unwrap();
                                println!("Discover peripheral {:?} services...", &local_name);
                                for service in peripheral.services() {
                                    println!(
                                        "Service UUID {}, primary: {}",
                                        service.uuid, service.primary
                                    );
                                    for characteristic in service.characteristics {
                                        println!("  {:?}", characteristic);
                                    }
                                }
                                if is_connected {
                                    println!("Disconnecting from peripheral {:?}...", &local_name);
                                    peripheral
                                        .disconnect()
                                        .await
                                        .expect("Error disconnecting from BLE peripheral");
                                }
                            }
                        }
                    }

                }
                // label.set_text_and_redraw(cx, &ds.join(";"));
            };
            rt.block_on(fut);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 

/*

// This is our custom allocator!
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicU64, Ordering},
};

pub struct TrackingHeapWrap{
    count: AtomicU64,
    total: AtomicU64,
}

impl TrackingHeapWrap {
    // A const initializer that starts the count at 0.
    pub const fn new() -> Self {
        Self{
            count: AtomicU64::new(0),
            total: AtomicU64::new(0)
        }
    }
    
    // Returns the current count.
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
    
    pub fn total(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for TrackingHeapWrap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Pass everything to System.
        self.count.fetch_add(1, Ordering::Relaxed); 
        self.total.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc(layout)
    }
        
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.count.fetch_sub(1, Ordering::Relaxed); 
        self.total.fetch_sub(layout.size() as u64, Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }
}

// Register our custom allocator.
#[global_allocator]
static TrackingHeap: TrackingHeapWrap = TrackingHeapWrap::new();*/