#![cfg(feature = "frontend")]
#![recursion_limit="1024"]

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod todo;

mod components;
use components::todo_list::TodoList;
use components::new_todo::NewTodo;
use todo::{Todo, get_todos, add_todo};

struct AppRoot {
    link: ComponentLink<Self>,
    todos: Vec<Todo>,
    pending_add_todo: bool
}

enum AppRootMessage {
    Refresh,
    GotTodos(Vec<Todo>),
    AddTodo(Todo),
    AddTodoSuccess(Todo)
}

impl Component for AppRoot {
    type Message = AppRootMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: vec![],
            pending_add_todo: false
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
            },
            AppRootMessage::AddTodo(new_todo) => {
                self.pending_add_todo = true;

                let link_clone = self.link.clone();
                let cloned_content = new_todo.content.clone();
                spawn_local(async move {
                    let new_todo = add_todo(cloned_content).await;
                    link_clone.send_message(AppRootMessage::AddTodoSuccess(new_todo))
                });
            },
            AppRootMessage::AddTodoSuccess(new_todo) => {
                self.todos.push(new_todo);
                self.pending_add_todo = false;
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
                <NewTodo
                    oncreate=self.link.callback(|todo| AppRootMessage::AddTodo(todo))
                    disabled=self.pending_add_todo
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppRoot>::new().mount_to_body();
}
