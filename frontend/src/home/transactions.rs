use std::sync::Arc;

use common::Transaction;
use yew::prelude::*;

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

#[derive(PartialEq, Properties)]
pub struct TransactionsComponentProps {
    pub transactions: Arc<Vec<Transaction>>,
}

#[function_component(TransactionsComponent)]
pub fn transactions_component(
    TransactionsComponentProps { transactions }: &TransactionsComponentProps,
) -> Html {
    let transaction_html: Html = transactions
        .iter()
        .map(|transaction| {
            html! {
            <TransactionComponent transaction={transaction.clone()}/>
            }
        })
        .collect();

    html! {
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
    }
}
