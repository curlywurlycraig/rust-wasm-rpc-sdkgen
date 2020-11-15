use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

use crate::todo::{Todo, set_completed};

static CONTAINER_STYLE: &str = "";
static TODO_CONTAINER_STYLE: &str = "
display: flex;
flex-direction: row;
align-items: center";

pub struct TodoList {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub todos: Vec<Todo>,
    pub on_toggle_todo: Callback<String>
}

pub enum Msg {
    ToggleTodo(String)
}

impl Component for TodoList {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleTodo(todo_id) => {
                self.props.on_toggle_todo.emit(todo_id);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div style=CONTAINER_STYLE>
                { for self.props.todos.iter().map(
                    |todo| {
                        let todo_id = todo.id.clone();
                        html! {
                            <div style=TODO_CONTAINER_STYLE>
                                <input
                                    type="checkbox"
                                    checked=todo.completed
                                    onclick=self.link.callback(move |_| Msg::ToggleTodo(todo_id.clone()))
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