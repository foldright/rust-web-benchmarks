# Bench Bot

Automate benchmarking process.

## Pre Install

- rewrk: `cargo install rewrk`

## Usage

See:

```
cargo run --release -- --help
```

## Example

```
cargo run --release -- -w ../benchmark -o ../result -c 100 -t 4
```
