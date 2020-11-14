use yew::prelude::*;

use crate::todo::{Todo};

pub struct TodoList {
    link: ComponentLink<Self>,
    todos: Vec<Todo>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>
}

impl Component for TodoList {
    type Message = ();
    type Properties = TodoListProps;
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: vec![],
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.todos = props.todos;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { for self.todos.iter().map(
                    |todo| html! {
                        <p>{ &todo.content }</p>
                    })
                }
            </div>
        }
    }
}