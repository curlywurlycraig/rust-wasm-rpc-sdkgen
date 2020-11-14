use warp::Filter;

mod todo;
mod store;

#[macro_use]
extern crate lazy_static;

use todo::{add_todo, get_todos, set_completed};

#[tokio::main]
async fn main() {
    let get_todo = warp::path!("rpc" / "get_todos")
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            get_todos(&body)
        }).with(warp::cors().allow_any_origin());

    let add_todo = warp::path!("rpc" / "add_todo")
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            add_todo(&body)
        }).with(warp::cors().allow_any_origin());

    let set_completed = warp::path!("rpc" / "set_completed")
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            set_completed(&body)
        }).with(warp::cors().allow_any_origin());

    let routes = get_todo
        .or(add_todo)
        .or(set_completed);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
