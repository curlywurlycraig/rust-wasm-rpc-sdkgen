# About

The goal of this project is to create a tool for building full stack Rust web applications.
In particular, it aims to allow for the definition of functions that are callable
from both the front and backend.

## Running

1. Generate wasm lib: `wasm-pack build -- --features frontend`
2. Serve backend: `cargo run --bin ideal_http --features backend`

## Next steps

- [x] Work out how serialization/deserialization can be done. (Learn to use bincode)
- [x] Create a test rust wasm project to work out how to use wasm-bindgen
- [x] Create an HTTP server that accepts serialized bincode as a body and calls a function with it as an argument
- [x] Create a client that can call that endpoint, by creating bindgen signatures.
- [ ] Generate both a wasm + binary application from the same codebase
  - [x] Practise making attributes
  - [x] Create an attribute that generates the serialisation + deserialisation on backend
    - [x] Decode inputs
    - [x] Encode outputs
  - [x] Use conditional compilation
    - [x] Write frontend part of remote attribute
  - [x] Serialise and deserialise frontend
    - [x] Transform inputs into a tuple before serialisation
  - [ ] Consolidate handling of all rpc calls in same endpoint
- [ ] Serve pre-rendered page + wasm when visiting site.
  - [ ] Generate wasm lib output in first pass
  - [ ] Serve wasm and RPC endpoint in second pass
- [ ] Reduce .wasm size
- [ ] Future fun things
  - [ ] Security, virtualdom, etc

# Thoughts

Think about the ownership model. When calling a function that sends something over RPC for example, it can't be a reference because a reference can't be serialised (or can it?). So when passing an entity to the backend via an RPC call, who owns it?

# Example

An ideated application is shown below:

```rust
struct Todo {
    ...
}

#[remote]
fn get_todos() -> Vec<Todo> {

}

#[remote]
fn mark_as_done(todo: &Todo) {
    // SQL
}

#[url("/todos")]
fn main() {
    html!{
        <div>
            { get_todos().iter().map(|todo| {
                html!{
                    <button
                        on_click={|| {
                            mark_as_done(&todo)
                        }}
                    />
                }
            })}
        </div>
    }
}
```

Where the html is server-side rendered, and the `on_click` handler for the button is able to call `get_todos()` and `mark_as_done(todo)` from the frontend, even though only the corresponding backend logic needs to be written for them.