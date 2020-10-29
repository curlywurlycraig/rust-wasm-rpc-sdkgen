# About

The goal of this project is to create a tool for building full stack Rust web applications.
In particular, it aims to allow for the definition of functions that are callable
from both the front and backend.

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
  - [ ] Do the same for frontend
    - [ ] Need to transform inputs into a tuple before serialisation
  - [ ] Generalise endpoint solution for both todo tasks
  - [ ] Generalise frontend solution for both todo tasks (serialisation + deserialisation)
  - [ ] Use conditional compilation https://doc.rust-lang.org/reference/conditional-compilation.html
  - [ ] Serve pre-rendered page + wasm when visiting site.
- [ ] Reduce .wasm size
- [ ] Future fun things
  - [ ] Security, virtualdom, etc


# Example

An ideated application is shown below:

```rust
#[mirror]
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