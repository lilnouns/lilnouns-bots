use async_trait::async_trait;
use log::error;
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::cache::Cache;
use crate::meta_gov::fetcher::{Proposal, Vote};
use crate::meta_gov::handler::Handler;

pub struct DiscordHandler {
  pub base_url: String,
  pub webhook_url: String,
  pub cache: Cache,
  pub client: Client,
}

impl DiscordHandler {
  pub fn new(base_url: String, webhook_url: String, cache: Cache, client: Client) -> Self {
    Self {
      base_url,
      webhook_url,
      cache,
      client,
    }
  }

  pub fn from(env: &Env) -> Result<DiscordHandler> {
    let base_url = env.var("META_GOV_BASE_URL")?.to_string();
    let webhook_url = env.secret("META_GOV_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::from(env);
    let client = Client::new();

    Ok(Self::new(base_url, webhook_url, cache, client))
  }

  async fn execute_webhook(&self, embed: Value) -> Result<()> {
    let msg_json = json!({"embeds": [embed]});

    self
      .client
      .post(&self.webhook_url)
      .header(header::CONTENT_TYPE, "application/json")
      .body(msg_json.to_string())
      .send()
      .await
      .map_err(|e| {
        error!("Failed to execute webhook: {}", e);
        Error::from(format!("Failed to execute webhook: {}", e))
      })?;

    Ok(())
  }
}

#[async_trait(? Send)]
impl Handler for DiscordHandler {
  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    todo!()
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    todo!()
  }
}
