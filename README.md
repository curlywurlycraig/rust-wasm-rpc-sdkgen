# About

`rust-wasm-rpcgen` is a set of tools for easily generating a WASM package containing RPC bindings to tagged functions in an API.

Simply tagging a function with an attribute is enough to generate a WASM binding and API endpoint capable of servicing the binding:


```
#[remote]
pub fn get_todo() -> Todo {
  // Perhaps make some SQL query here.

  Todo {
    ...
  }
}
```

Such a function can be imported in the WASM part of the project and used as if it were a local function with the same function signature.

# Usage

See the examples directory for usage help.