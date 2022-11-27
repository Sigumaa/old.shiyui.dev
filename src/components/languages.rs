use yew::{function_component, html};

#[function_component(Lang)]
pub fn lang() -> Html {
    html! {
        <>
        <h3>{"lang"}</h3>
        <ul>
            <li>{"Go"}</li>
            <li>{"Rust"}</li>
            <li>{"TypeScript"}</li>
            <li>{"Python"}</li>
            <li>{"Lua"}</li>
            <li>{"C++"}</li>
        </ul>
        <a>{"GoやRustが好きです。TSも書きます。"}</a><br/>
        <a>{"WebSocket楽しい"}</a><br/>
        <a>{"Pythonは書きたくない"}</a><br/>
        <a>{"C++はごくまれに書きます。"}</a>
        </>
    }
}
