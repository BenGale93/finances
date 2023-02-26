#![warn(clippy::all, clippy::nursery)]
use std::{env, sync::Arc};

use sqlx::Connection;
use tokio::sync::Mutex;
use warp::Filter;

mod handlers;
mod routes;

use common::Transaction;

pub type Transactions = Arc<Mutex<Vec<Transaction>>>;

pub async fn init_db(conn: &mut sqlx::SqliteConnection) -> anyhow::Result<Transactions> {
    let transactions = sqlx::query_as!(Transaction, "SELECT rowid as id, * from finances")
        .fetch_all(conn)
        .await?;
    Ok(Arc::new(Mutex::new(transactions)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = env::var("DATABASE_URL")?;

    let mut conn = sqlx::SqliteConnection::connect(&db_url).await?;
    let transactions = init_db(&mut conn).await?;

    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let routes = root
        .or(routes::transaction_routes(transactions.clone()))
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
    Ok(())
}
