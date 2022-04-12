use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
                <h1> {"Yew Learning Diary"}</h1>
                <h2> {"Documenting the journey to learn the Yew framework for WebAssembly apps in Rust"} </h2>
        </header>

    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <p> { "Copyright 2022 Jeffery D Mitchell All Rights Reserved" } </p>
            <p><a href="https://github.com/sentinel1909/yew-learning-diary" target ="_blank"> {"GitHub repo for this site"}</a></p>
        </footer>

    }
}