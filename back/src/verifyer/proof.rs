use PROOVER::WebProof;
use alloy::{primitives::{Address, ChainId}, providers::ProviderBuilder, sol};
use octocrab::models::issues::Issue;
use std::{process::Command, str::FromStr};
use tokio::task;
use tracing::{debug, error, info, instrument, trace, warn};
use url::Url;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    PROOVER,
    "contracts/proover.json"
);

/// Runs `vlayer web-proof-fetch` asynchronously and logs all steps.
// #[instrument(level = "info", skip(token))]
pub async fn generate_web_proof(notary_url: &str, token: &str, url: &str) -> String {
    info!("Generating web proof for URL: {}", url);

    let notary_url = notary_url.to_string();
    let token = token.to_string();
    let url = url.to_string();

    let output = task::spawn_blocking(move || {
        debug!("Executing `vlayer` command...");
        Command::new("vlayer")
            .args(&[
                "web-proof-fetch",
                "--notary",
                &notary_url,
                "--url",
                &url,
                "--header",
                &format!("Authorization: Bearer {}", token),
            ])
            .output()
    })
    .await
    .expect("Failed to spawn blocking task")
    .expect("Failed to execute `vlayer` command");


    if output.status.success() {
        let stdout = String::from_utf8(output.stdout.into_iter().collect()).unwrap();
        info!("Web proof generated successfully ({} bytes)", stdout.len());
        info!("Web proof JSON: {}", stdout);
        stdout
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Web proof generation failed: {}", stderr);
        panic!("CLI error: {}", stderr);
    }
}

/// Verifies the issue proof on-chain using the smart contract.
// #[instrument(level = "info", skip(rpc_url))]
pub async fn get_proof(
    issue: Issue,
    rpc_url: Url,
    contract_address: Address,
) -> anyhow::Result<PROOVER::verifyIssueReturn> {
    info!(
        "Verifying proof for GitHub issue #{} ({})",
        issue.number, issue.title
    );

    // let web_proof = generate_web_proof(
    //     "https://test-notary.vlayer.xyz/",
    //     issue.url.as_str(),
    // )
    // .await;

    debug!("Connecting to Ethereum provider at {}", rpc_url);
    let provider = ProviderBuilder::new().connect_http(Url::from_str("https://sepolia.optimism.io")?);
    let contract = PROOVER::new(contract_address, provider);

    info!("Calling smart contract at {}", contract_address);
    println!("{:#?}", WebProof {
                webProofJson: "".to_string(),
            });
    
    let result = contract
    .verifyIssue(
        WebProof {
            webProofJson: "é".to_string(),
        },
        "b3ww/vGrant".to_string(),
        issue.number.to_string(),
    ).chain_id(11155420);
    warn!("========================{result:#?}");
        // .send()
        // .await?;



    // info!("Contract returned: {:#?}", result);
    Ok(result.call().await?)
}
