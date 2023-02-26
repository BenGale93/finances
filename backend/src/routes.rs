use std::convert::Infallible;

use common::ListOptions;
use warp::{self, Filter};

use crate::{handlers, Transactions};

pub fn with_transactions(
    transactions: Transactions,
) -> impl Filter<Extract = (Transactions,), Error = Infallible> + Clone {
    warp::any().map(move || transactions.clone())
}

pub fn transaction_list(
    transactions: Transactions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("transactions")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_transactions(transactions))
        .and_then(handlers::list_transactions)
}

pub fn account_totals(
    transactions: Transactions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("accounts")
        .and(warp::get())
        .and(with_transactions(transactions))
        .and_then(handlers::get_account_totals)
}

pub fn transaction_routes(
    transactions: Transactions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    transaction_list(transactions.clone()).or(account_totals(transactions))
}
