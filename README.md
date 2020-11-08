# About

The goal of this project is to create a tool for building full stack Rust web applications.
In particular, it aims to allow for the definition of functions that are callable
from both the front and backend.

## Running

1. Generate wasm lib: `wasm-pack build -- --features frontend`
2. Serve backend: `cargo run --bin ideal_http --features backend`

# Example

See the examples directory for usage help.