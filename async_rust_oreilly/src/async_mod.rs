use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

pub type HandlerDict = LazyLock<Arc<Mutex<HashMap<String, JoinHandle<i32>>>>>;

static CUSTOM_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Error occurred creating Tokio runtime.")
});

async fn async_add(a: i32, b: i32) -> i32 {
    println!("[Inside add] Enter");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("[Inside add] Done");
    a + b
}

fn add_handler(
    a: Option<i32>,
    b: Option<i32>,
    key: Option<String>,
) -> Result<(Option<i32>, Option<String>), String> {
    static DICT: HandlerDict = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

    match (a, b, key) {
        (Some(num1), Some(num2), None) => {
            let handle = CUSTOM_RUNTIME.spawn(async_add(num1, num2));
            let key = uuid::Uuid::new_v4().to_string();
            DICT.lock().unwrap().insert(key.clone(), handle);
            Ok((None, Some(key)))
        }
        (None, None, Some(id)) => {
            let handle = match DICT.lock().unwrap().remove(&id) {
                Some(h) => h,
                None => return Err("Handler not found.".to_owned()),
            };
            let rslt = match CUSTOM_RUNTIME.block_on(handle) {
                Ok(rslt) => rslt,
                Err(e) => return Err(e.to_string()),
            };
            Ok((Some(rslt), None))
        }
        _ => Err("[add_handler] Illegal input".to_string()),
    }
}

// 提交一个a+b的任务，背后spawn一个async task来运算，
// task的handle存放在hashmap中，返回对应的key
// 方便调用者之后找到该handle，然后获取task的计算结果
pub fn submit_add(a: i32, b: i32) -> Result<String, String> {
    match add_handler(Some(a), Some(b), None) {
        Ok((None, Some(key))) => Ok(key),
        Err(e) => Err(e),
        Ok(_) => unreachable!(),
    }
}

pub fn retrieve_add(key: String) -> Result<i32, String> {
    match add_handler(None, None, Some(key)) {
        Ok((Some(rslt), None)) => Ok(rslt),
        Err(e) => Err(e),
        Ok(_) => unreachable!(),
    }
}
