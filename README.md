# Sabi (さび)

Sabi is a Sudoku solver / generator made with Rust

_'Sabi' (さび) means 'rust' in japanese_

## Performances

Performances are benchmarked through the `src/core/benchmark.rs` and the results are written in the `benchmark.log` file

## Dev

### Commands

> Many aliases and sequences are handled through [cargo-make](https://crates.io/crates/cargo-make) *you will need to install it*

```sh
cargo run # Builds and run the project

cargo fmt # Format code
cargo fmt -- --check # Throw error if unformated code

cargo clippy # Advanced linter
cargo clippy --fix # Fix auto fixable

cargo build # Only build it

cargo test # Run all unit tests
cargo test <file without extension> # Run specific test file inside the 'tests' folder (don't write it in path)
cargo test <specific function name> # Run specific test function

cargo add <package> [--dev] # Install a proect dependency (or a dev dependency)
cargo install <package> # Install a system wide dependency

cargo doc # Generates HTML documentation

cargo clean # Remove 'targer' directory (build artifacts, doc ...)
```

### Git hooks

Git hooks are handled with [rusty-hook](https://github.com/swellaby/rusty-hook), to enable them after a fresh install, run `cargo build`

## Tasks

Using [CLI-Manager](https://github.com/MikyStar/CLI-Manager) for task handling.
