use web_server::Server;

fn main() {
    let s = Server::new("127.0.0.1:8080", 3);
    s.start();
}
