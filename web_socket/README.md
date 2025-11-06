# HOWTO
```sh
# Bring up server with tls
cargo run --bin ws_server 127.0.0.1:8888 tls
# In a second terminal run client with tls
cargo run --bin ws_client 127.0.0.1:8888 tls
```
