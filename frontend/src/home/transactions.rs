use std::sync::Arc;

use common::Transaction;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct TransactionComponentProps {
    pub transaction: Transaction,
}

#[function_component(TransactionComponent)]
fn transaction_component(props: &TransactionComponentProps) -> Html {
    let TransactionComponentProps { transaction } = props;
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

#[derive(PartialEq, Properties)]
pub struct TransactionsComponentProps {
    pub transactions: Arc<Vec<Transaction>>,
}
pub struct TransactionsComponent {}

impl Component for TransactionsComponent {
    type Message = ();
    type Properties = TransactionsComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.props()
            .transactions
            .iter()
            .map(|transaction| {
                html! {
                <TransactionComponent transaction={transaction.clone()}/>
                }
            })
            .collect()
    }
}
