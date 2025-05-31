use regex::Regex;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct IssueRef {
    pub owner: String,
    pub repo: String,
    pub number: u64,
}

impl TryFrom<&str> for IssueRef {
    type Error = anyhow::Error;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(
            r"^https://(?:api\.github\.com/repos|github\.com)/([^/]+)/([^/]+)/issues/(\d+)"
        )?;

        let caps = re.captures(url)
            .ok_or_else(|| anyhow::anyhow!("URL invalide : {}", url))?;

        Ok(IssueRef {
            owner: caps[1].to_string(),
            repo: caps[2].to_string(),
            number: caps[3].parse()?,
        })
    }
}
