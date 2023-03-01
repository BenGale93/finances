use std::sync::Arc;

use common::Transaction;
use yew::prelude::*;

use crate::api;

#[derive(PartialEq, Properties)]
struct TransactionComponentProps {
    pub transaction: Transaction,
}

#[function_component(TransactionComponent)]
fn transaction_component(
    TransactionComponentProps { transaction }: &TransactionComponentProps,
) -> Html {
    html! {
        <tr>
            <td class="account">{transaction.account.to_owned()}</td>
            <td class="date">{transaction.date.to_owned()}</td>
            <td class="description">{transaction.description.to_owned()}</td>
            <td class="amount">{transaction.amount.to_owned()}</td>
            <td class="l1_tag">{transaction.l1_tag.to_owned()}</td>
            <td class="l2_tag">{transaction.l2_tag.to_owned()}</td>
            <td class="l3_tag">{transaction.l3_tag.to_owned()}</td>
        </tr>
    }
}

pub enum TransactionsMsg {
    NeedUpdate,
    Back,
    Forward,
    Update(Vec<Transaction>),
}

pub struct TransactionsComponent {
    transactions: Option<Arc<Vec<Transaction>>>,
    page: (usize, usize),
}

impl Component for TransactionsComponent {
    type Message = TransactionsMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            transactions: None,
            page: (0, 50),
        };

        ctx.link().send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;
        match msg {
            TransactionsMsg::NeedUpdate => {
                let (offset, limit) = self.page;
                ctx.link().send_future(async move {
                    TransactionsMsg::Update(api::get_transactions(offset, limit).await)
                });
            }
            TransactionsMsg::Update(transactions) => {
                if transactions.is_empty() {
                    /* Gone too far, let's go back */
                    ctx.link().send_message(TransactionsMsg::Forward);
                    should_render = false;
                } else {
                    self.transactions = Some(Arc::new(transactions));
                    should_render = true;
                }
            }
            TransactionsMsg::Back => {
                let transactions = match &self.transactions {
                    Some(t) => t,
                    None => return false,
                };
                if transactions.iter().len() == self.page.1 {
                    self.page.0 += self.page.1;
                    ctx.link().send_message(TransactionsMsg::NeedUpdate);
                } else {
                    should_render = true;
                }
            }
            TransactionsMsg::Forward => {
                self.page.0 = self.page.0.saturating_sub(self.page.1);
                ctx.link().send_message(TransactionsMsg::NeedUpdate);
            }
        }
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let transactions = match &self.transactions {
            Some(transactions) => transactions,
            None => return "".into(),
        };
        let transaction_html: Html = transactions
            .iter()
            .map(|transaction| {
                html! {
                <TransactionComponent transaction={transaction.clone()}/>
                }
            })
            .collect();

        html! {
            <div>
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
            {transaction_html}
            </table>
                <button onclick={ctx.link().callback(|_| TransactionsMsg::Back)}>{"back"}</button>
                <button onclick={ctx.link().callback(|_| TransactionsMsg::Forward)}>{"forward"}</button>
            </div>
        }
    }
}
