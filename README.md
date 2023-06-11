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
