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
            <p> {"Get diary entries into a database, they are currently drawn from a hardcode data file."} </p>
            <p> {"Learn Yew Router to add ability to navigate between different pages."} </p>
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