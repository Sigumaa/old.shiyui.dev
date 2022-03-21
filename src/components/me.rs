use yew::{function_component, html};

#[function_component(Me)]
pub fn me() -> Html {
    html! {
        <div>
            <h1>{"Shiyui"}</h1>
            <img class="logo" src="https://raw.githubusercontent.com/Sigumaa/Sigumaa/c24583e8d0ed0e62056f1c2c1567744f9b298d51/shiyui.jpeg" alt="my icon" />
        </div>
    }
}
