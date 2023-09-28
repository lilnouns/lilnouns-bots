use anyhow::{anyhow, Result};
use log::error;
use worker::Env;

pub struct Config {
  pub(crate) base_url: String,
  pub(crate) graphql_url: String,
  pub(crate) bearer_token: String,
  pub(crate) webhook_url: String,
}

impl Config {
  pub fn new(
    base_url: String,
    graphql_url: String,
    bearer_token: String,
    webhook_url: String,
  ) -> Self {
    Self {
      base_url,
      graphql_url,
      bearer_token,
      webhook_url,
    }
  }

  pub(crate) fn new_from_env(env: &Env) -> Result<Self> {
    let base_url = env
      .var("PROP_LOT_BASE_URL")
      .map_err(|e| {
        let error_message = "Failed to get PROP_LOT_BASE_URL from env";
        error!("{}: {:?}", error_message, e);
        anyhow!(error_message)
      })?
      .to_string();

    let graphql_url = env
      .var("PROP_LOT_GRAPHQL_URL")
      .map_err(|e| {
        let error_message = "Failed to get PROP_LOT_GRAPHQL_URL from env";
        error!("{}: {:?}", error_message, e);
        anyhow!(error_message)
      })?
      .to_string();

    let bearer_token = env
      .secret("PROP_LOT_WARP_CAST_TOKEN")
      .map_err(|e| {
        let error_message = "Failed to get PROP_LOT_WARP_CAST_TOKEN from env";
        error!("{}: {:?}", error_message, e);
        anyhow!(error_message)
      })?
      .to_string();

    let webhook_url = env
      .secret("PROP_LOT_DISCORD_WEBHOOK_URL")
      .map_err(|e| {
        let error_message = "Failed to get PROP_LOT_DISCORD_WEBHOOK_URL from env";
        error!("{}: {:?}", error_message, e);
        anyhow!(error_message)
      })?
      .to_string();

    Ok(Self::new(base_url, graphql_url, bearer_token, webhook_url))
  }
}
