# Rustline

A minimal HTTP server built from scratch in Rust

## Endpoints
- `GET /ping` – returns `PONG`.
- `GET /uptime` – returns server uptime.
- `GET /echo/<text>` – echoes `<text>` back.
- `GET /user-agent` – returns the `User-Agent` header from the request.

## Run

```bash
cargo run
````

## Test

To run tests:

```bash
cargo test
```
