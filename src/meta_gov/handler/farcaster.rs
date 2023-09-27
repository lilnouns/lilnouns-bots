use async_trait::async_trait;
use log::{debug, error};
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
};
use serde_json::Value;
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  meta_gov::{
    fetcher::{Proposal, Vote},
    handler::Handler,
  },
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

  pub fn from(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("META_GOV_BASE_URL")?.to_string();
    let bearer_token = env.secret("META_GOV_WARP_CAST_TOKEN")?.to_string();

    let cache = Cache::from(env);
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
