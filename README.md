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
  - [ ] Create an attribute that generates the serialisation + deserialisation on backend
    - [ ] Decode inputs
    - [ ] Encode outputs
  - [ ] Do the same for frontend
    - [ ] Need to transform inputs into a tuple before serialisation
  - [ ] Generalise endpoint solution for both todo tasks
  - [ ] Generalise frontend solution for both todo tasks (serialisation + deserialisation)
  - [ ] Use conditional compilation https://doc.rust-lang.org/reference/conditional-compilation.html
  - [ ] Serve pre-rendered page + wasm when visiting site.
- [ ] Reduce .wasm size
- [ ] Future fun things
  - [ ] Security, virtualdom, etc