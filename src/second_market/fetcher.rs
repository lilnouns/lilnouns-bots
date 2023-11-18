use reqwest::Client;
use serde::Deserialize;
use worker::{Env, Result};

use crate::second_market::Floor;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Event {
  collection: Collection,
  floor_ask: FloorAsk,
  event: EventDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Collection {
  id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FloorAsk {
  order_id: String,
  contract: String,
  token_id: String,
  maker: String,
  price: Price,
  valid_until: i64,
  source: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Price {
  currency: Currency,
  amount: Amount,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Currency {
  contract: String,
  name: String,
  symbol: String,
  decimals: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Amount {
  raw: String,
  decimal: f64,
  usd: f64,
  native: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventDetail {
  id: String,
  previous_price: f64,
  kind: String,
  tx_hash: Option<String>,
  tx_timestamp: Option<i64>,
  created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
  events: Vec<Event>,
  continuation: String,
}

pub struct RestFetcher {
  api_key: String,
  base_url: String,
  collection: String,
}

impl RestFetcher {
  pub fn new(api_key: String, base_url: String, collection: String) -> Self {
    Self {
      api_key,
      base_url,
      collection,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<RestFetcher> {
    let api_key = env.var("SECOND_MARKET_API_KEY")?.to_string();
    let base_url = env.var("SECOND_MARKET_API_BASE_URL")?.to_string();
    let collection = env.var("SECOND_MARKET_COLLECTION_ADDRESS")?.to_string();

    Ok(Self::new(api_key, base_url, collection))
  }

  pub async fn fetch_floors(&self) -> Option<Vec<Floor>> {
    let endpoint = format!(
      "{}/events/collections/floor-ask/v2?collection={}",
      self.base_url, self.collection
    );

    let client = Client::new();

    let response = client
      .get(endpoint)
      .header("X-Api-Key", &self.api_key)
      .send()
      .await
      .unwrap();

    let floors = response
      .json::<ApiResponse>()
      .await
      .unwrap()
      .events
      .iter()
      .map(|event| Floor {
        id: event.event.id.clone(),
        price: event.floor_ask.price.amount.decimal,
        source: event.floor_ask.source.clone(),
        created_at: event.event.created_at.clone(),
        previous_price: event.event.previous_price,
      })
      .collect();

    Some(floors)
  }
}
