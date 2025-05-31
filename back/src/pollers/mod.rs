mod github;
pub use github::github_poller;

mod solidity;
pub use solidity::solidity_poller;

mod issues;
use issues::IssueRef;