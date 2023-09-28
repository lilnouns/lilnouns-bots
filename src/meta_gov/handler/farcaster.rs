use async_trait::async_trait;
use log::{debug, error, info};
use regex::Regex;
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  meta_gov::{
    fetcher::{Proposal, Vote},
    handler::Handler,
  },
  utils::ens::get_wallet_handle,
};

pub struct FarcasterHandler {
  base_url: String,
  bearer_token: String,
  cache: Cache,
  client: Client,
}

impl FarcasterHandler {
  pub fn new(base_url: String, bearer_token: String, cache: Cache, client: Client) -> Self {
    Self {
      base_url,
      bearer_token,
      cache,
      client,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("META_GOV_BASE_URL")?.to_string();
    let bearer_token = env.secret("META_GOV_WARP_CAST_TOKEN")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(base_url, bearer_token, cache, client))
  }

  async fn make_http_request(&self, request_data: Value) -> Result<()> {
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

    Ok(())
  }

  async fn extract_proposal_info(&self, proposal: Proposal) -> Result<(u32, String)> {
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
    let proposal_id = proposal_id
      .as_str()
      .parse::<u32>()
      .map_err(|_| Error::from("Failed to parse proposal ID"))?;
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

        let url = format!("{}/{}", self.base_url, proposal_id);
        let description = format!(
          "A new Meta Gov proposal has been created: “{}”",
          proposal_title
        );

        let request_data = json!({
            "text": description,
            "embeds": [url],
            "channelKey": "lil-nouns"
        });

        self.make_http_request(request_data).await?;
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
      .unwrap()
      .clone();

    match self.extract_proposal_info(proposal.clone()).await {
      Ok((proposal_id, proposal_title)) => {
        let url = format!("{}/{}", self.base_url, proposal_id);

        let wallet = get_wallet_handle(&vote.voter, "xyz.farcaster").await;

        let description = format!(
          "“{}” voted {} by {}.",
          proposal_title.to_uppercase(),
          match vote.choice {
            1 => "for",
            2 => "against",
            3 => "abstain on",
            _ => "unknown",
          },
          wallet,
        );

        let request_data = json!({
            "text": description,
            "embeds": [url],
            "channelKey": "lil-nouns"
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
