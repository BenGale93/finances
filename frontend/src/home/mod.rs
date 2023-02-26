mod accounts;
mod transactions;

use common::Config;
use reqwasm::http::Request;
use yew::prelude::*;

use crate::home::{accounts::AccountSummaryComponent, transactions::TransactionsComponent};

fn get_config(config_cb: Callback<Config>) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let fetched_config = Request::get("http://localhost:5000/config")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            config_cb.emit(fetched_config);
        }
    })
}

pub struct HomeComponent {
    config: Option<Config>,
}

pub enum HomeMsg {
    LoadConfig(Config),
}

impl Component for HomeComponent {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let home_cb = ctx.link().callback(HomeMsg::LoadConfig);
        get_config(home_cb);
        Self { config: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::LoadConfig(c) => {
                self.config = Some(c);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let budget = match &self.config {
            Some(c) => c.budget(),
            None => 0.0,
        };
        html! {
            <div>
            <div class="row">
            <div class="column left">
                <h2>{"Accounts"}</h2>
            </div>
            <div class="column right">
                <div class="wrapper">
                    <div class="info"><h2>{"Total"}</h2></div>
                    <div class="info"><h2>{"Budget: "}{budget}</h2></div>
                </div>
            </div>
        </div>
        <div class="row">
            <div class="input_tran">
            {"add transaction TODO"}
            </div>
        </div>
        <div class="row">
            <div class="column left">
            <table>
            <tr>
                <th>{"Account"}</th>
                <th>{"Amount"}</th>
            </tr>
            <AccountSummaryComponent />
            </table>
            </div>
            <div class="column right">
            <table>
            <tr>
                <th>{"Account"}</th>
                <th>{"Date"}</th>
                <th>{"Description"}</th>
                <th>{"Amount"}</th>
                <th>{"L1 Tag"}</th>
                <th>{"L2 Tag"}</th>
                <th>{"L3 Tag"}</th>
            </tr>
            <TransactionsComponent />
            </table>
            </div>
        </div>
        </div>
        }
    }
}
