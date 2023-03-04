use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use common::{Config, ConfigOptions, Transaction};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api;

pub enum FormMsg {
    Error,
    Submit,
    Success,
    NeedUpdateConfig,
    UpdateConfig(Config),
    UpdateAccount(String),
    UpdateDate(String),
    UpdateDescription(String),
    UpdateAmount(String),
    UpdateL1Tag(String),
    UpdateL2Tag(String),
    UpdateL3Tag(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct UserTransaction {
    pub account: String,
    pub date: String,
    pub description: String,
    pub amount: String,
    pub l1_tag: String,
    pub l2_tag: String,
    pub l3_tag: String,
}

impl UserTransaction {
    pub fn to_transaction(&self, config: &Config) -> anyhow::Result<Transaction> {
        let id = 0;
        let account = if config.account_list().contains(&self.account) {
            self.account.to_owned()
        } else {
            return Err(anyhow!("Bad account."));
        };

        let date = match NaiveDate::parse_from_str(&self.date, "%Y-%m-%d") {
            Ok(d) => NaiveDateTime::new(d, NaiveTime::default()),
            Err(_) => return Err(anyhow!("Bad date.")),
        };

        let description = self.description.to_owned();

        let amount = match &self.amount.parse::<f64>() {
            Ok(a) => a.to_owned(),
            Err(_) => return Err(anyhow!("Bad amount")),
        };

        let l1_tag: String;
        let l2_tag: String;
        let l3_tag: String;
        if config
            .tags()
            .verify_tags(&self.l1_tag, &self.l2_tag, &self.l3_tag)
        {
            l1_tag = self.l1_tag.to_owned();
            l2_tag = self.l2_tag.to_owned();
            l3_tag = self.l3_tag.to_owned();
        } else {
            return Err(anyhow!("Bad tags."));
        }

        Ok(Transaction {
            id,
            account,
            date,
            description,
            amount,
            l1_tag,
            l2_tag,
            l3_tag,
        })
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormProperties {
    pub on_submit: Callback<()>,
}

pub struct TransactionForm {
    config: Option<Config>,
    transaction: UserTransaction,
}

impl Component for TransactionForm {
    type Message = FormMsg;
    type Properties = FormProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            config: None,
            transaction: UserTransaction::default(),
        };

        ctx.link().send_message(Self::Message::NeedUpdateConfig);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMsg::Error => (),
            FormMsg::Success => (),
            FormMsg::Submit => {
                log::info!("Handling submit");
                let Some(config) = &self.config else {
                    ctx.link().send_message(FormMsg::NeedUpdateConfig);
                    return false;
                };
                let transaction = match self.transaction.to_transaction(config) {
                    Ok(t) => t,
                    Err(e) => {
                        log::info!("Failed conversion: {e}");
                        ctx.link().send_message(FormMsg::Error);
                        return false;
                    }
                };
                log::info!("Making API post.");
                ctx.props().on_submit.emit(());
                ctx.link().send_future(async move {
                    api::create_transaction(transaction).await;
                    FormMsg::Success
                });
                self.transaction = UserTransaction::default();
                return true;
            }
            FormMsg::NeedUpdateConfig => {
                ctx.link().send_future(async move {
                    let config = api::get_config("all").await;
                    match config {
                        ConfigOptions::All(c) => FormMsg::UpdateConfig(c),
                        _ => FormMsg::Error,
                    }
                });
            }
            FormMsg::UpdateConfig(c) => {
                self.config = Some(c);
                return true;
            }
            FormMsg::UpdateAccount(account) => {
                self.transaction.account = account;
            }
            FormMsg::UpdateDate(date) => {
                self.transaction.date = date;
            }
            FormMsg::UpdateDescription(description) => {
                self.transaction.description = description;
            }
            FormMsg::UpdateAmount(amount) => {
                self.transaction.amount = amount;
            }
            FormMsg::UpdateL1Tag(tag) => {
                self.transaction.l1_tag = tag;
            }
            FormMsg::UpdateL2Tag(tag) => {
                self.transaction.l2_tag = tag;
            }
            FormMsg::UpdateL3Tag(tag) => {
                self.transaction.l3_tag = tag;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        yew::html! {
            <form>
                <table>
                <tr>
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
                        <input
                            class="form-control"
                            name="account"
                            required=true
                            value={ self.transaction.account.clone() }
                            oninput={ ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<HtmlInputElement>();
                                FormMsg::UpdateAccount(input.value())
                            }) }
                        />
                    </td>
                    <td>
                        <input
                            class="form-control"
                            name="date"
                            required=true
                            value={ self.transaction.date.clone() }
                            oninput={ ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<HtmlInputElement>();
                                FormMsg::UpdateDate(input.value())
                            }) }
                        />
                    </td>
                    <td>
                        <input
                            class="form-control"
                            name="description"
                            required=true
                            value={ self.transaction.description.clone() }
                            oninput={ ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<HtmlInputElement>();
                                FormMsg::UpdateDescription(input.value())
                            }) }
                        />
                    </td>
                    <td>
                        <input
                            class="form-control"
                            name="amount"
                            required=true
                            value={ self.transaction.amount.clone() }
                            oninput={ ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<HtmlInputElement>();
                                FormMsg::UpdateAmount(input.value())
                            }) }
                        />
                    </td>
                    <td>
                        <input
                            class="form-control"
                            name="l1_tag"
                            required=true
                            value={ self.transaction.l1_tag.clone() }
                            oninput={ ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<HtmlInputElement>();
                                FormMsg::UpdateL1Tag(input.value())
                            }) }
                        />
                    </td>
                    <td>
                        <input
                            class="form-control"
                            name="l2_tag"
                            required=true
                            value={ self.transaction.l2_tag.clone() }
                            oninput={ ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<HtmlInputElement>();
                                FormMsg::UpdateL2Tag(input.value())
                            }) }
                        />
                    </td>
                    <td>
                        <input
                            class="form-control"
                            name="l3_tag"
                            required=true
                            value={ self.transaction.l3_tag.clone() }
                            oninput={ ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<HtmlInputElement>();
                                FormMsg::UpdateL3Tag(input.value())
                            }) }
                        />
                    </td>
                </tr>

                </table>
                <button
                    onclick={ ctx.link().callback(|_| FormMsg::Submit) }
                >
                    { "Create" }
                </button>
            </form>
        }
    }
}
