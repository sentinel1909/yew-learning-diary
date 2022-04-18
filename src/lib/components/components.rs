use yew::{function_component, html, use_state};
use chrono::Datelike;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
                <h1> {"Yew (and Rust) Learning Diary"}</h1>
                <h2> {"Documenting the journey to learn Rust and the Yew framework for WebAssembly"} </h2>
        </header>

    }
}

#[function_component(Goals)]
pub fn goals() -> Html {
    html! {
        <section>
            <h3> {"Site Goals:"}</h3>
            <p> {"Further breakdown components into smaller bits...they're all in one file currently."} </p>
            <p> {"Get some images incorporated into the site."} </p>
            <p> {"Improve formatting...current layout is my usual hacky css and it sucks."} </p>
            <p> {"Diary entries should be pulled from a database via an API. All data is hard-coded into the app, diary entry IDs regenerate on each page load and they shouldn't."} </p>
        </section>
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