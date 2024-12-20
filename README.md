# Sabi

_Sabi means rust in japanese_

Sabi is a Sudoku solver / generator made with Rust

## Dev

### Commands

```sh
cargo run # Builds and run the project

cargo fmt # Format code
cargo fmt -- --check # Throw error if unformated code

cargo clippy # Advanced linter
cargo clippy --fix # Fix auto fixable

cargo build # Only build it

cargo test # Run all unit tests
cargo test <file without extension> # Run specific test file
cargo test <specific function name># Run specific test function
```

### Git hooks

Git hooks are handled with [rusty-hook](https://github.com/swellaby/rusty-hook)
