use std::str::FromStr;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};
use tracing::{error, info};

use alloy::eips::BlockNumberOrTag;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::Log;
use alloy::rpc::types::eth::Filter;
use alloy::transports::http::reqwest::Url;

use crate::pollers::chain::events::Event;

use anyhow::Result;
use sqlx::PgPool;

use crate::github::bot::Bot;
use crate::pollers::chain::parse_event_from_log;
use crate::pollers::issues::IssueRef;

pub async fn solidity_poller(
    sender: mpsc::Sender<IssueRef>,
    bot: &Bot,
    contract_address: Address,
    rpc_url: &str,
    mut start_block: u64,
    pool: PgPool,
) -> Result<()> {
    let provider = ProviderBuilder::new().connect_http(Url::from_str(rpc_url)?);

    loop {
        let latest_block = provider.get_block_number().await? as u64;

        if start_block > latest_block {
            sleep(Duration::from_millis(200)).await;
            continue;
        }

        let filter = Filter::new()
            .address(contract_address)
            .from_block(BlockNumberOrTag::Number(start_block))
            .to_block(BlockNumberOrTag::Number(start_block));

        let logs: Vec<Log> = provider.get_logs(&filter).await?;

        let events: Vec<Event> = logs
            .into_iter()
            .filter_map(|log| parse_event_from_log(&log))
            .collect();

        for event in events {
            if let Err(e) = event.send(&sender, bot).await {
                error!("Error sending event: {:?}", e);
            }
        }

        crate::checkpoint::update_value(&pool, start_block).await?;
        info!(
            "Checkpoint updated: block {} has been scanned.",
            start_block
        );

        start_block += 1;
        sleep(Duration::from_millis(200)).await;
    }
}
