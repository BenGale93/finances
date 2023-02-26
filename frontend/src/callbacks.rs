use common::{AccountSummary, Config, Transaction};
use reqwasm::http::Request;
use yew::Callback;

pub fn get_config(config_cb: Callback<Config>) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let fetched_config = Request::get("http://localhost:5000/config")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            config_cb.emit(fetched_config);
        }
    })
}

pub fn get_accounts(accounts_cb: Callback<Vec<AccountSummary>>) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let fetched_accounts = Request::get("http://localhost:5000/accounts")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            accounts_cb.emit(fetched_accounts);
        }
    })
}

pub fn get_transactions(transactions_cb: Callback<Vec<Transaction>>) {
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
