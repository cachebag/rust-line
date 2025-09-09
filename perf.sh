#!/bin/bash

set -e

PORT=8080
ADDR="127.0.0.1:$PORT"
DURATION=15s
CONNECTIONS=250
THREADS=10
URL="http://$ADDR/files/index.html"
BINARY=target/release/rustline
SERVE_DIR=~/Desktop

echo "----Building server...----"
cargo build --release

echo "----Starting server in background...----"
$BINARY directory "$SERVE_DIR" > /dev/null 2>&1 &
SERVER_PID=$!

sleep 1 

echo "----Running wrk load test on $URL ----"
wrk -c "$CONNECTIONS" -t "$THREADS" -d "$DURATION" "$URL"

echo "----Stopping server...----"
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "Done."
