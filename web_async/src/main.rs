use async_std::task;
use web_async::req_get;

fn main() -> std::io::Result<()> {
    //let rslt = task::block_on(req_get("www.baidu.com", 80, "/"))?;
    let rslt = task::block_on(req_get("ifconfig.me", 80, "/"))?;
    println!("Reply from target host:\r\n{}", rslt);
    Ok(())
}
