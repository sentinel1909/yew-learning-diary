use yew::prelude::*;

use crate::components::Header;
use crate::components::Goals;
use crate::components::Footer;
use crate::data::build_entries;

pub struct Home {}  

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {        
        let entries = build_entries().into_iter().map(|entry| html! { 
            <article class="article">
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
