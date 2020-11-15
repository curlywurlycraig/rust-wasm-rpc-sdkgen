#![cfg(feature = "frontend")]
#![recursion_limit="1024"]

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod todo;

mod components;
use components::todo_list::TodoList;
use components::new_todo::NewTodo;
use todo::{Todo, get_todos, add_todo, set_completed};

use wasm_logger;
use log;

static OUTER_CONTAINER_STYLE: &str = "
display: flex;
flex-direction: column;
align-items: center;";

static INNER_CONTAINER_STYLE: &str = "
max-width: 400px;
width: 400px;";

struct AppRoot {
    link: ComponentLink<Self>,
    todos: Vec<Todo>,
    pending_add_todo: bool,
    pending_get_todos: bool
}

enum AppRootMessage {
    Refresh,
    GotTodos(Vec<Todo>),
    AddTodo(Todo),
    AddTodoSuccess(Todo),
    ToggleTodo(String)
}

impl Component for AppRoot {
    type Message = AppRootMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_clone = link.clone();
        spawn_local(async move {
            let todos = get_todos().await;
            link_clone.send_message(AppRootMessage::GotTodos(todos));
        });

        Self {
            link,
            todos: vec![],
            pending_add_todo: false,
            pending_get_todos: true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppRootMessage::Refresh => {
                let link_clone = self.link.clone();
                spawn_local(async move {
                    let todos = get_todos().await;
                    link_clone.send_message(AppRootMessage::GotTodos(todos));
                });
            },
            AppRootMessage::GotTodos(new_todos) => {
                self.todos = new_todos;
                self.pending_get_todos = false;
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
            },
            AppRootMessage::ToggleTodo(todo_id) => {
                let todo_id_clone = todo_id.clone();
                let mut todo = self.todos.iter_mut()
                    .find(move |todo| todo.id == todo_id_clone)
                    .unwrap();

                todo.completed = !todo.completed;

                let completed_clone = todo.completed.clone();
                let todo_id_clone = todo_id.clone();
                spawn_local(async move {
                    set_completed(todo_id_clone, completed_clone).await;
                });
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.pending_get_todos {
            return html! {
                <></>
            };
        };

        html! {
            <div style=OUTER_CONTAINER_STYLE>
                <div style=INNER_CONTAINER_STYLE>
                    <button onclick=self.link.callback(|_| AppRootMessage::Refresh)>{ "Refresh" }</button>
                    <TodoList
                        todos=self.todos.clone()
                        on_toggle_todo=self.link.callback(|todo_id| AppRootMessage::ToggleTodo(todo_id))
                    />
                    <NewTodo
                        oncreate=self.link.callback(|todo| AppRootMessage::AddTodo(todo))
                        disabled=self.pending_add_todo
                    />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<AppRoot>::new().mount_to_body();
}
