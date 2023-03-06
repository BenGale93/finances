use std::sync::Arc;

use common::{Config, Transaction};
use yew::prelude::*;

use super::{fields, UserTransaction};
use crate::api;

pub enum UpdateFormMsg {
    Error,
    Submit,
    Delete,
    Deleted,
    Success(UserTransaction),
    UpdateAccount(String),
    UpdateDate(String),
    UpdateDescription(String),
    UpdateAmount(String),
    UpdateTags((String, String, String)),
}

#[derive(Clone, PartialEq, Properties)]
pub struct UpdateFormProps {
    pub given_transaction: UserTransaction,
    pub on_submit: Callback<()>,
    pub config: Arc<Config>,
}

pub struct TransactionComponent {
    transaction: UserTransaction,
}

impl Component for TransactionComponent {
    type Message = UpdateFormMsg;
    type Properties = UpdateFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            transaction: ctx.props().given_transaction.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UpdateFormMsg::Error => (),
            UpdateFormMsg::Success(t) => {
                ctx.props().on_submit.emit(());
                self.transaction = t;
            }
            UpdateFormMsg::Submit => {
                log::info!("Handling submit");
                let transaction = match self.transaction.to_transaction(&ctx.props().config) {
                    Ok(t) => t,
                    Err(e) => {
                        log::info!("Failed conversion: {e}");
                        ctx.link().send_message(UpdateFormMsg::Error);
                        return false;
                    }
                };
                log::info!("Making API patch with {:?}.", transaction);
                let submitted_transaction = self.transaction.clone();
                ctx.link().send_future(async move {
                    api::update_transaction(transaction).await;
                    UpdateFormMsg::Success(submitted_transaction)
                });
                self.transaction = UserTransaction::default();
            }
            UpdateFormMsg::Deleted => {
                ctx.props().on_submit.emit(());
            }
            UpdateFormMsg::Delete => {
                log::info!("Making API delete with {:?}.", self.transaction.id);
                let id = self.transaction.id;
                ctx.link().send_future(async move {
                    api::delete_transaction(id).await;
                    UpdateFormMsg::Deleted
                });
            }
            UpdateFormMsg::UpdateAccount(account) => {
                log::info!("Account: {}", account);
                self.transaction.account = account;
            }
            UpdateFormMsg::UpdateDate(date) => {
                log::info!("Date: {}", date);
                self.transaction.date = date;
            }
            UpdateFormMsg::UpdateDescription(description) => {
                self.transaction.description = description;
            }
            UpdateFormMsg::UpdateAmount(amount) => {
                self.transaction.amount = amount;
            }
            UpdateFormMsg::UpdateTags(tags) => {
                log::info!("Updating tags: {:?}", tags);
                self.transaction.l1_tag = tags.0;
                self.transaction.l2_tag = tags.1;
                self.transaction.l3_tag = tags.2;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().given_transaction.id != old_props.given_transaction.id {
            self.transaction = ctx.props().given_transaction.clone();
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = self.transaction.id.to_string();
        let given_account = self.transaction.account.clone();
        let given_date = self.transaction.date.clone();
        let given_description = self.transaction.description.clone();
        let given_amount = self.transaction.amount.clone();
        let given_tags = (
            self.transaction.l1_tag.clone(),
            self.transaction.l2_tag.clone(),
            self.transaction.l3_tag.clone(),
        );

        yew::html! {
                <tr>
                    <td>
                    <form id={id.clone()}></form>
                    </td>
                    <td>
                    <fields::AccountPicker id={id.clone()} account_list={ctx.props().config.account_list().to_owned()}
                    {given_account}
                    on_input={ctx.link().callback(UpdateFormMsg::UpdateAccount)}/>
                    </td>
                    <td>
                    <fields::DatePicker id={id.clone()} {given_date}
                    on_input={ctx.link().callback(UpdateFormMsg::UpdateDate)}/>
                    </td>
                    <td>
                    <fields::DescriptionField id={id.clone()} {given_description}
                    on_input={ctx.link().callback(UpdateFormMsg::UpdateDescription)}/>
                    </td>
                    <td>
                    <fields::AmountField id={id.clone()} {given_amount}
                    on_input={ctx.link().callback(UpdateFormMsg::UpdateAmount)}/>
                    </td>
                    <fields::TagPicker id={id.clone()} tags={ctx.props().config.tags().clone()} {given_tags}
                    on_input={ctx.link().callback(UpdateFormMsg::UpdateTags)}/>
                    <td>
                    <button onclick={ctx.link().callback(|_| UpdateFormMsg::Submit)}>{"💾"}</button>
                    <button onclick={ctx.link().callback(|_| UpdateFormMsg::Delete)}>{"❌"}</button>
                    </td>
                </tr>

        }
    }
}

#[derive(PartialEq, Properties)]
pub struct TransactionsComponentProps {
    pub transactions: Arc<Vec<Transaction>>,
    pub on_submit: Callback<()>,
    pub config: Arc<Config>,
}

pub enum TransactionsMsg {
    RefreshData,
}

pub struct TransactionsComponent {}

impl Component for TransactionsComponent {
    type Message = TransactionsMsg;
    type Properties = TransactionsComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TransactionsMsg::RefreshData => ctx.props().on_submit.emit(()),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let transaction_html: Html = ctx.props().transactions.iter()
        .map(|transaction| {
            html! {
            <TransactionComponent given_transaction={UserTransaction::from_transaction(transaction)}
            on_submit={ctx.link().callback(|_| TransactionsMsg::RefreshData)}
            config={ctx.props().config.clone()}
            />
            }
        })
        .collect();

        html! {
            <table>
            <tr>
                <th>{""}</th>
                <th>{"Account"}</th>
                <th>{"Date"}</th>
                <th>{"Description"}</th>
                <th>{"Amount"}</th>
                <th>{"L1 Tag"}</th>
                <th>{"L2 Tag"}</th>
                <th>{"L3 Tag"}</th>
                <th>{""}</th>
            </tr>
            {transaction_html}
            </table>
        }
    }
}
