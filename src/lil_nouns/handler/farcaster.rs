use async_trait::async_trait;
use log::info;
use reqwest::Client;
use serde_json::json;
use worker::{Env, Result};

use crate::{
  cache::Cache,
  lil_nouns::{handler::Handler, Proposal, Vote},
  utils::ens::get_wallet_handle,
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
    info!("Handling new proposal: {}", proposal.title);

    let url = format!("{}/{}", self.base_url, proposal.id);

    let wallet = get_wallet_handle(&proposal.proposer, "xyz.farcaster").await;

    let description = format!(
      "{} created a new proposal on Lil Nouns: “{}”",
      wallet, proposal.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": self.channel_key
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.voter);

    let proposals = self
      .cache
      .get::<Vec<Proposal>>("lil_nouns:proposals")
      .await?
      .unwrap();

    let proposal = proposals
      .iter()
      .find(|&a| a.id == vote.proposal_id)
      .unwrap()
      .clone();

    let url = format!("{}/{}", self.base_url, proposal.id);

    let wallet = get_wallet_handle(&vote.voter, "xyz.farcaster").await;

    let description = format!(
      "{} has voted {} “{}” proposal.",
      wallet,
      match vote.direction {
        0 => "against",
        1 => "for",
        2 => "abstain on",
        _ => "unknown",
      }
      proposal.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": self.channel_key
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }
}
