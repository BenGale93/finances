mod accounts;
mod transactions;

use std::sync::Arc;

use common::{AccountSummary, Config, Transaction};
use yew::prelude::*;

use crate::{
    callbacks,
    home::{accounts::AccountSummaryComponent, transactions::TransactionsComponent},
};

pub struct HomeComponent {
    config: Option<Config>,
    accounts: Option<Arc<Vec<AccountSummary>>>,
    transactions: Option<Arc<Vec<Transaction>>>,
}

pub enum HomeMsg {
    LoadConfig(Config),
    LoadAccounts(Vec<AccountSummary>),
    LoadTransactions(Vec<Transaction>),
}

impl Component for HomeComponent {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let home_cb = ctx.link().callback(HomeMsg::LoadConfig);
        callbacks::get_config(home_cb);
        let account_cb = ctx.link().callback(HomeMsg::LoadAccounts);
        callbacks::get_accounts(account_cb);
        let transactions_cb = ctx.link().callback(HomeMsg::LoadTransactions);
        callbacks::get_transactions(transactions_cb);
        Self {
            config: None,
            accounts: None,
            transactions: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::LoadConfig(c) => {
                self.config = Some(c);
            }
            HomeMsg::LoadAccounts(a) => {
                self.accounts = Some(Arc::new(a));
            }
            HomeMsg::LoadTransactions(t) => {
                self.transactions = Some(Arc::new(t));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let budget = match &self.config {
            Some(c) => c.budget(),
            None => return html! {<></>},
        };
        let accounts = match &self.accounts {
            Some(a) => a,
            None => return html! {<></>},
        };
        let transactions = match &self.transactions {
            Some(t) => t,
            None => return html! {<></>},
        };
        let total: f64 = accounts.iter().map(|a| a.amount).sum();
        let total = format!("{total:.2}");

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
            <AccountSummaryComponent accounts={accounts.clone()} />
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
            <TransactionsComponent transactions={transactions.clone()} />
            </table>
            </div>
        </div>
        </div>
        }
    }
}
