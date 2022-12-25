use log::{debug, info};
use serde::{Deserialize, Serialize};
use web_server::server::Server;

#[derive(Serialize, Deserialize)]
struct ServerConf {
    socket: String,
    num_worker: usize,
}

impl std::default::Default for ServerConf {
    fn default() -> Self {
        Self {
            socket: String::from("127.0.0.1:8088"),
            num_worker: 3,
        }
    }
}

fn main() {
    // Create a config file with default value
    //let conf = ServerConf::default();
    //confy::store_path("server.conf", conf).expect("Failed to write a config file !");

    let conf: ServerConf =
        confy::load_path("server.conf").expect("Failed to load config file server.config !");
    let s = Server::new(&conf.socket[..], conf.num_worker);
    //let s = Server::new("127.0.0.1:8080", 3);

    env_logger::init();
    info!("Starting server ...");
    debug!(
        "Listening on: {}; Workers: {}",
        &conf.socket[..],
        conf.num_worker
    );
    s.start();
}
