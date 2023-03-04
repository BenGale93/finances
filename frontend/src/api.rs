use common::{AccountSummary, ConfigOptions, Transaction};
use reqwasm::http::Request;

pub async fn get_config(key: &str) -> ConfigOptions {
    let query = format!("/api/config/{key}", key = key);
    fetch_data(&query).await
}

pub async fn get_accounts() -> Vec<AccountSummary> {
    fetch_data("/api/accounts").await
}

pub async fn get_transactions(offset: usize, limit: usize) -> Vec<Transaction> {
    let transaction_endpoint = format!("/api/transactions?offset={offset}&limit={limit}");
    fetch_data(&transaction_endpoint).await
}

pub async fn create_transaction(transaction: Transaction) {
    Request::post("/api/transactions")
        .body(serde_json::to_string(&transaction).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();
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
