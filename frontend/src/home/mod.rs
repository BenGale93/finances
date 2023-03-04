mod accounts;
mod transaction_form;
mod transactions;

use std::sync::Arc;

use common::{AccountSummary, ConfigOptions, Transaction};
use yew::prelude::*;

use crate::{
    api,
    home::{
        accounts::AccountsSummaryComponent, transaction_form::TransactionForm,
        transactions::TransactionsComponent,
    },
};

pub struct AccountData {
    accounts: Option<Arc<Vec<AccountSummary>>>,
    total: Option<f64>,
}

pub struct TransactionsData {
    transactions: Option<Arc<Vec<Transaction>>>,
    page: (usize, usize),
}

pub enum HomeMsg {
    Error,
    RefreshData,
    NeedUpdateConfig,
    UpdateBudget(f64),
    NeedUpdateAccount,
    UpdateAccount(Vec<AccountSummary>),
    NeedUpdateTransactions,
    UpdateTransactions(Vec<Transaction>),
    Back,
    Forward,
}

pub struct HomeComponent {
    account_data: AccountData,
    transactions_data: TransactionsData,
    budget: Option<f64>,
}

impl Component for HomeComponent {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            account_data: AccountData {
                accounts: None,
                total: None,
            },
            transactions_data: TransactionsData {
                transactions: None,
                page: (0, 50),
            },
            budget: None,
        };

        ctx.link().send_message(Self::Message::NeedUpdateConfig);
        ctx.link().send_message(Self::Message::RefreshData);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;
        match msg {
            HomeMsg::Error => (),
            HomeMsg::RefreshData => {
                ctx.link().send_message(HomeMsg::NeedUpdateAccount);
                ctx.link().send_message(HomeMsg::NeedUpdateTransactions);
            }
            HomeMsg::NeedUpdateAccount => {
                ctx.link()
                    .send_future(async move { HomeMsg::UpdateAccount(api::get_accounts().await) });
            }
            HomeMsg::NeedUpdateConfig => {
                ctx.link().send_future(async move {
                    let budget_config = api::get_config("budget").await;
                    match budget_config {
                        ConfigOptions::Budget(b) => HomeMsg::UpdateBudget(b),
                        _ => HomeMsg::Error,
                    }
                });
            }
            HomeMsg::NeedUpdateTransactions => {
                let (offset, limit) = self.transactions_data.page;
                ctx.link().send_future(async move {
                    HomeMsg::UpdateTransactions(api::get_transactions(offset, limit).await)
                });
            }
            HomeMsg::UpdateAccount(accounts) => {
                let total = accounts.iter().map(|a| a.amount).sum();
                self.account_data = AccountData {
                    accounts: Some(Arc::new(accounts)),
                    total: Some(total),
                };
                should_render = true;
            }
            HomeMsg::UpdateBudget(b) => {
                self.budget = Some(b);
                should_render = true;
            }
            HomeMsg::UpdateTransactions(transactions) => {
                if transactions.is_empty() {
                    /* Gone too far, let's go back */
                    ctx.link().send_message(HomeMsg::Forward);
                    should_render = false;
                } else {
                    self.transactions_data.transactions = Some(Arc::new(transactions));
                    should_render = true;
                }
            }
            HomeMsg::Back => {
                let transactions = match &self.transactions_data.transactions {
                    Some(t) => t,
                    None => return false,
                };
                if transactions.iter().len() == self.transactions_data.page.1 {
                    self.transactions_data.page.0 += self.transactions_data.page.1;
                    ctx.link().send_message(HomeMsg::NeedUpdateTransactions);
                } else {
                    should_render = true;
                }
            }
            HomeMsg::Forward => {
                self.transactions_data.page.0 = self
                    .transactions_data
                    .page
                    .0
                    .saturating_sub(self.transactions_data.page.1);
                ctx.link().send_message(HomeMsg::NeedUpdateTransactions);
            }
        }
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total = match &self.account_data.total {
            Some(t) => format!("{t:.2}"),
            None => return "".into(),
        };

        let accounts = match &self.account_data.accounts {
            Some(a) => a,
            None => return "".into(),
        };

        let budget = match &self.budget {
            Some(b) => b,
            None => return "".into(),
        };
        let transactions = match &self.transactions_data.transactions {
            Some(t) => t,
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
            <TransactionForm on_submit={ctx.link().callback(|_| HomeMsg::RefreshData)}/>
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
            <TransactionsComponent transactions={transactions.clone()}/>
            <button onclick={ctx.link().callback(|_| HomeMsg::Back)}>{"back"}</button>
            <button onclick={ctx.link().callback(|_| HomeMsg::Forward)}>{"forward"}</button>
            </div>
        </div>
        </div>
        }
    }
}
