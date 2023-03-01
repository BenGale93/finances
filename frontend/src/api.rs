use common::{AccountSummary, ConfigOptions, Transaction};
use reqwasm::http::Request;

pub async fn get_config(key: &str) -> ConfigOptions {
    let query = format!("api/config/{key}", key = key);
    fetch_data(&query).await
}

pub async fn get_accounts() -> Vec<AccountSummary> {
    fetch_data("/api/accounts").await
}

pub async fn get_transactions() -> Vec<Transaction> {
    let transaction_endpoint = format!("/api/transactions?offset={x}&limit={y}", x = 0, y = 50);
    fetch_data(&transaction_endpoint).await
}

async fn fetch_data<T: for<'de> serde::de::Deserialize<'de>>(url: &str) -> T {
    Request::get(url)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}
