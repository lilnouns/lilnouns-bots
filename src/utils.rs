use std::str::FromStr;

use anyhow::{anyhow, Result};
use ethers::{
  prelude::Provider,
  providers::{Http, Middleware},
  types::Address,
};

pub async fn get_domain_name(address: &str) -> Result<String> {
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

pub fn get_short_address(address: &str) -> String {
  let first_four: String = address.chars().take(4).collect();
  let last_four: String = address
    .chars()
    .rev()
    .take(4)
    .collect::<Vec<char>>()
    .iter()
    .rev()
    .collect();

  if first_four.len() == 4 && last_four.len() == 4 {
    format!("{}...{}", first_four, last_four)
  } else {
    address.to_string()
  }
}

pub fn get_explorer_address(address: &str) -> String {
  format!("https://etherscan.io/address/{}", address)
}
