#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <dayXX>"
    exit 1
fi

DAY=$1

if [ ! -d "$DAY" ]; then
    echo "Directory $DAY does not exist"
    exit 1
fi

cd "$DAY"

# Run Rust solution
if [ -f "Cargo.toml" ]; then
    cargo run
else
    echo "No Rust project found in $DAY"
    exit 1
fi
