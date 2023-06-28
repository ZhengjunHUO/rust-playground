# rust-playground

### When unstable features engaged, use nightly build
```sh
$ rustup override set nightly
```

### Perform macro expansion to debug
```sh
$ rustc -Zunpretty=expanded src/main.rs
```

### See the println in cargo test
```sh
$ cargo test -- --nocapture
```

### Docs
```sh
# Check std doc
$ rustup component add rust-docs
$ rustup doc

# Current crate's doc
$ cargo doc [--no-deps] --open
```

### Static C runtime linkage
```
$ rustup target add x86_64-unknown-linux-musl
$ sudo dnf install musl-gcc
$ sudo ln -s /bin/g++ /bin/musl-g++
$ RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl
```

### Using tokio
- [async_tasks](https://github.com/ZhengjunHUO/rust-playground/tree/main/async_tasks)
- [chat_async](https://github.com/ZhengjunHUO/rust-playground/tree/main/chat_async)
- [ckh-client](https://github.com/ZhengjunHUO/rust-playground/tree/main/ckh-client)
- [clickhouse-client](https://github.com/ZhengjunHUO/rust-playground/tree/main/clickhouse-client)
- [docker](https://github.com/ZhengjunHUO/rust-playground/tree/main/docker)
- [k8s](https://github.com/ZhengjunHUO/rust-playground/tree/main/k8s)
- [redis-server](https://github.com/ZhengjunHUO/rust-playground/tree/main/redis-server)
- [s3-client](https://github.com/ZhengjunHUO/rust-playground/tree/main/s3-client)
