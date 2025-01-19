use std::{
    io::{stdout, Write},
    sync::{atomic::Ordering, Arc, LazyLock, Mutex},
};

use async_rust_oreilly::observers::{Display, HeatLoss, Heater, SWITCH_ON, TEMPERATURE, WANTED};
use device_query::{DeviceEvents, DeviceState};

static USER_INPUT: LazyLock<Arc<Mutex<String>>> =
    LazyLock::new(|| Arc::new(Mutex::new(String::new())));
static DEVICE_STATE: LazyLock<Arc<Mutex<DeviceState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(DeviceState::new())));

fn render(temperature: i16, wanted: i16, switch_on: bool, input: String) {
    clearscreen::clear().unwrap();
    let mut stdout = stdout().lock();
    println!(
        "Acutal temperature: {} [Wanted: {}]; HEATER IS {}",
        temperature as f32 / 100.0,
        wanted as f32 / 100.0,
        if switch_on { "[ON]" } else { "[OFF]" }
    );
    println!("User input: {input}");
    stdout.flush().unwrap();
}

fn func_take_callback<F>(callback: F)
where
    F: Fn(u32),
{
    println!("Inside wrapper func, will execute callback func.");
    callback(25);
    println!("Inside wrapper func, all done, quit.");
}

#[tokio::main]
async fn main() {
    let cb = |num: u32| {
        println!("Inside callback func, the num is {num}");
    };

    func_take_callback(cb);

    // 创建了一个监听键盘的输入的event listener: 创建了一个thread并运行一个event loop
    let _guard = DEVICE_STATE.lock().unwrap().on_key_down(|key| {
        {
            let mut input = USER_INPUT.lock().unwrap();
            input.push_str(&key.to_string());
        }
        render(
            TEMPERATURE.load(Ordering::SeqCst),
            WANTED.load(Ordering::SeqCst),
            SWITCH_ON.load(Ordering::SeqCst),
            USER_INPUT.lock().unwrap().clone(),
        );
    });

    let handlers = vec![
        tokio::spawn(Display::new()),
        tokio::spawn(Heater::new()),
        tokio::spawn(HeatLoss::new()),
    ];
    for handler in handlers {
        handler.await.unwrap();
    }
}
