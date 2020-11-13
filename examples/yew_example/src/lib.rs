#![cfg(feature = "frontend")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod todo;
use todo::{mark_as_done};

struct Model {
    link: ComponentLink<Self>,
    value: String,
}

enum Msg {
    UpdateTodoContent(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTodoContent(content) => {
                self.value = content;
                let value_clone = self.value.clone();

                spawn_local(async move {
                    mark_as_done(1).await;
                });
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    value={&self.value}
                    oninput=self.link.callback(|inp: InputData| {
                       Msg::UpdateTodoContent(inp.value)
                    })
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
