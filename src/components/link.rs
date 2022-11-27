use yew::{function_component, html};

#[function_component(Link)]
pub fn link() -> Html {
    html! {
        <>
        <h3>{"Links"}</h3>
        <ul>
            <li><a rel="me" href="https://twitter.com/shiii_i_">{"Twitter"}</a></li>
            <li><a rel="me" href="https://github.com/Sigumaa">{"GitHub"}</a></li>
            <li><a rel="me" href="https://qiita.com/Siguma">{"Qiita"}</a></li>
            <li><a rel="me" href="https://zenn.dev/shiyui">{"Zenn"}</a></li>
            <li><a rel="me" href="https://filmarks.com/users/shiii_i_">{"Filmarks"}</a></li>
            <li><a rel="me" href="https://shiyui.hatenablog.com/">{"Hatena Blog"}</a></li>
        </ul>
        </>
    }
}
