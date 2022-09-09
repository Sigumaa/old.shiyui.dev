use yew::{function_component, html};

#[function_component(Me)]
pub fn me() -> Html {
    html! {
        <div>
            <h1>{"Shiyui"}</h1>
            <img class="logo" src="https://github.com/Sigumaa/Sigumaa/blob/main/shiyui.jpeg?raw=true" alt="my icon" />
        </div>
    }
}
