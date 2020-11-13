# Running

Generate frontend:

`wasm-pack build --target web --out-name wasm --out-dir ./static -- --features frontend`

Serve frontend:

`miniserve ./static --index index.html`

Serve backend:

`cargo run --bin yew_example --features backend`