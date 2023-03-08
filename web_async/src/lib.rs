use async_std::io::prelude::*;
use async_std::net;

pub async fn req_get(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    // Open a new connection
    let mut conn = net::TcpStream::connect((host, port)).await?;

    // Send a Get to destination
    let req = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    conn.write_all(req.as_bytes()).await?;
    conn.shutdown(net::Shutdown::Write)?;

    // Read response to a string 
    let mut resp = String::new();
    conn.read_to_string(&mut resp).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {}
}
