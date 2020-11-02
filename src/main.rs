use warp::Filter;

mod todo;
use todo::{add_todo, get_todo};

#[tokio::main]
async fn main() {
    let get_todo = warp::path!("rpc" / "get_todo")
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            get_todo(&body)
        }).with(warp::cors().allow_any_origin());

    let add_todo = warp::path!("rpc" / "add_todo")
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            add_todo(&body)
        }).with(warp::cors().allow_any_origin());

    let routes = get_todo
        .or(add_todo);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
