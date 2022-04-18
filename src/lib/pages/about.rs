use yew::prelude::*;

use crate::components::Header;
use crate::components::Footer;

pub struct About {}  

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {        
        html! {
            <main class="container">
                <Header />
                <h3> { "About Me" }</h3>
                <p> { "Coming soon..." }</p>
                <Footer />
            </main>
        }
    }
}