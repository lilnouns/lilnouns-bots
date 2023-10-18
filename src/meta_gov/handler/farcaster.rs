use std::collections::HashMap;

use async_trait::async_trait;
use log::{debug, error, info};
use regex::Regex;
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
  Response,
};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  meta_gov::{handler::Handler, Proposal, Vote},
  utils::{ens::get_wallet_handle, link::Link},
};

pub struct FarcasterHandler {
  base_url: String,
  bearer_token: String,
  channel_key: String,
  cache: Cache,
  client: Client,
  link: Link,
}

impl FarcasterHandler {
  pub fn new(
    base_url: String,
    bearer_token: String,
    channel_key: String,
    cache: Cache,
    client: Client,
    link: Link,
  ) -> Self {
    Self {
      base_url,
      bearer_token,
      channel_key,
      cache,
      client,
      link,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("META_GOV_BASE_URL")?.to_string();
    let bearer_token = env.secret("META_GOV_WARP_CAST_TOKEN")?.to_string();
    let channel_key = env.secret("META_GOV_WARP_CAST_CHANNEL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();
    let link = Link::new_from_env(&env);

    Ok(Self::new(
      base_url,
      bearer_token,
      channel_key,
      cache,
      client,
      link,
    ))
  }

  async fn make_http_request(&self, request_data: Value) -> Result<Response> {
    let url = "https://api.warpcast.com/v2/casts";
    let token = format!("Bearer {}", self.bearer_token);
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

  async fn extract_proposal_info(&self, proposal: Proposal) -> Result<(String, String)> {
    let captures = Regex::new(r"(\d+): (.+)")
      .unwrap()
      .captures(&*proposal.title)
      .ok_or(Error::from("Capture Failed"))?;
    let proposal_id = captures
      .get(1)
      .ok_or(Error::from("Failed to get proposal ID"))?;
    let proposal_title = captures
      .get(2)
      .ok_or(Error::from("Failed to get proposal Title"))?;
    let proposal_id = proposal_id.as_str().to_string();
    let proposal_title = proposal_title.as_str().to_string();

    Ok((proposal_id, proposal_title))
  }
}

#[async_trait(? Send)]
impl Handler for FarcasterHandler {
  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    match self.extract_proposal_info(proposal.clone()).await {
      Ok((proposal_id, proposal_title)) => {
        info!("Handling new proposal: {}", proposal_title);

        let url = &self
          .link
          .generate(format!("{}/{}", self.base_url, proposal_id))
          .await
          .unwrap_or_else(|_| format!("{}/{}", self.base_url, proposal_id));

        let description = format!(
          "A new Meta Gov proposal has been created: “{}”",
          proposal_title
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
          .get::<HashMap<String, String>>("meta_gov:proposals:casts")
          .await?
          .unwrap_or_default();

        proposals_casts.insert(proposal_id, cast_hash.to_string());

        self
          .cache
          .put("meta_gov:proposals:casts", &proposals_casts)
          .await;
      }
      Err(e) => {
        error!("Failed to extract proposal info: {}", e);
      }
    }

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.voter);

    let proposals = self
      .cache
      .get::<Vec<Proposal>>("meta_gov:proposals")
      .await?
      .unwrap();

    let proposal = proposals
      .iter()
      .find(|&a| a.id == vote.proposal_id)
      .clone()
      .ok_or("Proposal not found in the funding list.")?;

    match self.extract_proposal_info(proposal.clone()).await {
      Ok((proposal_id, proposal_title)) => {
        let proposals_casts = self
          .cache
          .get::<HashMap<String, String>>("meta_gov:proposals:casts")
          .await?
          .unwrap_or_default();

        let empty_string = String::new();
        let cast_hash = proposals_casts.get(&proposal_id).unwrap_or(&empty_string);

        let wallet = get_wallet_handle(&vote.voter, "xyz.farcaster").await;

        let description = format!(
          "{} has voted {} “{}” proposal.",
          wallet,
          match vote.choice {
            1 => "for",
            2 => "against",
            3 => "abstain on",
            _ => "unknown",
          },
          proposal_title
        );

        let request_data = json!({
          "text": description,
          "channelKey": self.channel_key,
          "parent": {"hash": cast_hash},
        });

        self.make_http_request(request_data).await?;
      }
      Err(e) => {
        error!("Failed to extract proposal info: {}", e);
      }
    }

    Ok(())
  }
}
