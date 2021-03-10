# Arcades

## Quick Start

- add `.env` file in the project's root with the following lines.
```
RUST_LOG=error,info
RUST_BACKTRACE=0
SERVER_ADDR=127.0.0.1:9002
```

- `cargo build --bin server && ./target/debug/server` or `cargo run --bin server` to run server.
- `cargo build --bin client && ./target/debug/client` or `cargo run --bin client` to run client.

## Before each push:

- `cargo check` should not fail !!
- `cargo fmt` to format your source code.