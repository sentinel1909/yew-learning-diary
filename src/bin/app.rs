// src/app.rs
// to run this app, type "trunk serve --open" at the command line at the top of the yew-learning-diary directory

use yew::prelude::*;

use yewlearningdiary::data::*;
use yewlearningdiary::components::*;

enum Msg {

}

struct App {}

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
        let entries = build_entries().into_iter().map(|entry| html! { 
            <article class="article">
                <p> {&entry.id} </p>
                <h3> {&entry.date} </h3>
                <h3> {&entry.title} </h3>
                <p> {&entry.content} </p>
            </article>
        }).collect::<Html>();
        html! {
            <main class="container">
                <Header />
                <Goals />
                <section class="section">   
                    { entries }
                </section>
                <Footer />
            </main>
        }
    }
}

// start up and mount root component into index.html
fn main() {
    yew::start_app::<App>();
}


