use crate::{Bot, pollers::IssueRef};
use alloy::primitives::Address;
use anyhow::Result;
use dotenvy::from_path_iter;
use futures::{StreamExt, stream};
use octocrab::models::IssueState;
use std::env;
use std::path::Path;
use std::process::Command;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};
use tracing::info;
use url::Url;

pub async fn github_poller(
    bot: &Bot,
    mut receiver: mpsc::Receiver<IssueRef>,
    concurrency: usize,
    rpc_url: Url,
    proof_contract_addr: Address,
    verify_contract_addr: Address,
) -> Result<()> {
    let mut issues = Vec::new();

    loop {
        while let Ok(new_issue) = receiver.try_recv() {
            issues.push(new_issue);
        }

        let fetches = stream::iter(issues.into_iter())
            .map(|issue| {
                let bot = bot;
                async move {
                    let fetched = bot
                        .fetch_issue(&issue.owner, &issue.repo, issue.number)
                        .await?;
                    Ok::<_, anyhow::Error>((issue, fetched))
                }
            })
            .buffer_unordered(concurrency);

        let mut remaining_issues = Vec::new();

        futures::pin_mut!(fetches);

        while let Some(res) = fetches.next().await {
            match res {
                Ok((issue, data)) => {
                    if data.state == IssueState::Open {
                        remaining_issues.push(issue);
                    } else {
                        info!(
                            "Issue {}#{} is closed. It will be dropped.",
                            issue.repo, issue.number
                        );
                        let env_path = Path::new("../../vGrant-contract/vlayer/.env.testnet");

                        let env_vars =
                            from_path_iter(env_path).expect("Failed to read .env.testnet");
                        unsafe {
                            env::set_var("ISSUE_ID", issue.number.to_string());
                            env::set_var("ISSUE_URL", data.url.to_string());
                        }
                        let mut command = Command::new("bun");
                        command.args(&["run", "prove:testnet"]);
                        command.current_dir(Path::new("../../vGrant-contract/vlayer/"));

                        for item in env_vars {
                            if let Ok((key, value)) = item {
                                command.env(key, value);
                            }
                        }
                        let output = command.output().expect("Failed to execute bun script");

                        // .output()
                        // .expect("failed to execute script");

                        if output.status.success() {
                            println!("✅ Script executed successfully");
                        } else {
                            eprintln!(
                                "❌ Script failed: {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                        }
                        // .await?
                    }
                }
                Err(_) => {}
            }
        }

        issues = remaining_issues;

        sleep(Duration::from_secs(2)).await;
    }
}
