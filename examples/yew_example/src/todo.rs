use rust_wasm_rpcgen::remote;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Todo {
    pub id: String,
    pub content: String,
    pub completed: bool
}

#[remote]
pub fn get_todos() -> Vec<Todo> {
    use crate::store;
    store::get_todos()
}

#[remote]
pub fn add_todo(content: String) {
    use crate::store;
    use uuid::Uuid;

    store::add_todo(Todo {
        content,
        completed: false,
        id: Uuid::new_v4().hyphenated().to_string()
    });
}

#[remote]
pub fn mark_as_done(id: u8) {
    println!("Marking as done! {}", id);
}