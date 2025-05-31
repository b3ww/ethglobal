use octocrab::models::IssueState;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::pollers::IssueRef;

// TODO

pub async fn solidity_poller(
    sender: mpsc::Sender<IssueRef>,
    _concurrency: usize,
) -> Result<()> {
    let repos = vec![
        ("openai", "gpt"),
        ("rust-lang", "rust"),
        ("ethereum", "solidity"),
    ];

    let mut rng = rand::thread_rng();

    loop {
        if let Some((owner, repo)) = repos.choose(&mut rng) {
            let issue_number = rng.gen_range(1..=500);

            let issue = IssueRef {
                owner: owner.to_string(),
                repo: repo.to_string(),
                number: issue_number,
            };

            if let Err(e) = sender.send(issue).await {
                eprintln!("Erreur d'envoi: {:?}", e);
            } else {
                println!("Issue envoyée: {}/{}#{}", owner, repo, issue_number);
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}
