use std::collections::HashMap;

use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
  Response,
};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  prop_house::{handler::Handler, Auction, Proposal, Vote},
  utils::{ens::get_wallet_handle, link::Link},
};

pub(crate) struct FarcasterHandler {
  base_url: String,
  warpcast_url: String,
  bearer_token: String,
  channel_key: String,
  cache: Cache,
  client: Client,
  link: Link,
}

impl FarcasterHandler {
  pub fn new(
    base_url: String,
    warpcast_url: String,
    bearer_token: String,
    channel_key: String,
    cache: Cache,
    client: Client,
    link: Link,
  ) -> Self {
    Self {
      base_url,
      warpcast_url,
      bearer_token,
      channel_key,
      cache,
      client,
      link,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("PROP_HOUSE_BASE_URL")?.to_string();
    let warpcast_url = env.var("WARP_CAST_API_BASE_URL")?.to_string();
    let bearer_token = env.secret("PROP_HOUSE_WARP_CAST_TOKEN")?.to_string();
    let channel_key = env.var("PROP_HOUSE_WARP_CAST_CHANNEL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();
    let link = Link::new_from_env(&env);

    Ok(Self::new(
      base_url,
      warpcast_url,
      bearer_token,
      channel_key,
      cache,
      client,
      link,
    ))
  }

  async fn make_http_request(&self, request_data: Value) -> Result<Response> {
    let url = format!("{}/casts", self.warpcast_url);
    let token = format!("Bearer {}", self.bearer_token);
    let mut headers = HeaderMap::new();

    let parsed_token =
      HeaderValue::from_str(&token).map_err(|_| Error::from("Error while parsing token"))?;

    headers.insert(AUTHORIZATION, parsed_token);
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = self
      .client
      .post(url)
      .headers(headers)
      .json(&request_data)
      .send()
      .await
      .map_err(|e| {
        error!("Failed to execute request: {}", e);
        Error::from(format!("Failed to execute request: {}", e))
      })?;

    debug!("Response status: {:?}", response.status());

    Ok(response)
  }
}

#[async_trait(? Send)]
impl Handler for FarcasterHandler {
  async fn handle_new_auction(&self, auction: &Auction) -> Result<()> {
    info!("Handling new auction: {}", auction.title);

    let url = format!(
      "{}/{}",
      self.base_url,
      auction.title.replace(' ', "-").to_lowercase()
    );
    let description = format!(
      "A new Prop House round has been created: “{}”",
      auction.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": self.channel_key
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }

  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    info!("Handling new proposal: {}", proposal.title);

    let auctions = self
      .cache
      .get::<Vec<Auction>>("prop_house:auctions")
      .await?
      .unwrap();

    let auction = auctions
      .iter()
      .find(|&a| a.id == proposal.auction_id)
      .clone()
      .ok_or("Auction not found in the funding list.")?;

    let url = format!(
      "{}/{}/{}",
      self.base_url,
      auction.title.replace(' ', "-").to_lowercase(),
      proposal.id
    );

    let wallet = get_wallet_handle(&proposal.address, "xyz.farcaster").await;

    let description = format!(
      "{} created a new proposal on Prop House: “{}”",
      wallet, proposal.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": self.channel_key
    });

    let response = self.make_http_request(request_data).await.map_err(|e| {
      error!("Failed to make HTTP request: {}", e);
      return e;
    })?;

    let response_body = response.text().await.map_err(|e| {
      error!("Failed to get text from response: {}", e);
      Error::from(format!("Failed to get text from response: {}", e))
    })?;

    let parsed_body: serde_json::Result<Value> = serde_json::from_str(&response_body);

    let response_body: Value = match parsed_body {
      Ok(body) => body,
      Err(e) => {
        error!("Failed to parse JSON: {}", e);
        return Err(e.into());
      }
    };

    let cast_hash = response_body["result"]["cast"]["hash"]
      .as_str()
      .unwrap_or_default();

    let mut proposals_casts = self
      .cache
      .get::<HashMap<isize, String>>("prop_house:proposals:casts")
      .await?
      .unwrap_or_default();

    proposals_casts.insert(proposal.id, cast_hash.to_string());

    self
      .cache
      .put("prop_house:proposals:casts", &proposals_casts)
      .await;

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.address);

    let proposals = self
      .cache
      .get::<Vec<Proposal>>("prop_house:proposals")
      .await?
      .unwrap();

    let proposal = proposals
      .iter()
      .find(|&a| a.id == vote.proposal_id)
      .clone()
      .ok_or("Proposal not found in the funding list.")?;

    let proposals_casts = self
      .cache
      .get::<HashMap<isize, String>>("prop_house:proposals:casts")
      .await?
      .unwrap_or_default();

    let cast_hash = proposals_casts
      .get(&proposal.id)
      .ok_or("Cast hash not found")?;

    let wallet = get_wallet_handle(&vote.address, "xyz.farcaster").await;

    let description = format!(
      "{} has voted {} “{}” proposal.",
      wallet,
      match vote.direction {
        1 => "for",
        _ => "against",
      },
      proposal.title
    );

    let request_data = json!({
      "text": description,
      "channelKey": self.channel_key,
      "parent": {"hash": cast_hash},
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }
}
