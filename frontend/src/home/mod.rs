mod accounts;
mod fields;
mod transaction_form;
mod transactions;

use std::sync::Arc;

use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use common::{AccountSummary, Config, ConfigOptions, Transaction};
use yew::prelude::*;

use crate::{
    api,
    home::{
        accounts::AccountsSummaryComponent, transaction_form::CreateForm,
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
    UpdateConfig(Config),
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
    config: Option<Arc<Config>>,
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
            config: None,
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
                log::info!("Refreshing data.");
                ctx.link().send_message(HomeMsg::NeedUpdateAccount);
                ctx.link().send_message(HomeMsg::NeedUpdateTransactions);
            }
            HomeMsg::NeedUpdateAccount => {
                log::info!("Getting all accounts.");
                ctx.link()
                    .send_future(async move { HomeMsg::UpdateAccount(api::get_accounts().await) });
            }
            HomeMsg::NeedUpdateConfig => {
                ctx.link().send_future(async move {
                    let config = api::get_config("all").await;
                    match config {
                        ConfigOptions::All(c) => HomeMsg::UpdateConfig(c),
                        _ => HomeMsg::Error,
                    }
                });
            }
            HomeMsg::UpdateConfig(c) => {
                self.config = Some(Arc::new(c));
                should_render = true;
            }
            HomeMsg::NeedUpdateTransactions => {
                log::info!("Updating transactions {:?}", self.transactions_data.page);
                let (offset, limit) = self.transactions_data.page;
                ctx.link().send_future(async move {
                    HomeMsg::UpdateTransactions(api::get_transactions(offset, limit).await)
                });
            }
            HomeMsg::UpdateAccount(accounts) => {
                log::info!("Got all accounts.");
                let total = accounts.iter().map(|a| a.amount).sum();
                self.account_data = AccountData {
                    accounts: Some(Arc::new(accounts)),
                    total: Some(total),
                };
                should_render = true;
            }
            HomeMsg::UpdateTransactions(transactions) => {
                if transactions.is_empty() {
                    log::info!("Empty transactions");
                    /* Gone too far, let's go back */
                    ctx.link().send_message(HomeMsg::Forward);
                    should_render = false;
                } else {
                    self.transactions_data.transactions = Some(Arc::new(transactions));
                    should_render = true;
                }
            }
            HomeMsg::Back => {
                log::info!("Going back");
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
                log::info!("Going forward");
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

        let config = match &self.config {
            Some(c) => c,
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
                    <div class="info"><h2>{"Budget: £"}{config.budget()}</h2></div>
                </div>
            </div>
        </div>
        <div class="row">
            <div class="input_tran">
            <CreateForm on_submit={ctx.link().callback(|_| HomeMsg::RefreshData)} config={config.clone()}/>
            </div>
        </div>
        <div class="row">
            <div class="column left">
            <table class="accounts">
            <tr>
                <th>{"Account"}</th>
                <th>{"Amount"}</th>
            </tr>
            <AccountsSummaryComponent accounts={accounts.clone()} />
            </table>
            </div>
            <div class="column right">
            <TransactionsComponent transactions={transactions.clone()}
            on_submit={ctx.link().callback(|_| HomeMsg::RefreshData)}
            config={config.clone()}/>
            <button onclick={ctx.link().callback(|_| HomeMsg::Back)}>{"back"}</button>
            <button onclick={ctx.link().callback(|_| HomeMsg::Forward)}>{"forward"}</button>
            </div>
        </div>
        </div>
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserTransaction {
    pub id: i64,
    pub account: AttrValue,
    pub date: AttrValue,
    pub description: AttrValue,
    pub amount: AttrValue,
    pub l1_tag: AttrValue,
    pub l2_tag: AttrValue,
    pub l3_tag: AttrValue,
}

impl UserTransaction {
    pub fn to_transaction(&self, config: &Config) -> anyhow::Result<Transaction> {
        let id = self.id;
        let account = if config
            .account_list()
            .iter()
            .any(|a| a == self.account.as_str())
        {
            self.account.to_owned()
        } else {
            return Err(anyhow!("Bad account {:?}.", &self.account));
        };

        let date = match NaiveDate::parse_from_str(&self.date, "%Y-%m-%d") {
            Ok(d) => NaiveDateTime::new(d, NaiveTime::default()),
            Err(_) => return Err(anyhow!("Bad date {:?}.", &self.date)),
        };

        let description = self.description.to_owned();

        let amount = match &self.amount.parse::<f64>() {
            Ok(a) => a.to_owned(),
            Err(_) => return Err(anyhow!("Bad amount {:?}", &self.amount)),
        };

        let l1_tag: AttrValue;
        let l2_tag: AttrValue;
        let l3_tag: AttrValue;
        if config
            .tags()
            .verify_tags(&self.l1_tag, &self.l2_tag, &self.l3_tag)
        {
            l1_tag = self.l1_tag.clone();
            l2_tag = self.l2_tag.clone();
            l3_tag = self.l3_tag.clone();
        } else {
            return Err(anyhow!(
                "Bad tags: {:?}, {:?}, {:?}.",
                &self.l1_tag,
                &self.l2_tag,
                &self.l3_tag
            ));
        }

        Ok(Transaction {
            id,
            account: account.to_string(),
            date,
            description: description.to_string(),
            amount,
            l1_tag: l1_tag.to_string(),
            l2_tag: l2_tag.to_string(),
            l3_tag: l3_tag.to_string(),
        })
    }

    pub fn from_transaction(transaction: &Transaction) -> Self {
        let id = transaction.id;
        let account = AttrValue::from(transaction.account.to_owned());
        let date = AttrValue::from(transaction.date.date().to_string());
        let description = AttrValue::from(transaction.description.to_owned());
        let amount = AttrValue::from(transaction.amount.to_string());
        let l1_tag = AttrValue::from(transaction.l1_tag.to_owned());
        let l2_tag = AttrValue::from(transaction.l2_tag.to_owned());
        let l3_tag = AttrValue::from(transaction.l3_tag.to_owned());

        Self {
            id,
            account,
            date,
            description,
            amount,
            l1_tag,
            l2_tag,
            l3_tag,
        }
    }
}
