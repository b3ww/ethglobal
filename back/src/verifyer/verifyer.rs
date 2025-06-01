use octocrab::models::issues::Issue;
use std::str;
use url::Url;

use alloy::{primitives::Address, providers::ProviderBuilder, sol, sol_types::SolValue};

use crate::verifyer::get_proof;
use tracing::{debug, error, info, instrument, warn};

sol!(
    #[derive(Debug)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    VERIFYER,
    "contracts/verifyer.json"
);

#[instrument(level = "info", skip(rpc_url, issue))]
pub async fn verify(
    issue: Issue,
    rpc_url: Url,
    proof_contract_address: Address,
    verify_contract_address: Address,
) -> anyhow::Result<()> {
    info!(
        "Starting on-chain verification for GitHub issue #{} ({})",
        issue.number, issue.title
    );

    debug!("Connecting to RPC provider at {}", rpc_url);
    let provider = ProviderBuilder::new().connect_http(rpc_url.clone());

    let contract = VERIFYER::new(verify_contract_address, provider);
    info!("Loaded VERIFYER contract at {}", verify_contract_address);
    warn!("begin ");
    let proof = get_proof(issue, rpc_url, proof_contract_address).await?;
    warn!("after");
    debug!("Proof successfully obtained from `get_proof`");

    let decoded_proof = VERIFYER::Proof::abi_decode(&proof._0.abi_encode())?;
    debug!("Proof decoded successfully");

    println!("==============> {:?}", decoded_proof);
    let call = contract.verify(decoded_proof, proof._1, proof._2, proof._3);
    let result = call.call().await;

    match result {
        Ok(_) => {
            info!("Proof successfully verified on-chain");
            Ok(())
        }
        Err(e) => {
            error!("Smart contract verification failed: {:#?}", e);
            Err(e.into())
        }
    }
}
