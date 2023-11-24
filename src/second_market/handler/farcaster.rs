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
  second_market::{handler::Handler, Floor},
};

pub(crate) struct FarcasterHandler {
  warpcast_url: String,
  bearer_token: String,
  channel_key: String,
  collection: String,
  cache: Cache,
  client: Client,
}

impl FarcasterHandler {
  pub fn new(
    warpcast_url: String,
    bearer_token: String,
    channel_key: String,
    collection: String,
    cache: Cache,
    client: Client,
  ) -> Self {
    Self {
      warpcast_url,
      bearer_token,
      channel_key,
      collection,
      cache,
      client,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<Self> {
    let warpcast_url = env.var("WARP_CAST_API_BASE_URL")?.to_string();
    let bearer_token = env.secret("SECOND_MARKET_WARP_CAST_TOKEN")?.to_string();
    let channel_key = env.var("SECOND_MARKET_WARP_CAST_CHANNEL")?.to_string();
    let collection = env.var("SECOND_MARKET_COLLECTION_ADDRESS")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(
      warpcast_url,
      bearer_token,
      channel_key,
      collection,
      cache,
      client,
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
  async fn handle_new_floor(&self, floor: &Floor) -> Result<()> {
    info!("Handling new floor: {}", floor.id);

    let url = match floor.clone().source.unwrap_or_else(String::new).as_str() {
      "blur.io" => format!("https://blur.io/collection/{}", self.collection),
      _ => format!("https://opensea.io/assets/ethereum/{}", self.collection),
    };

    let description = format!(
      "There has been a change in the floor price on the second market. The new floor price is \
       now {} Ξ, while the previous was {} Ξ.",
      floor.new_price.unwrap().to_string(),
      floor.old_price.unwrap().to_string()
    );

    let request_data = json!({
      "text": description,
      "embeds": [url],
      "channelKey": self.channel_key
    });

    self.make_http_request(request_data).await.map_err(|e| {
      error!("Failed to make HTTP request: {}", e);
      return e;
    })?;

    Ok(())
  }
}
