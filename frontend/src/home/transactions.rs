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
    Update(Vec<Transaction>),
}

pub struct TransactionsComponent {
    transactions: Option<Arc<Vec<Transaction>>>,
}

impl Component for TransactionsComponent {
    type Message = TransactionsMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self { transactions: None };

        ctx.link().send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;
        match msg {
            TransactionsMsg::NeedUpdate => {
                ctx.link().send_future(async move {
                    TransactionsMsg::Update(api::get_transactions().await)
                });
            }
            TransactionsMsg::Update(transactions) => {
                self.transactions = Some(Arc::new(transactions));
                should_render = true;
            }
        }
        should_render
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let transactions = match &self.transactions {
            Some(transactions) => transactions,
            None => return "".into(),
        };
        transactions
            .iter()
            .map(|transaction| {
                html! {
                <TransactionComponent transaction={transaction.clone()}/>
                }
            })
            .collect()
    }
}
