#![recursion_limit = "256"]

use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod balance;
mod budget;
mod home;
mod monthly;

use balance::BalanceComponent;
use budget::BudgetComponent;
use home::HomeComponent;
use monthly::MonthlyComponent;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/balance")]
    Balance,
    #[at("/budget")]
    Budget,
    #[at("/monthly")]
    Monthly,
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div>
                    <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                    <Link<Route> to={Route::Balance}>{"Balance History"}</Link<Route>>
                    <Link<Route> to={Route::Budget}>{"Budget Progress"}</Link<Route>>
                    <Link<Route> to={Route::Monthly}>{"Monthly  Summary"}</Link<Route>>
                </div>
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
        Route::Balance => {
            html! { <BalanceComponent /> }
        }
        Route::Budget => {
            html! { <BudgetComponent /> }
        }
        Route::Monthly => {
            html! { <MonthlyComponent /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
