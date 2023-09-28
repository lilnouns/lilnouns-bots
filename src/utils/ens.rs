use std::str::FromStr;

use anyhow::{anyhow, Result};
use ethers::{addressbook::Address, middleware::Middleware, prelude::Provider, providers::Http};

const ETHEREUM_MAINNET_RPC_URL: &'static str = "https://eth.llamarpc.com";

async fn create_provider() -> Result<Provider<Http>> {
  Provider::<Http>::try_from(ETHEREUM_MAINNET_RPC_URL)
    .map_err(|error| anyhow!("Failed to create provider from endpoint: {}", error))
}

pub async fn get_domain_name(address: &str) -> Result<String> {
  let provider = create_provider().await?;

  let ethereum_address =
    Address::from_str(address).map_err(|error| anyhow!("Failed to parse address: {}", error))?;

  let domain_name = provider
    .lookup_address(ethereum_address)
    .await
    .map_err(|error| anyhow!("Failed to lookup address: {}", error))?;

  Ok(domain_name)
}

pub async fn get_domain_field(domain: &str, field: &str) -> Result<String> {
  let provider = create_provider().await?;

  let value = provider
    .resolve_field(domain, field)
    .await
    .map_err(|error| anyhow!("Failed to resolve field: {}", error))?;

  Ok(value)
}
