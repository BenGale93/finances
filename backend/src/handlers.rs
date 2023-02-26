use std::{collections::HashMap, convert::Infallible};

use common::{AccountSummary, Config, ListOptions, Transaction};

use crate::{ConfigDb, TransactionsDb};

pub async fn list_transactions(
    opts: ListOptions,
    transactions_db: TransactionsDb,
) -> Result<impl warp::Reply, Infallible> {
    let transactions = transactions_db.lock().await;
    let transactions: Vec<Transaction> = transactions
        .clone()
        .into_iter()
        .rev()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(usize::MAX))
        .collect();
    Ok(warp::reply::json(&transactions))
}

pub async fn get_account_totals(
    transactions_db: TransactionsDb,
) -> Result<impl warp::Reply, Infallible> {
    let transactions = transactions_db.lock().await;
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
    Ok(warp::reply::json(&accounts))
}

pub async fn get_config(config_db: ConfigDb) -> Result<impl warp::Reply, Infallible> {
    let config = config_db.lock().await;
    let config: Config = config.clone();

    Ok(warp::reply::json(&config))
}
