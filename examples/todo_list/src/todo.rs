use rust_wasm_rpcgen::remote;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u8,
    pub content: String,
    pub completed: bool
}

#[remote]
pub fn get_todos() -> Vec<Todo> {
    vec![
        Todo { id: 10, content: String::from("Create new project"), completed: true },
        Todo { id: 10, content: String::from("Finish new project"), completed: false },
    ]
}

#[remote]
pub fn add_todo(new_todo: Todo) {
    println!("Got a new todo! {:?}", &new_todo);
}

#[remote]
pub fn mark_as_done(id: u8) {
    println!("Marking as done! {}", id);
}