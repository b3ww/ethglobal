use octocrab::models::IssueState;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use anyhow::Result;
use futures::{stream, StreamExt};
use crate::{pollers::IssueRef, Bot};

pub async fn github_poller(
    bot: &Bot,
    mut receiver: mpsc::Receiver<IssueRef>,
    concurrency: usize,
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
                let fetched = bot.fetch_issue(&issue.owner, &issue.repo, issue.number).await?;
                    Ok::<_, anyhow::Error>((issue, fetched.state))
                }
            })
            .buffer_unordered(concurrency);

        let mut remaining_issues = Vec::new();

        futures::pin_mut!(fetches);

        while let Some(res) = fetches.next().await {
            match res {
                Ok((issue, state)) => {
                    if state == IssueState::Open {
                        remaining_issues.push(issue);
                    } else {
                        // close_grant()
                    }
                }
                Err(e) => {
                    // Gestion erreur
                    // Par défaut on ignore et continue
                }
            }
        }

        issues = remaining_issues;

        sleep(Duration::from_secs(15)).await;
    }
}

