# Advent of Code 2025

This repository contains my solutions for Advent of Code 2025. All days are implemented in Rust.

## Structure

- `dayXX/`: Directory for each day's solution
  - `src/main.rs`: Main solution implementation
  - `src/alternative.rs`: Alternative/optimized solution approaches
  - `Cargo.toml`: Rust project configuration
  - `input.txt`: Puzzle input for the day
  - `test_input.txt`: Example input for testing (when applicable)

## Running Solutions

Use `run.sh` script to execute a specific day's solution:

```bash
./run.sh day01
```

Or run directly from the day's directory:

```bash
cd day01 && cargo run
```

To run alternative solutions:

```bash
cd day03 && cargo run --bin alternative
```

## Days

- Day 1: Rust
- Day 2: Rust (with alternative solution)
- Day 3: Rust (with optimized alternative solution)
