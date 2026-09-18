# RustLogAnalyzer

A lightweight command-line log analysis tool built in **Rust**. It parses structured application logs, summarizes entries by severity, and filters logs by level.

## Why this project?

RustLogAnalyzer is a portfolio project focused on practical systems-programming fundamentals: strongly typed data modeling, file I/O, parsing, collections, error handling, testing, and command-line application design.

## Features

- Parse structured log lines in the format `[timestamp] [LEVEL] message`
- Preserve unstructured log messages instead of silently dropping them
- Summarize log entries by severity
- Filter logs by `TRACE`, `DEBUG`, `INFO`, `WARN`, or `ERROR`
- Unit and integration tests
- GitHub Actions CI with formatting, Clippy, and test checks
- Uses only the Rust standard library

## Project structure

```text
RustLogAnalyzer/
├── .github/workflows/ci.yml
├── examples/sample.log
├── src/lib.rs
├── src/main.rs
├── tests/integration_test.rs
├── .gitignore
├── Cargo.toml
└── README.md
```

## Requirements

Install the current stable Rust toolchain with Cargo.

Check your installation:

```bash
rustc --version
cargo --version
```

## Run

Clone the repository and enter the project folder:

```bash
git clone https://github.com/Arondith/RustLogAnalyzer.git
cd RustLogAnalyzer
```

Analyze the included sample log:

```bash
cargo run -- examples/sample.log
```

Example output:

```text
Log summary for: examples/sample.log
Total entries: 7
       DEBUG: 1
       ERROR: 2
        INFO: 3
        WARN: 1
```

Filter by severity:

```bash
cargo run -- examples/sample.log --level ERROR
```

## Test

```bash
cargo test
```

## Code quality

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Skills demonstrated

- Rust fundamentals and type safety
- Enums, structs, iterators, and collections
- File handling and command-line argument processing
- Parsing and data transformation
- Error handling
- Unit and integration testing
- Git/GitHub workflow and CI

## Roadmap

Possible future improvements include JSON output, date-range filtering, streaming very large log files, regex search, and performance benchmarking.

## License

MIT
