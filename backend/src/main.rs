#![warn(clippy::all, clippy::nursery)]
use std::{env, fs::read_to_string, sync::Arc};

use sqlx::Connection;
use tokio::sync::Mutex;
use warp::Filter;

mod handlers;
mod routes;

use common::{Config, Transaction};

pub type TransactionsDb = Arc<Mutex<Vec<Transaction>>>;

pub type ConfigDb = Arc<Mutex<Config>>;

pub async fn init_db(conn: &mut sqlx::SqliteConnection) -> anyhow::Result<TransactionsDb> {
    let transactions = sqlx::query_as!(Transaction, "SELECT rowid as id, * from finances")
        .fetch_all(conn)
        .await?;
    Ok(Arc::new(Mutex::new(transactions)))
}

pub fn load_config() -> anyhow::Result<ConfigDb> {
    let config_file = read_to_string("config.json")?;
    Ok(Arc::new(Mutex::new(serde_json::from_str(&config_file)?)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = env::var("DATABASE_URL")?;

    let mut conn = sqlx::SqliteConnection::connect(&db_url).await?;
    let transactions = init_db(&mut conn).await?;
    let config_db = load_config()?;

    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let routes = root
        .or(routes::transaction_routes(transactions.clone()))
        .or(routes::config_routes(config_db.clone()))
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
    Ok(())
}
