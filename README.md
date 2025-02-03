# Sabita (錆びた)

![Lint/Security/Dependencies/Tests](https://github.com/MikyStar/Sabita/actions/workflows/test-lint-audit.yml/badge.svg)
![crates.io version](https://img.shields.io/crates/v/sabita)

Sabita is a Sudoku solver / generator made with Rust

_'Sabita' (錆びた) means 'rusty' in japanese_

## CLI

### Install

**You will need to [install Rust](https://www.rust-lang.org/tools/install)**

```sh
cargo install sabita
```

### Use

```sh
# Generator
sabita g file/to/create # Generate a sudoku with no missing value
sabita g file/to/create 50 # Generate a sudoku with 50 missing values

# Solver
sabita s file/to/solve # Solve the sudoku (check out file sudoku.example) to see format

# Benchmark
sabita --benchmark # Launch the benchmarking script, see the 'Performances' section below

# Help
sabita -h
sabita --help

# Version
sabita -v
sabita --version
```

## Performances

Performances are benchmarked through the `src/core/bench.rs` and the results are written in the `benchmarks` folder

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

cargo add <package> [--dev] # Install a project dependency (or a dev dependency)
cargo install <package> # Install a system wide dependency

cargo doc # Generates HTML documentation

cargo clean # Remove 'targer' directory (build artifacts, doc ...)

cargo publish # Publish project to crates.io registry

cargo tree # Recursize list of lib dependencies
```

### Git hooks

Git hooks are handled with [rusty-hook](https://github.com/swellaby/rusty-hook), to enable them after a fresh install, run `cargo build`

### Tasks

Using [CLI-Manager](https://github.com/MikyStar/CLI-Manager) for task handling.
