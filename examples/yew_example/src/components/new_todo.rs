use yew::prelude::*;

use crate::todo::{Todo};

pub struct NewTodo {
    link: ComponentLink<Self>,
    content: String,
    oncreate: Callback<Todo>
}

pub enum Msg {
    UpdateTodoContent(String),
    Submit
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub oncreate: Callback<Todo>
}

impl Component for NewTodo {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            content: String::from(""),
            oncreate: props.oncreate
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTodoContent(content) => {
                self.content = content;
            },
            Msg::Submit => {
                self.oncreate.emit(Todo {
                    content: self.content.clone(),
                    id: String::from("-1"),
                    completed: false
                });

                self.content = String::from("");
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    value={&self.content}
                    oninput=self.link.callback(|inp: InputData| {
                       Msg::UpdateTodoContent(inp.value)
                    })
                />

                <button onclick=self.link.callback(|_| Msg::Submit)>
                    { "Submit" }
                </button>
            </div>
        }
    }
}