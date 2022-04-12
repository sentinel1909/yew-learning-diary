use yew::{function_component, html, use_state};
use chrono::Datelike;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
                <h1> {"Yew Learning Diary"}</h1>
                <h2> {"Documenting the journey to learn the Yew framework for Rust WebAssembly"} </h2>
        </header>

    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let time = use_state(|| chrono::Local::now().date() );
    html! {
        <footer class="footer">
            <p> { "Copyright" } {" "} {time.year()} {" "} { "Jeffery D Mitchell All Rights Reserved" } </p>
            <p><a href="https://github.com/sentinel1909/yew-learning-diary" target ="_blank"> {"GitHub repo for this site"}</a></p>
        </footer>

    }
}