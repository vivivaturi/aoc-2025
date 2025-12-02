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

# Detect language and run
if [ -f "Cargo.toml" ]; then
    cargo run
elif [ -f "package.json" ]; then
    npm start
else
    echo "Unknown project type in $DAY"
    exit 1
fi
