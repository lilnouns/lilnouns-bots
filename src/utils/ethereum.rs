use std::{convert::TryFrom, str::FromStr};

use anyhow::{anyhow, Result};
use ethers::{
  prelude::{Http, Provider, Transaction},
  providers::Middleware,
  types::{Address, H256},
};

const ETHEREUM_MAINNET_RPC_URL: &'static str = "https://eth.llamarpc.com";

async fn create_provider() -> Result<Provider<Http>> {
  Provider::<Http>::try_from(ETHEREUM_MAINNET_RPC_URL)
    .map_err(|error| anyhow!("Failed to create provider from endpoint: {}", error))
}

pub async fn get_transaction_data(tx_hash: &str) -> Result<Option<Transaction>> {
  // Initialize provider
  let provider = create_provider().await?;

  // Parse transaction id into H256
  let hash = match H256::from_str(tx_hash) {
    Ok(parsed_hash) => parsed_hash,
    Err(_) => return Err(anyhow!("Transaction hash is not a valid H256")),
  };

  // Fetch transaction data
  let tx_result = provider.get_transaction(hash).await;
  let tx = tx_result?; // Handling the Result error here
  Ok(tx)
}

pub async fn get_transaction_signer(tx_hash: &str) -> Result<Address> {
  // Get transaction data
  let tx = match get_transaction_data(tx_hash).await? {
    Some(transaction) => transaction,
    None => return Err(anyhow!("No transaction data was fetched")),
  };

  // Check if `tx.from` is properly defined, if not, return an error.
  let from_address = tx.from;
  let invalid_address = Address::from_slice(&[0u8; 20]); // The Ethereum address made entirely of zeroes is often considered "invalid"
  if from_address == invalid_address {
    return Err(anyhow!("Transaction does not have a signer address"));
  }

  Ok(from_address)
}
