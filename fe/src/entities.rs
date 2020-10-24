use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Uint8Array;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, Request, RequestMode, RequestInit};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u8,
    pub content: String,
    pub completed: bool
}

/// What work does this function do?
/// 1. Get the window handle.
/// 2. Make a fetch call to the RPC endpoint.
/// 3. Deserialize the result.
///
/// All of the above could be generalized and automated at build time to produce
/// a kind of RPC SDK on the fly.
///
/// That can be done by using an attribute #[bind_frontend] which generates
pub async fn get_todo() -> Todo {
    let window = web_sys::window().unwrap();

    // This actually should be called in some init function.
    // It also should be disabled in production to make the wasm smaller.
    console_error_panic_hook::set_once();

    // Serialize input
    // let input_as_bytes: Vec<u8> = bincode::serialize(&new_todo).unwrap();
    // println!("input as bytes is {:?}", input_as_bytes);

    // Create request
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(None);

    let request = Request::new_with_str_and_init("http://localhost:3030/get_todo", &opts).unwrap();

    let response: Response = JsFuture::from(window
        .fetch_with_request(&request)).await.unwrap()
        .dyn_into()
        .expect("Failed to cast to response.");

    let bin: Vec<u8> = JsFuture::from(response.text().unwrap()).await.unwrap()
        .as_string().unwrap().into_bytes();

    let todo: Todo = bincode::deserialize(&bin[..]).unwrap();

    todo
}

pub async fn add_todo(new_todo: &Todo) {
    let window = web_sys::window().unwrap();

    // This actually should be called in some init function.
    // It also should be disabled in production to make the wasm smaller.
    console_error_panic_hook::set_once();

    // Serialize input
    let input_as_bytes: Vec<u8> = bincode::serialize(&new_todo).unwrap();
    println!("input as bytes is {:?}", input_as_bytes);

    // Create request
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&Uint8Array::from(&input_as_bytes[..])));

    let request = Request::new_with_str_and_init("http://localhost:3030/add_todo", &opts).unwrap();

    let response: Response = JsFuture::from(window
        .fetch_with_request(&request)).await.unwrap()
        .dyn_into()
        .expect("Failed to cast to response.");

    // let bin: Vec<u8> = JsFuture::from(response.text().unwrap()).await.unwrap()
        // .as_string().unwrap().into_bytes();

    // let todo: Todo = bincode::deserialize(&bin[..]).unwrap();
}

#[wasm_bindgen]
pub async fn do_get_todo() -> JsValue {
    let todo: Todo = get_todo().await;

    JsValue::from_serde(&todo).unwrap()
}

#[wasm_bindgen]
pub async fn do_add_todo(content: JsValue) {
    let todo: Todo = content.into_serde().unwrap();
    add_todo(&todo).await;
}
