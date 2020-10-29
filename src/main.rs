use bincode;
use serde::{Serialize, Deserialize};
use warp::Filter;
use remote_attr::remote;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u8,
    pub content: String,
    pub completed: bool
}

#[remote]
fn get_todo() -> Todo {
    Todo { id: 10, content: String::from("Hello craig!"), completed: false }
}

#[remote]
fn add_todo(mut new_todo: Todo) {
    println!("Got a new todo! {:?}", new_todo);
}

#[tokio::main]
async fn main() {
    let get_todo = warp::path!("get_todo")
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            get_todo(&body)
        }).with(warp::cors().allow_any_origin());

    let add_todo = warp::path!("add_todo")
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
