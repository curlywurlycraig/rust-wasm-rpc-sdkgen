#![cfg(feature = "backend")]

use std::sync::Mutex;
use crate::todo::Todo;

lazy_static! {
    static ref TODO_STORE: Mutex<Vec<Todo>> = Mutex::new(Vec::new());
}

pub fn get_todos() -> Vec<Todo> {
    TODO_STORE.lock().unwrap().clone()
}

pub fn add_todo(new_todo: Todo) {
    TODO_STORE.lock().unwrap().push(new_todo);
}

pub fn set_completed(id: String, completed: bool) {
    let mut store = TODO_STORE.lock().unwrap();
    store.iter_mut()
        .find(|todo| todo.id == id)
        .unwrap()
        .completed = completed;
}