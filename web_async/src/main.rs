use async_std::task;
use web_async::{req_get_batch, req_get_batch_surf};

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
            Ok(resp) => println!("**********\r\n{}\r\n", resp.len()),
            Err(e) => eprintln!("Failed to Get: {}", e),
        }
    }

    // Using surf framework
    let rs = &[
        "http://ifconfig.me".to_string(),
        "http://www.baidu.com".to_string(),
        "https://www.google.com".to_string(),
    ];

    let results = task::block_on(req_get_batch_surf(rs));
    for result in results {
        match result {
            Ok(resp) => println!("**********\r\n{}\r\n", resp),
            Err(e) => eprintln!("Failed to Get: {}", e),
        }
    }
}
