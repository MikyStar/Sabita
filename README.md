# Sabi

_Sabi means rust in japanese_

Sabi is a Sudoku solver / generator made with Rust

## Dev

### Commands

> Many alias and sequences are handled with [cargo-make](https://crates.io/crates/cargo-make)

```sh
cargo run # Builds and run the project

cargo fmt # Format code
cargo fmt -- --check # Throw error if unformated code

cargo clippy # Advanced linter
cargo clippy --fix # Fix auto fixable

cargo build # Only build it

cargo test # Run all unit tests
cargo test <file without extension> # Run specific test file
cargo test <specific function name> # Run specific test function

cargo doc # Generates HTML documentation

cargo clean # Remove 'targer' directory (build artifacts, doc ...)
```

### Git hooks

Git hooks are handled with [rusty-hook](https://github.com/swellaby/rusty-hook)
