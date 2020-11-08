### How to run this example

1. Generate wasm package: `wasm-pack build -- --features frontend`
2. Serve backend: `cargo run --bin todo_list --features backend`
3. Build and serve website: `cd www && npm install && npm run start`