# agni

A Redis-like in-memory cache server written in Rust.

## Workspace

| Crate | Description |
|---|---|
| `agni` | Core library — store, protocol, config. Publishable to crates.io |
| `agni-server` | TCP server binary |
| `agni-client` | CLI client binary |

## Project Structure

```
agni/
├── agni/                  # Core library
│   └── src/
│       ├── config.rs      # Server configuration
│       ├── protocol/      # Command parsing and responses
│       ├── store/         # In-memory key-value store
│       └── cmd/           # Command implementations
├── agni-server/           # Server binary
│   └── src/
│       ├── main.rs        # Entry point
│       └── server/        # TCP listener and connection handling
└── agni-client/           # CLI client binary
    └── src/
        └── main.rs        # Entry point
```

## Getting Started

```bash
# Run the server
cargo run -p agni-server -- --config config.example.yml

# Send a command
cargo run -p agni-client -- PING
```

## Using agni as a library

```toml
[dependencies]
agni = "0.1"
```

```rust
use agni::store::Store;

let store = Store::new();
store.set("key".to_string(), b"value".to_vec());
```

## Logging

agni uses [`tracing`](https://docs.rs/tracing) for structured, async-aware logging. Unlike `println!`, it does not lock stdout on every call, avoiding I/O bottlenecks under high concurrency.

Log level is controlled at runtime via the `RUST_LOG` environment variable:

```bash
RUST_LOG=info ./agni-server --config config.example.yml
RUST_LOG=debug ./agni-server --config config.example.yml
```

Available levels: `error`, `warn`, `info`, `debug`, `trace`.
If `RUST_LOG` is not set, no logs are emitted.

## Roadmap

- [ ] Core commands (`GET`, `SET`, `DEL`, `EXISTS`, `EXPIRE`, `TTL`)
- [ ] TTL and background expiry cleanup
- [ ] Persistence
- [ ] Additional data types (lists, hashes, sets)
