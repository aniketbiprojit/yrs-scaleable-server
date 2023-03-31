# YRS Server

## Core Server

Immediately syncs all clients and passes updates to persist-server.

Opens the docs and keeps them in memory.

## Persist Server

Takes all the updates and stores them in database.

### Use Channel

```bash
cargo watch -x "run --bin core-server"
```

### Use Mutex

```bash
cargo watch -x "run --bin core-server --features use_mutex --no-default-features"
```
