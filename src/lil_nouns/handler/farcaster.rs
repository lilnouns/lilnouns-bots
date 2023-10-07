use async_trait::async_trait;
use reqwest::Client;
use worker::{Env, Result};

use crate::{
  cache::Cache,
  lil_nouns::{handler::Handler, Proposal, Vote},
};

pub(crate) struct FarcasterHandler {
  base_url: String,
  bearer_token: String,
  channel_key: String,
  cache: Cache,
  client: Client,
}

impl FarcasterHandler {
  pub fn new(
    base_url: String,
    bearer_token: String,
    channel_key: String,
    cache: Cache,
    client: Client,
  ) -> Self {
    Self {
      base_url,
      bearer_token,
      channel_key,
      cache,
      client,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("LIL_NOUNS_BASE_URL")?.to_string();
    let bearer_token = env.secret("LIL_NOUNS_WARP_CAST_TOKEN")?.to_string();
    let channel_key = env.var("LIL_NOUNS_WARP_CAST_CHANNEL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(
      base_url,
      bearer_token,
      channel_key,
      cache,
      client,
    ))
  }
}

#[async_trait(? Send)]
impl Handler for FarcasterHandler {
  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    todo!()
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    todo!()
  }
}
