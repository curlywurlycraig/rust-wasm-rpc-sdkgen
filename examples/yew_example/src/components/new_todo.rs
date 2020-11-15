use yew::prelude::*;

use crate::todo::{Todo};

static INPUT_STYLE: &str = "margin-right: 20px";

pub struct NewTodo {
    link: ComponentLink<Self>,
    content: String,
    oncreate: Callback<Todo>,
    disabled: bool
}

pub enum Msg {
    UpdateTodoContent(String),
    Submit
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub oncreate: Callback<Todo>,
    pub disabled: bool
}

impl Component for NewTodo {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            content: String::from(""),
            oncreate: props.oncreate,
            disabled: props.disabled
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
        self.disabled = props.disabled;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    disabled=self.disabled
                    value=&self.content
                    oninput=self.link.callback(|inp: InputData| {
                       Msg::UpdateTodoContent(inp.value)
                    })
                    placeholder="Go for a bike ride"
                    style=INPUT_STYLE
                />

                <button
                    onclick=self.link.callback(|_| Msg::Submit)
                    disabled=self.disabled
                >
                    { "Add todo" }
                </button>
            </div>
        }
    }
}