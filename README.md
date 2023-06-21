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
cargo test -- --nocapture
```

### Build package's doc
```sh
cargo doc [--no-deps] --open
```

### Static C runtime linkage
```
$ rustup target add x86_64-unknown-linux-musl
$ sudo dnf install musl-gcc
$ sudo ln -s /bin/g++ /bin/musl-g++
$ RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl
```
