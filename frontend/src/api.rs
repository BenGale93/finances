use common::{
    AccountSummary, BalanceByTime, BudgetProgress, BudgetProgressOptions, CategorySpend,
    CategorySpendOptions, ConfigOptions, DateGrouping, Transaction,
};
use reqwasm::http::Request;

pub async fn get_config(key: &str) -> ConfigOptions {
    let query = format!("/api/config/{key}", key = key);
    fetch_data(&query).await
}

pub async fn get_accounts() -> Vec<AccountSummary> {
    fetch_data("/api/accounts").await
}

pub async fn balance_by_date(grouping: DateGrouping) -> Vec<BalanceByTime> {
    fetch_data(&format!("/api/balance?{}", grouping.url_encode())).await
}

pub async fn get_transactions(offset: usize, limit: usize) -> Vec<Transaction> {
    let transaction_endpoint = format!("/api/transactions?offset={offset}&limit={limit}");
    fetch_data(&transaction_endpoint).await
}

pub async fn budget_progress(options: &BudgetProgressOptions) -> BudgetProgress {
    fetch_data(&format!("/api/budget?{}", options.url_encode())).await
}

pub async fn category_spend(options: &CategorySpendOptions) -> Vec<CategorySpend> {
    fetch_data(&format!("/api/category?{}", options.url_encode())).await
}

pub async fn create_transaction(transaction: Transaction) {
    Request::post("/api/transactions")
        .body(serde_json::to_string(&transaction).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();
}

pub async fn update_transaction(transaction: Transaction) {
    Request::patch("/api/transactions")
        .body(serde_json::to_string(&transaction).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();
}

pub async fn delete_transaction(id: i64) {
    Request::delete("/api/transactions")
        .body(serde_json::to_string(&id).unwrap())
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
