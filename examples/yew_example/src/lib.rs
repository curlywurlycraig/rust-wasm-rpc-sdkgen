#![cfg(feature = "frontend")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod todo;

mod components;
use components::todo_list::TodoList;
use components::new_todo::NewTodo;
use todo::{Todo, get_todos};

struct AppRoot {
    link: ComponentLink<Self>,
    todos: Vec<Todo>
}

enum AppRootMessage {
    Refresh,
    GotTodos(Vec<Todo>)
}

impl Component for AppRoot {
    type Message = AppRootMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: vec![]
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppRootMessage::Refresh => {
                let link = self.link.clone();
                spawn_local(async move {
                    let todos = get_todos().await;
                    link.send_message(AppRootMessage::GotTodos(todos));
                });
            },
            AppRootMessage::GotTodos(new_todos) => {
                self.todos = new_todos;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| AppRootMessage::Refresh)>{ "Refresh" }</button>
                <TodoList todos=self.todos.clone() />
                <NewTodo />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppRoot>::new().mount_to_body();
}
