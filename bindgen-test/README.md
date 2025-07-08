```sh
# Generate rust code on the fly
$ cargo build
# Checkout the generated file under ./target/debug/build/bindgen-test-xxx/out/bindings.rs
$ cargo test

# Using CLI
$ cargo install bindgen-cli
$ bindgen /usr/include/bzlib.h -o /tmp/bindings.rs
```
