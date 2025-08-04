# rust-line

A minimal HTTP server built from scratch in Rust

## Endpoints
- `GET /ping` – returns `PONG`.
- `GET /uptime` – returns server uptime.
- `GET /echo/<text>` – echoes `<text>` back.
- `GET /user-agent` – returns the `User-Agent` header from the request.
- `GET /files` - Returns content of a specified file

## Performance (WIP)
`perf.sh` will use `wrk` to run a performance test on the server. It runs 100 connections on 8 threads.
```bash
chmod +x perf.sh
./perf.sh
```
#### Current Benchmarks (may vary based on your machine's specs):
- ~174,000+ requests/second
- 544μs average response time
- Currently handles 100 concurrent connections
<br>
<img width="514" height="346" alt="image" src="https://github.com/user-attachments/assets/a8f3567e-0b1b-4cf6-abc1-c24aba776dad" />

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
