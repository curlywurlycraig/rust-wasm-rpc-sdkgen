use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::todo::{add_todo, get_todos};

pub struct NewTodo {
    link: ComponentLink<Self>,
    content: String,
}

pub enum Msg {
    UpdateTodoContent(String),
    Submit
}

impl Component for NewTodo {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            content: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTodoContent(content) => {
                self.content = content;
            },
            Msg::Submit => {
                let cloned_content = self.content.clone();
                spawn_local(async move {
                    add_todo(cloned_content).await;
                    get_todos().await;
                });
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