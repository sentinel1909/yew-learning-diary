// src/app.rs
// to run this app, type "trunk serve --open" at the command line at the top of the yew-learning-diary directory

use yew::prelude::*;
use yew::html::Scope;
use yew_router::prelude::*;

use yewlearningdiary::pages::Home;
use yewlearningdiary::pages::About;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/About")]
    About,
}

struct App {}  

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {        
        html! {
            <BrowserRouter>
                <main>
                    { self.nav_bar(ctx.link()) }
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl App {
    fn nav_bar(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class={classes!("navbar")}>
                <Link<Route> to={Route::Home}>
                    { "Home | " }
                </Link<Route>>
                <Link<Route> to={Route::About}>
                    { "About" }
                </Link<Route>>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
         },
         Route::About => {
             html! { <About /> }
         }
    }
}

fn main() {
    yew::start_app::<App>();
}


