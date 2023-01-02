use yew::{function_component, html};

#[function_component(Me)]
pub fn me() -> Html {
    html! {
        <div>
            <h1>{"Shiyui"}</h1>
            <img class="logo"  src="https://avatars.githubusercontent.com/u/66453922?v=4" alt="my icon" width="200" height="200"/>
            <br /><small>{"GitHub Icon"}</small><br /><br />
            <a>{"学生です。サーバーサイドが好きです。"}</a><br />
            <a>{"I'm a college student.
            I prefer server side.
            I like Go language."}</a><br />
        </div>
    }
}
