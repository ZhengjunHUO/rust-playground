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
# Fedora
$ sudo dnf install musl-gcc
# Ubuntu
$ sudo apt install musl-tools pkg-config
$ sudo ln -s /bin/g++ /bin/musl-g++
$ RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl
```
Error caused by libssl during build
```
  run pkg_config fail: pkg-config has not been configured to support cross-compilation.

  Install a sysroot for the target platform and configure it via
  PKG_CONFIG_SYSROOT_DIR and PKG_CONFIG_PATH, or install a
  cross-compiling wrapper for pkg-config and set it via
  PKG_CONFIG environment variable.

  --- stderr
  thread 'main' panicked at '

  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.

  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

  If you're in a situation where you think the directory *should* be found
  automatically, please open a bug at https://github.com/sfackler/rust-openssl
  and include information about your system as well as this message.

  $HOST = x86_64-unknown-linux-gnu
  $TARGET = x86_64-unknown-linux-musl
  openssl-sys = 0.9.90
```
Specify vendored feature in Cargo.toml
```
openssl-sys = {version = "0.9.90", features = ["vendored"]}
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

### Aya BPF prerequis
```sh
$ rustup install stable
$ rustup toolchain install nightly --component rust-src
$ cargo install bpf-linker
$ cargo install cargo-generate
$ sudo apt install libssl-dev linux-tools-common
$ cargo generate https://github.com/aya-rs/aya-template
```
