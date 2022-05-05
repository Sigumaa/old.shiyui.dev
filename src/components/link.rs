use yew::{function_component, html};

#[function_component(Link)]
pub fn link() -> Html {
    html! {
        <>
        <h3>{"Links"}</h3>
        <ul>
            <li><a rel="me" href="https://twitter.com/shiii_i_">{"Twitter"}</a></li>
            <li><a rel="me" href="https://pawoo.net/@sushiyui">{"Mastodon"}</a></li>
        </ul>
        </>
    }
}
