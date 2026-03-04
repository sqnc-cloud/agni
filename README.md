# agni

A Redis-like in-memory cache server written in Rust.

## Project Structure

```
src/
├── main.rs          # Entry point — starts the TCP server
├── lib.rs           # Library root
├── config.rs        # Server configuration
├── server/          # Server bootstrap & per-connection handling
├── protocol/        # Protocol parsing and serialization
├── store/           # In-memory store with TTL tracking
└── cmd/             # Command implementations
```

## Roadmap

- [ ] Core commands (`GET`, `SET`, `DEL`, `EXISTS`, `EXPIRE`, `TTL`)
- [ ] TTL and background expiry cleanup
- [ ] Persistence
- [ ] Additional data types (lists, hashes, sets)
