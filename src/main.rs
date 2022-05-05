use components::languages::Lang;
use components::link::Link;
use components::me::Me;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Me />
            <Lang />
            <Link />
        </>
    }
}
fn main() {
    yew::start_app::<App>();
}
