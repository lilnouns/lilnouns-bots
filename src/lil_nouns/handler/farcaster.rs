use std::collections::HashMap;

use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::{
  Client,
  header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
  Response,
};
use serde_json::{json, to_string, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  lil_nouns::{handler::Handler, Proposal, Vote},
  utils::{fname::get_username_by_address, link::Link},
};

pub(crate) struct FarcasterHandler {
  base_url: String,
  warpcast_url: String,
  warpcast_bearer_token: String,
  warpcast_channel_key: String,
  farquest_api_key: String,
  cache: Cache,
  client: Client,
  link: Link,
}

impl FarcasterHandler {
  pub fn new(
    base_url: String,
    warpcast_url: String,
    warpcast_bearer_token: String,
    warpcast_channel_key: String,
    farquest_api_key: String,
    cache: Cache,
    client: Client,
    link: Link,
  ) -> Self {
    Self {
      base_url,
      warpcast_url,
      warpcast_bearer_token,
      warpcast_channel_key,
      farquest_api_key,
      cache,
      client,
      link,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("LIL_NOUNS_BASE_URL")?.to_string();
    let warpcast_url = env.var("WARPCAST_API_BASE_URL")?.to_string();
    let warpcast_bearer_token = env.secret("LIL_NOUNS_WARPCAST_TOKEN")?.to_string();
    let warpcast_channel_key = env.var("LIL_NOUNS_WARPCAST_CHANNEL")?.to_string();
    let farquest_api_key = env.secret("FARQUEST_API_KEY")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();
    let link = Link::new_from_env(&env);

    Ok(Self::new(
      base_url,
      warpcast_url,
      warpcast_bearer_token,
      warpcast_channel_key,
      farquest_api_key,
      cache,
      client,
      link,
    ))
  }

  async fn make_http_request(&self, request_data: Value) -> Result<Response> {
    let url = format!("{}/casts", self.warpcast_url);
    let token = format!("Bearer {}", self.warpcast_bearer_token);
    let mut headers = HeaderMap::new();

    let parsed_token =
      HeaderValue::from_str(&token).map_err(|_| Error::from("Error while parsing token"))?;

    headers.insert(AUTHORIZATION, parsed_token);
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Send the HTTP POST request
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
  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    info!("Handling new proposal: {}", proposal.title);

    let url = &self
      .link
      .generate(format!("{}/{}", self.base_url, proposal.id))
      .await
      .unwrap_or_else(|_| format!("{}/{}", self.base_url, proposal.id));

    let wallet = get_username_by_address(self.farquest_api_key.as_str(), &proposal.proposer).await;

    let description = format!(
      "{} created a new proposal on Lil Nouns: “{}”",
      wallet, proposal.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": self.warpcast_channel_key
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
      .ok_or("Failed to get cast hash")?;
    debug!("Cast hash: {}", cast_hash);

    let mut proposals_casts = self
      .cache
      .get::<HashMap<String, String>>("lil_nouns:proposals:casts")
      .await?
      .ok_or("Failed to retrieve proposals casts")?;
    debug!("Proposals casts before insertion: {:?}", proposals_casts);

    proposals_casts.insert(proposal.id.to_string(), cast_hash.to_string());
    debug!("Proposals casts after insertion: {:?}", proposals_casts);

    let proposals_casts_as_string = to_string(&proposals_casts).unwrap();
    debug!("Ideas casts as string: {}", proposals_casts_as_string);

    self
      .cache
      .put("lil_nouns:proposals:casts", &proposals_casts_as_string)
      .await;
    debug!("Finished putting proposals casts in cache");

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.voter);

    let proposals = self
      .cache
      .get::<Vec<Proposal>>("lil_nouns:proposals")
      .await?
      .unwrap_or_default();

    let proposal = proposals
      .iter()
      .find(|&a| a.id == vote.proposal_id)
      .cloned()
      .ok_or("Proposal not found in the funding list.")?;

    let proposals_casts = self
      .cache
      .get::<HashMap<String, String>>("lil_nouns:proposals:casts")
      .await?
      .ok_or("Failed to retrieve proposals casts")?;

    let cast_hash = proposals_casts
      .get(&proposal.id.to_string())
      .ok_or("Cast hash not found")?;

    let wallet = get_username_by_address(self.farquest_api_key.as_str(), &vote.voter).await;

    let mut description = format!(
      "{} has voted {} “{}” proposal.",
      wallet,
      match vote.direction {
        0 => "against",
        1 => "for",
        2 => "abstain on",
        _ => "unknown",
      },
      proposal.title
    );

    if !vote.reason.is_none() {
      let chars_limit = 1024 - 10 - description.len();
      let mut vote_reason = vote.clone().reason.unwrap_or_default();
      if vote_reason.len() > chars_limit {
        vote_reason.truncate(chars_limit);
        vote_reason.push_str("...");
      }
      description = format!("{}\n\n“{}”", description, vote_reason);
    }

    let request_data = json!({
      "text": description,
      "channelKey": self.warpcast_channel_key,
      "parent": {"hash": cast_hash},
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }
}
