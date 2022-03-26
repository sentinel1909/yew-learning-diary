// src/main.rs
// to run this app, type "trunk serve --open" at the command line at the top of the yew-learning-diary directory

// pull in the yew crate 
use yew::prelude::*;

/// struct to represent a diary entry
struct Entry {
    id: usize,
    date: String,
    title: String,
    content: String,
}

/// enum for the Msg type
enum Msg {

}

/// struct for the root component "App"
struct App {}

/// implementation block for the App component
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // declare a vector of diary entries
        let entries = vec![
            Entry {
                id: 1,
                date: String::from("2022-03-26"),
                title: String::from("It Lives!"),
                content: String::from("Barebones site up and running.")
            }
        ];
        html! {
            <main>
                <header>
                    <h1> {"Yew Learning Diary"}</h1>
                    <h2> {"Documenting the journey to learn the Yew framework for WebAssembly apps in Rust"} </h2>
                </header>
                <section>   // render the diary entries, so far there's just one...need to iterate on this eventually
                    <h3> {&entries[0].date} </h3>
                    <h3> {&entries[0].title} </h3>
                    <p> {&entries[0].content} </p>
                </section>
                <footer>
                    <p> { "Copyright 2022 Jeffery D Mitchell All Rights Reserved" } </p>
                </footer>
            </main>
        }
    }
}

/// start up and mount App component into index.html
fn main() {
    yew::start_app::<App>();
}


