use components::me::Me;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
            <Me />
    }
}
fn main() {
    yew::start_app::<App>();
}
