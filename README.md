# Overview

*Search-logger* uses the same protocol as CP-Profiler and it is designed to record coming from a solver messages (nodes and commands) to a file.
The resulting file is readable by [*search-reader*](https://github.com/cp-profiler/search-reader) to simulate the original solver execution.

# Building

- make sure that Rust and Cargo are installed (can be installed using [rustup](https://www.rustup.rs))
- execute `cargo build --release` in the same directory as `Cargo.toml`

