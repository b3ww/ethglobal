use crate::github::error::{BotError, Result};
use octocrab::{Octocrab, models::issues::Issue};

pub struct Bot {
    client: Octocrab,
}

impl Bot {
    pub fn try_new(token: &str) -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;
        Ok(Self { client })
    }

    pub async fn fetch_issue(&self, owner: &str, repo: &str, number: u64) -> Result<Issue> {
        Ok(self.client.issues(owner, repo).get(number).await?)
    }

    pub async fn add_issue_comment(&self,  owner: &str, repo: &str, number: u64, body: &str) -> Result<()> {
        let _ = self.client.issues(owner, repo).create_comment(number, body).await?;
        Ok(())
    }
}
