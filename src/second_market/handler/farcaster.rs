use async_trait::async_trait;
use chrono::{DateTime, Utc};
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
  second_market::{fetcher::Collection, handler::Handler},
};

pub(crate) struct FarcasterHandler {
  warpcast_url: String,
  warpcast_bearer_token: String,
  warpcast_channel_key: String,
  cache: Cache,
  client: Client,
}

impl FarcasterHandler {
  pub fn new(
    warpcast_url: String,
    warpcast_bearer_token: String,
    warpcast_channel_key: String,
    cache: Cache,
    client: Client,
  ) -> Self {
    Self {
      warpcast_url,
      warpcast_bearer_token,
      warpcast_channel_key,
      cache,
      client,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<Self> {
    let warpcast_url = env.var("WARPCAST_API_BASE_URL")?.to_string();
    let warpcast_bearer_token = env.secret("SECOND_MARKET_WARPCAST_TOKEN")?.to_string();
    let warpcast_channel_key = env.var("SECOND_MARKET_WARPCAST_CHANNEL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(
      warpcast_url,
      warpcast_bearer_token,
      warpcast_channel_key,
      cache,
      client,
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
  async fn handle_new_floor(&self, collection: &Collection) -> Result<()> {
    info!(
      "Handling new floor: {}",
      collection.floor_ask.price.amount.decimal
    );
    let now: DateTime<Utc> = Utc::now();

    let old_price = self
      .cache
      .get::<f64>("second_market:old_price")
      .await?
      .unwrap_or_default();
    let new_price = collection.floor_ask.price.amount.decimal;

    let mut url = format!("https://blur.io/eth/collection/{}", collection.slug);
    url = format!("{}?{}", url, now.timestamp());

    let description = format!(
      "There has been a change in the floor price on the second market. The new floor price is \
       now {} Ξ, while the previous was {} Ξ.",
      new_price, old_price
    );

    let request_data = json!({
      "text": description,
      "embeds": [url],
      "channelKey": self.warpcast_channel_key,
      "castDistribution": "channel-only"
    });

    self.make_http_request(request_data).await.map_err(|e| {
      error!("Failed to make HTTP request: {}", e);
      return e;
    })?;

    Ok(())
  }
}
