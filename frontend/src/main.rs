#![recursion_limit = "256"]

use yew::prelude::*;
use yew_router::prelude::*;

mod home;

use home::HomeComponent;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
}

pub enum Msg {}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <HomeComponent /> }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
