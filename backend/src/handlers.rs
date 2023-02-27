use std::sync::Arc;

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
    let pool = app_state.pool.clone();
    let limit = opts.limit.unwrap_or(50) as i64;
    let offset = opts.offset.unwrap_or(0) as i64;
    let transactions = sqlx::query_as!(
        Transaction,
        r#"SELECT rowid as "id!", account as "account!", date as "date!", description as "description!",
        amount as "amount!", l1_tag as "l1_tag!", l2_tag as "l2_tag!", l3_tag as "l3_tag!"
        FROM finances ORDER BY date DESC LIMIT ? OFFSET ?"#,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(transactions)
}

pub async fn get_account_totals(
    State(app_state): State<Arc<AppState>>,
) -> Json<Vec<AccountSummary>> {
    let pool = app_state.pool.clone();
    let accounts = sqlx::query_as!(
        AccountSummary,
        r#"WITH grouped AS
        (SELECT account as name, SUM(amount) as amount FROM finances GROUP BY account ORDER BY name)
        SELECT name, amount as "amount!" FROM grouped WHERE amount > 0.001"#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(accounts)
}

pub async fn get_config(State(app_state): State<Arc<AppState>>) -> Json<Config> {
    let config = app_state.config_db.lock().await;
    let config: Config = config.clone();

    Json(config)
}
