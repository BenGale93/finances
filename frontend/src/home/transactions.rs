use common::Transaction;
use reqwasm::http::Request;
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

pub struct TransactionsComponent {
    transactions: Option<Vec<Transaction>>,
}

fn get_transactions(transactions_cb: Callback<Vec<Transaction>>) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let transaction_endpoint = format!(
                "http://localhost:5000/transactions?offset={x}&limit={y}",
                x = 0,
                y = 50
            );
            let fetched_transactions = Request::get(&transaction_endpoint)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            transactions_cb.emit(fetched_transactions);
        }
    })
}

pub enum TransactionsMsg {
    Load(Vec<Transaction>),
}

impl Component for TransactionsComponent {
    type Message = TransactionsMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let transactions_cb = ctx.link().callback(TransactionsMsg::Load);
        get_transactions(transactions_cb);
        Self { transactions: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TransactionsMsg::Load(t) => {
                self.transactions = Some(t);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.transactions {
            Some(f) => f
                .iter()
                .map(|transaction| {
                    html! {
                    <TransactionComponent transaction={transaction.clone()}/>
                    }
                })
                .collect(),
            None => {
                html! {
                    <>
                        {"Loading transactions"}
                    </>
                }
            }
        }
    }
}
