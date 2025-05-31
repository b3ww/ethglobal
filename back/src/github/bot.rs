use crate::github::error::Result;
use octocrab::{Octocrab, models::issues::Issue};
use tracing::info;

pub struct Bot {
    client: Octocrab,
}

impl Bot {
    pub fn try_new(token: &str) -> Result<Self> {
        info!("Initializing Octocrab client.");
        let client = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;
        info!("Octocrab client initialized successfully.");
        Ok(Self { client })
    }

    pub async fn fetch_issue(&self, owner: &str, repo: &str, number: u64) -> Result<Issue> {
        info!("Fetching issue #{number} from {owner}/{repo}.");
        let issue = self.client.issues(owner, repo).get(number).await?;
        info!("Successfully fetched issue #{number}.");
        Ok(issue)
    }

    pub async fn add_issue_comment(&self, owner: &str, repo: &str, number: u64, body: &str) -> Result<()> {
        info!("Adding comment to issue #{number} in {owner}/{repo}.");
        let _ = self.client.issues(owner, repo).create_comment(number, body).await?;
        info!("Successfully added comment to issue #{number}.");
        Ok(())
    }
}
