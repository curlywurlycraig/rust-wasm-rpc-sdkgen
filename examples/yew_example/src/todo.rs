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
pub fn add_todo(content: String) -> Todo {
    use crate::store;
    use uuid::Uuid;

    let new_todo = Todo {
        content,
        completed: false,
        id: Uuid::new_v4().hyphenated().to_string()
    };

    store::add_todo(new_todo.clone());

    new_todo
}

#[remote]
pub fn set_completed(id: String, completed: bool) {
    use crate::store;

    store::set_completed(id, completed);
}