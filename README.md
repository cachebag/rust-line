# rust-line

A minimal HTTP server built from scratch in Rust

## Endpoints
- `GET /ping` – returns `PONG`.
- `GET /uptime` – returns server uptime.
- `GET /echo/<text>` – echoes `<text>` back.
- `GET /user-agent` – returns the `User-Agent` header from the request.
- `GET /files` - Returns content of a specified file

## Performance (WIP)
`perf.sh` will use `wrk` to run a performance test on the server. It runs 250 connections on 10 threads. It serves `index.html`.
```bash
chmod +x perf.sh
./perf.sh
```
#### Current Benchmarks (may vary based on your machine's specs):
- ~100,000+ requests/second
- ~2.4ms average response time
- Tests 250 concurrent connections on 10 threads
- Throughout -> 6.8MB/s
<br>
<img width="737" height="326" alt="image" src="https://github.com/user-attachments/assets/8f2aa02c-fe1d-4acb-9dee-dd99a8aba900" />

###### **Device Specs**: 
<img width="625" height="531" alt="image" src="https://github.com/user-attachments/assets/c44c6a65-9c34-406b-b37d-25faa3e5a338" />

## Run

No directory specified
```bash
cargo run --release -- ns 
````

Set cwd
```bash
cargo run --release -- directory /path/to/dir
````

Default cwd to `.`
```bash
cargo run --release -- directory 
````

## Test (WIP)

**Integration Tests**
```bash
cargo test --test load_test -- --nocapture
```
**Unit Tests**
```bash
cargo test --lib
```

**Everything**
```bash
cargo test
```
