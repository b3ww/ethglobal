mod github;
use github::bot::Bot;

mod pollers;
use dotenvy::dotenv;

use tokio::sync::mpsc;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(100); // buffer size de 100 messages
    let _ = dotenv();
    let bot = Bot::try_new(&std::env::var("GITHUB_BOT_TOKEN").unwrap()).unwrap(); // ou passe tes configs ici
    let concurrency = 10;

    // Lancer les deux pollers en parallèle
    tokio::try_join!(
        pollers::solidity_poller(tx, concurrency),
        pollers::github_poller(&bot, rx, concurrency)
    )?;

    Ok(())
}
