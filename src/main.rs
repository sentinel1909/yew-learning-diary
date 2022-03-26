// src/main.rs
// to run this app, type "trunk serve --open" at the command line at the top of the yew-learning-diary directory

// pull in the yew crate 
use yew::prelude::*;

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
        html! {
            <div>
                <h1> {"Yew Learning Diary"}</h1>
                <h2> {"Documenting the journey to learn the Yew framework for WebAssembly apps in Rust"} </h2>
            </div>
        }
    }
}

/// start up and mount App component into index.html
fn main() {
    yew::start_app::<App>();
}


