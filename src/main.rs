use components::languages::Lang;
use components::me::Me;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div >
            <Me />
            <Lang />
        </div>
    }
}
fn main() {
    yew::start_app::<App>();
}
