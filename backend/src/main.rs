#![warn(clippy::all, clippy::nursery)]
use std::{env, fs::read_to_string, net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::sync::Mutex;

mod handlers;

use common::{Config, Transaction};

pub type TransactionsDb = Arc<Mutex<Vec<Transaction>>>;

pub type ConfigDb = Arc<Mutex<Config>>;

pub fn load_config() -> anyhow::Result<ConfigDb> {
    let config_file = read_to_string("config.json")?;
    Ok(Arc::new(Mutex::new(serde_json::from_str(&config_file)?)))
}

#[derive(Clone)]
pub struct AppState {
    pub config_db: ConfigDb,
    pub pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let db_url = env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let config_db = load_config()?;

    let state = Arc::new(AppState { config_db, pool });

    let app = Router::new()
        .route("/api/", get(root))
        .route(
            "/api/transactions",
            get(handlers::list_transactions).post(handlers::create_transaction),
        )
        .route("/api/config/:key", get(handlers::get_config))
        .route("/api/accounts", get(handlers::get_account_totals))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
