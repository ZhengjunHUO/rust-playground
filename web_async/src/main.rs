use async_std::task;
use web_async::req_get_batch;

fn main() {
    // block on single request
    //let rslt = task::block_on(req_get("ifconfig.me".to_string(), 80, "/".to_string()))?;

    let reqs = vec![
        ("ifconfig.me".to_string(), 80, "/".to_string()),
        ("www.baidu.com".to_string(), 80, "/".to_string()),
        ("wikipedia.com".to_string(), 80, "/".to_string()),
    ];

    // block on a pool of request
    // block_on will be notified by the some future when its await is ready,
    // then poll that future (which is worth polling) until its next await
    let rslt = task::block_on(req_get_batch(reqs));

    // examine the result
    for r in rslt {
        match r {
            Ok(resp) => println!("**********\r\n{}", resp.len()),
            Err(e) => eprintln!("Failed to Get: {}", e),
        }
    }
}
