mod accounts;
mod transactions;

use std::sync::Arc;

use common::{AccountSummary, Config};
use yew::prelude::*;

use crate::{
    api,
    home::{accounts::AccountsSummaryComponent, transactions::TransactionsComponent},
};

pub struct HomeData {
    accounts: Option<Arc<Vec<AccountSummary>>>,
    total: Option<f64>,
}

pub enum HomeMsg {
    NeedUpdateConfig,
    UpdateConfig(Config),
    NeedUpdateData,
    UpdateData(Vec<AccountSummary>),
}

pub struct HomeComponent {
    data: HomeData,
    config: Option<Arc<Config>>,
}

impl Component for HomeComponent {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            data: HomeData {
                accounts: None,
                total: None,
            },
            config: None,
        };

        ctx.link().send_message(Self::Message::NeedUpdateData);
        ctx.link().send_message(Self::Message::NeedUpdateConfig);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;
        match msg {
            HomeMsg::NeedUpdateData => {
                ctx.link()
                    .send_future(async move { HomeMsg::UpdateData(api::get_accounts().await) });
            }
            HomeMsg::NeedUpdateConfig => {
                ctx.link()
                    .send_future(async move { HomeMsg::UpdateConfig(api::get_config().await) });
            }
            HomeMsg::UpdateData(accounts) => {
                let total = accounts.iter().map(|a| a.amount).sum();
                self.data = HomeData {
                    accounts: Some(Arc::new(accounts)),
                    total: Some(total),
                };
                should_render = true;
            }
            HomeMsg::UpdateConfig(config) => {
                self.config = Some(Arc::new(config));
                should_render = true;
            }
        }
        should_render
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let total = match &self.data.total {
            Some(t) => format!("{t:.2}"),
            None => return "".into(),
        };

        let accounts = match &self.data.accounts {
            Some(a) => a,
            None => return "".into(),
        };

        let budget = match &self.config {
            Some(c) => c.budget(),
            None => return "".into(),
        };

        html! {
            <div>
            <div class="row">
            <div class="column left">
                <h2>{"Accounts"}</h2>
            </div>
            <div class="column right">
                <div class="wrapper">
                    <div class="info"><h2>{"Total: £"}{total}</h2></div>
                    <div class="info"><h2>{"Budget: £"}{budget}</h2></div>
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
            <AccountsSummaryComponent accounts={accounts.clone()} />
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
