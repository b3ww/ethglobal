use std::str::FromStr;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod checkpoint;
mod github;
mod pollers;

use alloy::primitives::Address;
use github::bot::Bot;
use sqlx::PgPool;

#[derive(Debug, Clone)]
struct Config {
    database_url: String,
    github_token: String,
    contract_address: Address,
}

impl Config {
    fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .context("Missing DATABASE_URL environment variable")?;
        let github_token = std::env::var("GITHUB_BOT_TOKEN")
            .context("Missing GITHUB_BOT_TOKEN env var")?;
        let contract_address_str =
            std::env::var("CONTRACT_ADDRESS").context("Missing CONTRACT_ADDRESS env var")?;
        let contract_address = Address::from_str(&contract_address_str)
            .context("Failed to parse CONTRACT_ADDRESS")?;

        Ok(Self {
            database_url,
            github_token,
            contract_address,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global logger");

    info!("Starting application...");
    dotenv().ok();

    let config = Config::from_env()?;

    let pool = PgPool::connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    let checkpoint = match std::env::var("CHECKPOINT") {
        Ok(s) => s.parse::<u64>().context("Failed to parse CHECKPOINT env var")?,
        Err(_) => {
            info!("CHECKPOINT env var missing, loading from DB");
            checkpoint::get_value(&pool).await.context("Failed to get checkpoint from DB")?
        }
    };

    info!(
        "Configuration loaded. Contract: {}, Checkpoint: {}",
        config.contract_address, checkpoint
    );

    let bot = Bot::try_new(&config.github_token).context("Failed to initialize GitHub bot")?;

    let concurrency = 10;
    let (tx, rx) = mpsc::channel(100);

    info!(
        "Launching pollers with concurrency={}, checkpoint={}",
        concurrency, checkpoint
    );

    if let Err(e) = tokio::try_join!(
        pollers::solidity_poller(tx, concurrency, config.contract_address, "", checkpoint, pool),
        pollers::github_poller(&bot, rx, concurrency),
    ) {
        error!("Fatal error: {:?}", e);
        return Err(e);
    }

    Ok(())
}
