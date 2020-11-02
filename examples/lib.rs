#![cfg(feature = "frontend")]

use wasm_bindgen::prelude::*;

mod todo;
use todo::{Todo, get_todo, add_todo};

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
pub async fn do_get_todo() -> JsValue {
    let todo: Todo = get_todo().await;

    JsValue::from_serde(&todo).unwrap()
}

#[wasm_bindgen]
pub async fn do_add_todo(content: JsValue) {
    let todo: Todo = content.into_serde().unwrap();
    add_todo(todo).await;
}
