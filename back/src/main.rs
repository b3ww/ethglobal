use std::str::FromStr;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

use alloy::primitives::Address;
use sqlx::PgPool;
use url::Url;

mod checkpoint;
mod github;
mod pollers;
mod verifyer;

use github::bot::Bot;

#[derive(Debug, Clone)]
struct Config {
    database_url: String,
    github_token: String,
    contract_address: Address,
    proof_contract_address: Address,
    rpc_url: Url,
}

impl Config {
    fn from_env() -> Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").context("Missing DATABASE_URL environment variable")?;
        let github_token =
            std::env::var("GITHUB_BOT_TOKEN").context("Missing GITHUB_BOT_TOKEN env var")?;
        let contract_address_str = std::env::var("VGRANT_CONTRACT_ADDRESS")
            .context("Missing VGRANT_CONTRACT_ADDRESS env var")?;
        let proof_contract_address_str = std::env::var("VGRANT_PROOF_CONTRACT_ADDRESS")
            .context("Missing VGRANT_PROOF_CONTRACT_ADDRESS env var")?;
        let rpc_url_str =
            std::env::var("RPC_URL").context("Missing RPC_URL environment variable")?;

        let contract_address = Address::from_str(&contract_address_str)
            .context("Failed to parse VGRANT_CONTRACT_ADDRESS")?;
        let proof_contract_address = Address::from_str(&proof_contract_address_str)
            .context("Failed to parse VGRANT_PROOF_CONTRACT_ADDRESS")?;
        let rpc_url = Url::from_str(&rpc_url_str).context("Failed to parse RPC_URL")?;

        Ok(Self {
            database_url,
            github_token,
            contract_address,
            proof_contract_address,
            rpc_url,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global logger");

    info!("🔧 Loading configuration...");
    let config = Config::from_env()?;

    info!("🔌 Connecting to database...");
    let pool = PgPool::connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    let checkpoint = match std::env::var("CHECKPOINT") {
        Ok(s) => s
            .parse::<u64>()
            .context("Failed to parse CHECKPOINT env var")?,
        Err(_) => {
            info!("No CHECKPOINT specified. Loading from DB...");
            checkpoint::get_value(&pool)
                .await
                .context("Failed to get checkpoint from DB")?
        }
    };

    info!(
        "✅ Config loaded. Contract: {}, Proof contract: {}, Checkpoint: {}",
        config.contract_address, config.proof_contract_address, checkpoint
    );

    let bot = Bot::try_new(&config.github_token).context("Failed to initialize GitHub bot")?;

    let concurrency = 10;
    let (tx, rx) = mpsc::channel(100);

    info!(
        "🚀 Launching pollers (concurrency={}, checkpoint={})...",
        concurrency, checkpoint
    );

    if let Err(e) = tokio::try_join!(
        pollers::solidity_poller(
            tx,
            &bot,
            config.contract_address,
            &config.rpc_url.as_str(),
            checkpoint,
            pool
        ),
        pollers::github_poller(
            &bot,
            rx,
            concurrency,
            config.rpc_url.clone(),
            config.proof_contract_address,
            config.contract_address
        ),
    ) {
        error!("💥 Fatal error: {:?}", e);
        return Err(e);
    }

    Ok(())
}
