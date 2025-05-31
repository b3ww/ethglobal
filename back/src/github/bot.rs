use octocrab::{Octocrab, models::issues::Issue};
use std::ops::Deref;
use crate::github::error::{Result, BotError};

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

    pub async fn create_issue(
        &self,
        repo_owner: &str,
        repo_name: &str,
        title: &str,
        body: &str,
    ) -> Result<Issue> {
        Ok(self
            .issues(repo_owner, repo_name)
            .create(title)
            .body(body)
            .send()
            .await?)
    }
}

impl Deref for Bot {
    type Target = Octocrab;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
