use components::intro::Intro;
use components::me::Me;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div >
            <Me />
            <Intro />
        </div>
    }
}
fn main() {
    yew::start_app::<App>();
}
