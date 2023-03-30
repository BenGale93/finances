use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::{
    AccountSummary, BalanceByTime, BalanceTimeOptions, BudgetProgress, BudgetProgressOptions,
    CategorySpend, CategorySpendOptions, Config, ConfigOptions, DateGrouping, ListOptions,
    Transaction,
};
use sqlx::{QueryBuilder, Row, Sqlite};

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

pub async fn create_transaction(
    State(app_state): State<Arc<AppState>>,
    Json(transaction): Json<Transaction>,
) -> impl IntoResponse {
    let mut conn = app_state.pool.acquire().await.unwrap();
    let Transaction {
        id: _,
        account,
        date,
        description,
        amount,
        l1_tag,
        l2_tag,
        l3_tag,
    } = transaction;

    let id = sqlx::query!(
        r#"
        INSERT INTO finances ( account, date, description, amount, l1_tag, l2_tag, l3_tag)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        account,
        date,
        description,
        amount,
        l1_tag,
        l2_tag,
        l3_tag
    )
    .execute(&mut conn)
    .await
    .unwrap()
    .last_insert_rowid();

    (StatusCode::CREATED, Json(id))
}

#[axum::debug_handler]
pub async fn update_transaction(
    State(app_state): State<Arc<AppState>>,
    Json(patch_transaction): Json<Transaction>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = app_state.pool.acquire().await.unwrap();

    let Transaction {
        id,
        account,
        date,
        description,
        amount,
        l1_tag,
        l2_tag,
        l3_tag,
    } = patch_transaction;

    let result = sqlx::query!(
        r#"
        UPDATE finances
        SET account = ?1, date = ?2, description = ?3, amount = ?4,
        l1_tag = ?5, l2_tag = ?6, l3_tag = ?7
        WHERE rowid = ?8
        "#,
        account,
        date,
        description,
        amount,
        l1_tag,
        l2_tag,
        l3_tag,
        id
    )
    .execute(&mut conn)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn get_account_totals(
    State(app_state): State<Arc<AppState>>,
) -> Json<Vec<AccountSummary>> {
    let pool = app_state.pool.clone();
    let accounts = sqlx::query_as!(
        AccountSummary,
        r#"WITH grouped AS
        (SELECT account as name, SUM(amount) as amount FROM finances GROUP BY account ORDER BY name)
        SELECT name, amount as "amount!" FROM grouped WHERE abs(amount) > 0.001"#
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
        "all" => ConfigOptions::All(config),
        "budget" => ConfigOptions::Budget(config.budget()),
        "account_list" => ConfigOptions::AccountList(config.account_list().to_owned()),
        "period_items" => ConfigOptions::PeriodItems(config.period_items().to_owned()),
        "budget_items" => ConfigOptions::BudgetItems(config.budget_items().to_owned()),
        "tags" => ConfigOptions::Tags(config.tags().to_owned()),
        _ => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Json(option))
}

pub async fn delete_transaction(
    State(app_state): State<Arc<AppState>>,
    Json(id): Json<i64>,
) -> StatusCode {
    let mut conn = app_state.pool.acquire().await.unwrap();
    let result = sqlx::query!(
        r#"
        DELETE FROM finances WHERE rowid = ?1
        "#,
        id
    )
    .execute(&mut conn)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

pub async fn balance_by_date(
    Query(opts): Query<BalanceTimeOptions>,
    State(app_state): State<Arc<AppState>>,
) -> Json<Vec<BalanceByTime>> {
    let pool = app_state.pool.clone();
    let balance = match opts.grouping.unwrap_or(DateGrouping::Day) {
        DateGrouping::Day => sqlx::query_as!(
            BalanceByTime,
            r#"SELECT STRFTIME("%Y-%m-%d", date) as "date!",
            SUM(CASE WHEN amount >= 0 THEN amount END) as "incoming!",
            SUM(CASE WHEN amount < 0 THEN amount END) as "outgoing!",
            SUM(amount) as "balance!"
        FROM finances WHERE l1_tag not in ("Transfers", "Balance", "Repayments")
        GROUP BY STRFTIME("%Y-%m-%d", date)
        "#
        )
        .fetch_all(&pool)
        .await
        .unwrap(),
        DateGrouping::Month => sqlx::query_as!(
            BalanceByTime,
            r#"SELECT STRFTIME("%Y-%m", date) as "date!",
            SUM(CASE WHEN amount >= 0 THEN amount END) as "incoming!",
            SUM(CASE WHEN amount < 0 THEN amount END) as "outgoing!",
            SUM(amount) as "balance!"
        FROM finances WHERE l1_tag not in ("Transfers", "Balance", "Repayments")
        GROUP BY STRFTIME("%Y-%m", date)
        "#
        )
        .fetch_all(&pool)
        .await
        .unwrap(),
    };
    Json(balance)
}

pub async fn budget_progress(
    Query(opts): Query<BudgetProgressOptions>,
    State(app_state): State<Arc<AppState>>,
) -> Json<BudgetProgress> {
    let pool = app_state.pool.clone();
    let config = app_state.config_db.lock().await;
    let budget = config.budget();
    let budget_items = config.budget_items();

    let date = opts.date.format("%Y-%m").to_string();

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
        SELECT ABS(SUM(amount)) as spend
        FROM finances WHERE l1_tag in ("#,
    );
    let mut separated = query_builder.separated(", ");
    for tag in budget_items.iter() {
        separated.push_bind(tag);
    }
    separated.push_unseparated(r#") AND STRFTIME("%Y-%m", date) = "#);
    separated.push_unseparated(format!("\"{date}\""));

    let query = query_builder.build();
    let row = query.fetch_one(&pool).await.unwrap();

    Json(BudgetProgress {
        budget,
        spend: row.try_get("spend").unwrap(),
    })
}

pub async fn category_spend(
    Query(opts): Query<CategorySpendOptions>,
    State(app_state): State<Arc<AppState>>,
) -> Json<Vec<CategorySpend>> {
    let pool = app_state.pool.clone();

    let date = opts.date.format("%Y-%m").to_string();

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
        SELECT l1_tag, ABS(SUM(amount)) as spend
        FROM finances WHERE l1_tag in ("#,
    );
    let mut separated = query_builder.separated(", ");
    for tag in opts.l1_tags.iter() {
        separated.push_bind(tag);
    }
    separated.push_unseparated(r#") AND STRFTIME("%Y-%m", date) = "#);
    separated.push_unseparated(format!("\"{date}\" GROUP BY l1_tag"));

    let query = query_builder.build();
    let rows = query
        .map(|row: sqlx::sqlite::SqliteRow| CategorySpend {
            name: row.try_get("l1_tag").unwrap(),
            amount: row.try_get("spend").unwrap(),
        })
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(rows)
}
