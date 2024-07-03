use std::{convert::TryFrom, str::FromStr};

use anyhow::anyhow;
use ethers::{
  prelude::{Http, Provider, Transaction},
  providers::Middleware,
  types::H256,
};

const ETHEREUM_MAINNET_RPC_URL: &'static str = "https://eth.llamarpc.com";

async fn create_provider() -> anyhow::Result<Provider<Http>> {
  Provider::<Http>::try_from(ETHEREUM_MAINNET_RPC_URL)
    .map_err(|error| anyhow!("Failed to create provider from endpoint: {}", error))
}

async fn get_transaction_data(tx_hash: &str) -> anyhow::Result<Option<Transaction>> {
  // Initialize provider
  let provider = create_provider().await?;

  // Parse transaction id into H256
  let hash = match H256::from_str(tx_hash) {
    Ok(parsed_hash) => Ok(parsed_hash.into()),
    Err(_) => Err(anyhow!("Transaction hash is not a valid H256")),
  }?;

  // Fetch transaction data
  let tx_result = provider.get_transaction(hash).await;
  let tx = tx_result?; // Handling the Result error here
  Ok(tx)
}
