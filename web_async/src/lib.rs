use async_std::io::prelude::*;
use async_std::{net, task};
use surf;

pub async fn req_get_batch_surf(reqs: &[String]) -> Vec<Result<String, surf::Error>> {
    let mut hs = vec![];
    let mut rslt = vec![];

    let client = surf::Client::new();
    for r in reqs {
        let req = client.get(&r).recv_string();
        hs.push(task::spawn(req));
    }

    for h in hs {
        rslt.push(h.await);
    }

    rslt
}

pub async fn req_get_batch(reqs: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    let mut hs = vec![];
    let mut rslt = vec![];

    for (host, port, path) in reqs {
        // spawn async fn, collect the handler (future) to a vec
        hs.push(task::spawn_local(req_get(host, port, path)));
    }

    // trigger await, then all req will run concurrently
    for h in hs {
        rslt.push(h.await);
    }

    rslt
}

pub async fn req_get(host: String, port: u16, path: String) -> std::io::Result<String> {
    eprintln!("[DEBUG] Get {}:{}{}", host, port, path);
    // Open a new connection
    let mut conn = net::TcpStream::connect((&*host, port)).await?;
    eprintln!("[DEBUG] [{}] Connection established!", host);

    // Send a Get to destination
    let req = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    conn.write_all(req.as_bytes()).await?;
    eprintln!("[DEBUG] [{}] Request sent!", host);
    conn.shutdown(net::Shutdown::Write)?;
    eprintln!("[DEBUG] [{}] Stop write!", host);

    // Read response to a string
    let mut resp = String::new();
    conn.read_to_string(&mut resp).await?;
    eprintln!("[DEBUG] [{}] Response received!", host);

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {}
}
