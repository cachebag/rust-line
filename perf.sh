#!/bin/bash

set -e

PORT=8080
ADDR="127.0.0.1:$PORT"
DURATION=15s
CONNECTIONS=100
THREADS=8
URL="http://$ADDR/ping"
BINARY=target/release/rustline

echo "----Building server...----"
cargo build --release

echo "----Starting server in background...----"
$BINARY > /dev/null 2>&1 & 
SERVER_PID=$!

sleep 1 

echo "----Running wrk load test...----"
wrk -c "$CONNECTIONS" -t "$THREADS" -d "$DURATION" "$URL"

echo "----Stopping server...----"
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "Done."

