use std::ops::Add;

use crate::github::bot::Bot;
use crate::github::{close_issue, increase_deadline, open_issue};
use crate::pollers::IssueRef;
use alloy::primitives::{Address, I256, U256};
use alloy::rpc::types::Log;
use alloy::sol;
use alloy::sol_types::SolEvent;
use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use tracing::{error, info};

sol! {
    event BountyApproved(string url, uint256 bounty, uint256 deadline, address author);
    event BountyClaimed(string url, uint256 bounty, address dev, int256 githubId);
    event BountyIncreasedDeadline(string url, uint256 newDeadline, uint256 oldDeadline);
}

pub enum Event {
    BountyApproved {
        url: String,
        price: U256,
        deadline: U256,
        author: Address,
    },
    BountyIncreasedDeadline {
        url: String,
        new: U256,
        old: U256,
    },
    BountyClaimed {
        url: String,
        price: U256,
        dev_address: Address,
        github_id: I256,
    },
}

impl Event {
    pub async fn send(&self, sender: &mpsc::Sender<IssueRef>, bot: &Bot) -> Result<()> {
        match self {
            Self::BountyApproved {
                url,
                price,
                deadline,
                author,
            } => {
                let issue_ref = IssueRef::try_from(url.as_str())
                    .map_err(|e| anyhow!("Failed to parse issue ref: {:?}", e))?;
                sender
                    .send(issue_ref.clone())
                    .await
                    .map_err(|e| anyhow!("Failed to send issue ref: {:?}", e))?;
                let _ = bot
                    .add_issue_comment(
                        &issue_ref.owner,
                        &issue_ref.repo,
                        issue_ref.number,
                        &open_issue(*author, *price, *deadline),
                    )
                    .await;
            }
            Self::BountyIncreasedDeadline { url, new, old } => {
                let issue_ref = IssueRef::try_from(url.as_str())
                    .map_err(|e| anyhow!("Failed to parse issue ref: {:?}", e))?;

                let _ = bot
                    .add_issue_comment(
                        &issue_ref.owner,
                        &issue_ref.repo,
                        issue_ref.number,
                        &increase_deadline(*old, *new),
                    )
                    .await;
            }
            Self::BountyClaimed { url, price, dev_address, github_id } => {
                let issue_ref = IssueRef::try_from(url.as_str())
                    .map_err(|e| anyhow!("Failed to parse issue ref: {:?}", e))?;

                let _ = bot
                    .add_issue_comment(
                        &issue_ref.owner,
                        &issue_ref.repo,
                        issue_ref.number,
                        &close_issue(*price, *dev_address, *github_id),
                    )
                    .await;
            }
        }
        Ok(())
    }
}

pub fn parse_event_from_log(log: &Log) -> Option<Event> {
    println!(
        "{:?} {:?}",
        log.topic0(),
        Some(&BountyApproved::SIGNATURE_HASH)
    );
    match log.topic0() {
        Some(&BountyApproved::SIGNATURE_HASH) => {
            println!("ds,jfgklùswjndfgklsqjwedgilkjwsdlgmk");
            match BountyApproved::decode_log_data(log.data()) {
                Ok(event) => {
                    info!(
                        "BountyApproved event found: issue = {}, block = {:?}",
                        event.url, log.block_number
                    );
                    Some(Event::BountyApproved {
                        url: event.url,
                        price: event.bounty,
                        deadline: event.deadline,
                        author: event.author,
                    })
                }
                Err(e) => {
                    error!("Failed to decode BountyApproved: {:?}", e);
                    None
                }
            }
        }
        Some(&BountyIncreasedDeadline::SIGNATURE_HASH) => {
            match BountyIncreasedDeadline::decode_log_data(log.data()) {
                Ok(event) => {
                    info!(
                        "BountyIncreasedDeadline event found: issue = {}, newDeadline = {:?}, block = {:?}",
                        event.url, event.newDeadline, log.block_number
                    );
                    Some(Event::BountyIncreasedDeadline {
                        url: event.url,
                        new: event.newDeadline,
                        old: event.oldDeadline,
                    })
                }
                Err(e) => {
                    error!("Failed to decode BountyIncreasedDeadline: {:?}", e);
                    None
                }
            }
        }
        Some(&BountyClaimed::SIGNATURE_HASH) => match BountyClaimed::decode_log_data(log.data()) {
            Ok(event) => {
                info!(
                    "BountyClaimed event found: issue = {}, block = {:?}",
                    event.url, log.block_number
                );
                Some(Event::BountyClaimed {
                    url: event.url,
                    price: event.bounty,
                    dev_address: event.dev,
                    github_id: event.githubId,
                })
            }
            Err(e) => {
                error!("Failed to decode BountyClaimed: {:?}", e);
                None
            }
        },
        _ => None,
    }
}
