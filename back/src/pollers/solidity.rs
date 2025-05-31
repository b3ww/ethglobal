use alloy::sol_types::SolEvent;
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

use alloy::sol;

sol! {
    event BountyApproved(string issue);
    event BountyClaimed(string issue, address claimer);
}

use anyhow::{Result, anyhow};

use crate::pollers::IssueRef;
use sqlx::PgPool;

pub enum Event {
    BountyApproved(String),
    BountyClaimed(String, Address),
}

impl Event {
    pub async fn send(&self, sender: &mpsc::Sender<IssueRef>) -> Result<()> {
        match self {
            Self::BountyApproved(issue_url) => {
                let issue_ref = IssueRef::try_from(issue_url.as_str())
                    .map_err(|e| anyhow!("Failed to parse issue ref: {:?}", e))?;
                sender.send(issue_ref).await.map_err(|e| anyhow!("Failed to send issue ref: {:?}", e))?;
            }
            Self::BountyClaimed(_, _) => {
            }
        }
        Ok(())
    }
}

fn parse_event(log: &Log) -> Option<Event> {
    match log.topic0() {
        Some(&BountyApproved::SIGNATURE_HASH) => {
            match BountyApproved::decode_log_data(log.data()) {
                Ok(event) => {
                    info!(
                        "BountyApproved event found: issue = {}, block = {:?}",
                        event.issue,
                        log.block_number
                    );
                    Some(Event::BountyApproved(event.issue))
                }
                Err(e) => {
                    error!("Failed to decode BountyApproved: {:?}", e);
                    None
                }
            }
        }
        Some(&BountyClaimed::SIGNATURE_HASH) => {
            match BountyClaimed::decode_log_data(log.data()) {
                Ok(event) => {
                    info!(
                        "BountyClaimed event found: issue = {}, claimer = {:?}, block = {:?}",
                        event.issue,
                        event.claimer,
                        log.block_number
                    );
                    Some(Event::BountyClaimed(event.issue, event.claimer))
                }
                Err(e) => {
                    error!("Failed to decode BountyClaimed: {:?}", e);
                    None
                }
            }
        }
        _ => None,
    }
}


pub async fn solidity_poller(
    sender: mpsc::Sender<IssueRef>,
    _concurrency: usize,
    contract_address: Address,
    rpc_url: &str,
    mut start_block: u64,
    pool: PgPool,
) -> Result<()> {
    let provider = ProviderBuilder::new().connect_http(Url::from_str(rpc_url)?);

    loop {
        let latest_block = provider.get_block_number().await? as u64;

        if start_block > latest_block {
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        let filter = Filter::new()
            .address(contract_address)
            .from_block(BlockNumberOrTag::Number(start_block))
            .to_block(BlockNumberOrTag::Number(start_block));

        let logs: Vec<Log> = provider.get_logs(&filter).await?;

        let events = logs.into_iter().filter_map(|log| parse_event(&log));

        for event in events {
            if let Err(e) = event.send(&sender).await {
                error!("Error sending event: {:?}", e);
            }
        }

        crate::checkpoint::update_value(&pool, start_block).await?;

        start_block += 1;
    }
}
