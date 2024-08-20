use crate::builder::{Builder, thread::SpawnAttach as _, Runtime as _};

lazy_static! {
    pub static ref MYRUNTIME: Runtime = Builder::new();
}

pub fn exec_async_without_response() {
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