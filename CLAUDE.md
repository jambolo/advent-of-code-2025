# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build all days
cargo build

# Build specific day <NN>
cargo build -p dayNN

# Run a specific day <NN> (input file required as argument)
cargo run -p dayNN -- dayNN/dayNN-input.txt

# Run a specific day <NN> with example input (input file required as argument)
cargo run -p dayNN -- dayNN/dayNN-input-example.txt

# Run part 1 (disable default part2 feature if enabled)
cargo run -p dayNN --no-default-features -- dayNN/dayNN-input.txt

# Run tests
cargo test

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt
```

## Architecture

- **Workspace structure**: Each day is a separate crate (`day01`, `day02`, etc.) with a shared `common` crate.
- **Part selection**: Parts are selected via Cargo features. If `part2` is not the default feature then only part 1 can be run. When `part2` is the default feature; use `--no-default-features` for part 1.
- **Input loading**: The `common::load` module provides helpers (`lines()`, `string()`, `map()`, `numbers_map()`, `comma_separated_values()`) that read from the file path passed as the first CLI argument.
- **Multi-file days**: Complex days may split logic into `part1.rs` and `part2.rs` modules, conditionally compiled based on the `part2` feature.
