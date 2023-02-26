use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    Json,
};
use common::{AccountSummary, Config, ListOptions, Transaction};

use crate::AppState;

pub async fn list_transactions(
    Query(opts): Query<ListOptions>,
    State(app_state): State<Arc<AppState>>,
) -> Json<Vec<Transaction>> {
    let transactions = app_state.transactions_db.lock().await;
    let transactions: Vec<Transaction> = transactions
        .clone()
        .into_iter()
        .rev()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(usize::MAX))
        .collect();
    Json(transactions)
}

pub async fn get_account_totals(
    State(app_state): State<Arc<AppState>>,
) -> Json<Vec<AccountSummary>> {
    let transactions = app_state.transactions_db.lock().await;
    let transactions: Vec<Transaction> = transactions.clone();

    let mut accounts: HashMap<String, f64> = HashMap::new();
    for transaction in &transactions {
        *accounts.entry(transaction.account.clone()).or_insert(0.0) += transaction.amount;
    }
    let accounts: HashMap<_, _> = accounts.iter().filter(|&(_, v)| v.abs() > 0.001).collect();
    let mut accounts: Vec<AccountSummary> = accounts
        .into_iter()
        .map(|(k, v)| AccountSummary {
            name: k.to_owned(),
            amount: v.to_owned(),
        })
        .collect();
    accounts.sort();
    Json(accounts)
}

#[axum::debug_handler]
pub async fn get_config(State(app_state): State<Arc<AppState>>) -> Json<Config> {
    let config = app_state.config_db.lock().await;
    let config: Config = config.clone();

    Json(config)
}
