use async_trait::async_trait;
use reqwest::Client;
use worker::{Env, Result};

use crate::{
  cache::Cache,
  lil_nouns::{handler::Handler, Proposal, Vote},
};

pub(crate) struct DiscordHandler {
  base_url: String,
  webhook_url: String,
  cache: Cache,
  client: Client,
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

  pub fn new_from_env(env: &Env) -> Result<DiscordHandler> {
    let base_url = env.var("LIL_NOUNS_BASE_URL")?.to_string();
    let webhook_url = env.secret("LIL_NOUNS_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(base_url, webhook_url, cache, client))
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
