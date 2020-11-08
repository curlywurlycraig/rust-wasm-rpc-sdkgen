#![cfg(feature = "frontend")]

use wasm_bindgen::prelude::*;

mod todo;
use todo::{Todo, get_todos as _get_todos, add_todo as _add_todo, mark_as_done as _mark_as_done};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn get_todos() -> JsValue {
    let todo: Vec<Todo> = _get_todos().await;

    JsValue::from_serde(&todo).unwrap()
}

#[wasm_bindgen]
pub async fn add_todo(content: JsValue) {
    let todo: Todo = content.into_serde().unwrap();
    _add_todo(todo).await;
}

#[wasm_bindgen]
pub async fn mark_as_done(content: JsValue) {
    let id: u8 = content.into_serde().unwrap();
    _mark_as_done(id).await;
}
