use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use common::{AccountSummary, ConfigOptions, ListOptions, Transaction};

use crate::{AppState, Config};

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

pub async fn get_config(
    Path(key): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<ConfigOptions>, StatusCode> {
    let config = app_state.config_db.lock().await;
    let config: Config = config.clone();
    let option = match key.as_str() {
        "budget" => ConfigOptions::Budget(config.budget()),
        "account_list" => ConfigOptions::AccountList(config.account_list().to_owned()),
        "period_items" => ConfigOptions::PeriodItems(config.period_items().to_owned()),
        "budget_items" => ConfigOptions::BudgetItems(config.period_items().to_owned()),
        "tags" => ConfigOptions::Tags(config.tags().to_owned()),
        _ => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Json(option))
}
