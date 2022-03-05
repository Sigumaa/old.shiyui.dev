use yew::{function_component, html, Html};

#[function_component(Me)]
pub fn me() -> Html {
    html! {
        <div>
            <h1>{"Hi,There👋"}</h1>
            <img class="logo" src="https://raw.githubusercontent.com/Sigumaa/Sigumaa/main/shiyui.jpeg" alt="my icon" />
        </div>
    }
}
