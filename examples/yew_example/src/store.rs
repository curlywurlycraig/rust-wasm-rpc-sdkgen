#![cfg(feature = "backend")]

use std::collections::HashMap;
use std::sync::Mutex;
use crate::todo::Todo;

lazy_static! {
    static ref TODO_STORE: Mutex<HashMap<String, Todo>> = Mutex::new(HashMap::new());
}

pub fn get_todos() -> Vec<Todo> {
    TODO_STORE.lock().unwrap().iter().map(|(_, todo): (&String, &Todo)| todo.clone()).collect()
}

pub fn add_todo(new_todo: Todo) {
    TODO_STORE.lock().unwrap().insert(new_todo.id.clone(), new_todo);
}

pub fn set_completed(id: String, completed: bool) {
    let mut store = TODO_STORE.lock().unwrap();
    let existing = store.get(&id.clone()).unwrap().clone();
    store.insert(id, Todo {
        completed,
        id: existing.id,
        content: existing.content
    });
}