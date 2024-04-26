use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use web_warp::filters::all_routes_handled;

const SRV_ADDR: &str = "DR_API_SERVER_ADDRESS";
const SRV_PORT: &str = "DR_API_SERVER_PORT";

#[tokio::main]
async fn main() {
    let mut port = 8000;
    let mut ipv4 = Ipv4Addr::new(0, 0, 0, 0);

    if let Ok(s_port) = env::var(SRV_PORT) {
        if let Ok(val) = s_port.parse::<u16>() {
            port = val;
        }
    }

    if let Ok(s_addr) = env::var(SRV_ADDR) {
        if let Ok(addr) = s_addr.parse::<Ipv4Addr>() {
            ipv4 = addr;
        }
    }

    warp::serve(all_routes_handled())
        .run(SocketAddrV4::new(ipv4, port))
        .await;
}
