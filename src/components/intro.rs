use yew::{function_component, html, Html};

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <>
        <h3>{"languages (Just a little bit)"}</h3>
        <ul>
            <li>{"Rust"}</li>
            <li>{"TypeScript(server)"}</li>
            <li>{"Python"}</li>
            <li>{"Lua"}</li>
            <li>{"Go"}</li>
            <li>{"C++"}</li>
        </ul>
        <h4>{"Others"}</h4>
        <ul>
            <li>{"Prisma"}</li>
            <li>{"Mariadb"}</li>
        </ul>
        </>
    }
}
