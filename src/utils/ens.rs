use std::str::FromStr;

use anyhow::anyhow;
use ethers::{addressbook::Address, middleware::Middleware, prelude::Provider, providers::Http};

pub async fn get_domain_name(address: &str) -> anyhow::Result<String> {
  let endpoint = "https://eth.llamarpc.com";
  let provider = Provider::<Http>::try_from(endpoint)
    .map_err(|error| anyhow!("Failed to create provider from endpoint: {}", error))?;

  let ethereum_address =
    Address::from_str(address).map_err(|error| anyhow!("Failed to parse address: {}", error))?;

  let domain_name = provider
    .lookup_address(ethereum_address)
    .await
    .map_err(|error| anyhow!("Failed to lookup address: {}", error))?;

  Ok(domain_name)
}
