use components::link::Link;
use components::me::Me;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Me />
            <Link />
            <marquee behavior="alternate" direction="right" scrollamount="15" width="100%" height="100%">
                <h1 style="color: #2f2f2f; font-size: 100px; font-family: 'Times New Roman', Times, serif;">{"SHIYUI DEV"}</h1>
            </marquee>
        </>
        
    }
}
fn main() {
    yew::start_app::<App>();
}
