use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

use crate::todo::{Todo, set_completed};

pub struct TodoList {
    link: ComponentLink<Self>,
    todos: Vec<Todo>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>
}

pub enum TodoListMessage {
    ToggleTodo(String)
}

impl Component for TodoList {
    type Message = TodoListMessage;
    type Properties = TodoListProps;
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoListMessage::ToggleTodo(todo_id) => {
                spawn_local(async move {
                    set_completed(todo_id, true).await;
                });
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.todos = props.todos;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { for self.todos.iter().map(
                    |todo| {
                        let todo_id = todo.id.clone();
                        html! {
                            <div style="display: flex; flex-direction: row; align-items: center">
                                <input
                                    type="checkbox"
                                    checked=todo.completed
                                    onclick=self.link.callback(move |_| TodoListMessage::ToggleTodo(todo_id.clone()))
                                />
                                <p>{ &todo.content }</p>
                            </div>
                        }
                    })
                }
            </div>
        }
    }
}