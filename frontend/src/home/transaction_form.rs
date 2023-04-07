use std::sync::Arc;

use common::Config;
use yew::prelude::*;

use super::{fields, UserTransaction};
use crate::api;

pub enum CreateFormMsg {
    Error,
    Submit,
    Success(UserTransaction),
    UpdateAccount(AttrValue),
    UpdateDate(AttrValue),
    UpdateDescription(AttrValue),
    UpdateAmount(AttrValue),
    UpdateTags((AttrValue, AttrValue, AttrValue)),
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateFormProps {
    pub on_submit: Callback<()>,
    pub config: Arc<Config>,
}

pub struct CreateForm {
    transaction: UserTransaction,
}

impl Component for CreateForm {
    type Message = CreateFormMsg;
    type Properties = CreateFormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("Creating form");
        Self {
            transaction: UserTransaction::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateFormMsg::Error => (),
            CreateFormMsg::Success(t) => {
                log::info!("Submitted new transaction");
                ctx.props().on_submit.emit(());
                self.transaction = t;
            }
            CreateFormMsg::Submit => {
                log::info!("Handling submit");
                let transaction = match self.transaction.to_transaction(&ctx.props().config) {
                    Ok(t) => t,
                    Err(e) => {
                        log::info!("Failed conversion: {e}");
                        ctx.link().send_message(CreateFormMsg::Error);
                        return false;
                    }
                };
                log::info!("Making API post with {:?}.", transaction);
                let submitted_transaction = self.transaction.clone();
                ctx.link().send_future(async move {
                    api::create_transaction(transaction).await;
                    CreateFormMsg::Success(submitted_transaction)
                });
                self.transaction = UserTransaction::default();
            }
            CreateFormMsg::UpdateAccount(account) => {
                log::info!("Account: {}", account);
                self.transaction.account = account;
            }
            CreateFormMsg::UpdateDate(date) => {
                log::info!("Date: {}", date);
                self.transaction.date = date;
            }
            CreateFormMsg::UpdateDescription(description) => {
                self.transaction.description = description;
            }
            CreateFormMsg::UpdateAmount(amount) => {
                self.transaction.amount = amount;
            }
            CreateFormMsg::UpdateTags(tags) => {
                log::info!("Updating tags: {:?}", tags);
                self.transaction.l1_tag = tags.0;
                self.transaction.l2_tag = tags.1;
                self.transaction.l3_tag = tags.2;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = "create".to_string();
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
            <>
                <table>
                <tr>
                    <th></th>
                    <th><label for="account">{ "Account" }</label></th>
                    <th><label for="date">{ "Date" }</label></th>
                    <th><label for="description">{ "Description" }</label></th>
                    <th><label for="amount">{ "Amount" }</label></th>
                    <th><label for="l1_tag">{ "L1 Tag" }</label></th>
                    <th><label for="l2_tag">{ "L2 Tag" }</label></th>
                    <th><label for="l3_tag">{ "L3 Tag" }</label></th>
                </tr>
                <tr>
                    <td>
                    <form id={id.clone()}></form>
                    </td>
                    <td>
                    <fields::AccountPicker id={id.clone()} account_list={ctx.props().config.account_list().to_owned()}
                    {given_account}
                    on_input={ctx.link().callback(CreateFormMsg::UpdateAccount)}/>
                    </td>
                    <td>
                    <fields::DatePicker id={id.clone()} {given_date}
                    on_input={ctx.link().callback(CreateFormMsg::UpdateDate)}/>
                    </td>
                    <td>
                    <fields::DescriptionField id={id.clone()} {given_description}
                    on_input={ctx.link().callback(CreateFormMsg::UpdateDescription)}/>
                    </td>
                    <td>
                    <fields::AmountField id={id.clone()} {given_amount}
                    on_input={ctx.link().callback(CreateFormMsg::UpdateAmount)}/>
                    </td>
                    <fields::TagPicker id={id.clone()} tags={ctx.props().config.tags().clone()} {given_tags}
                    on_input={ctx.link().callback(CreateFormMsg::UpdateTags)}/>
                </tr>

                </table>
                <button
                    onclick={ ctx.link().callback(|_| CreateFormMsg::Submit) }
                >
                    { "Create" }
                </button>
            </>
        }
    }
}
