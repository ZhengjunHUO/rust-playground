use log::info;
use web_server::server::Server;

fn main() {
    let s = Server::new("127.0.0.1:8080", 3);

    env_logger::init();
    info!("Starting server ...");
    s.start();
}
