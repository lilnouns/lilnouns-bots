use reqwest::Client;
use serde::Deserialize;
use worker::{Env, Result};

use crate::second_market::Floor;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FloorAskResponse {
  events: Vec<Event>,
  continuation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Event {
  collection: Collection,
  floor_ask: FloorAsk,
  event: EventDetail,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Collection {
  id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FloorAsk {
  order_id: Option<String>,
  contract: Option<String>,
  token_id: Option<String>,
  maker: Option<String>,
  price: Option<Price>,
  valid_until: Option<i64>,
  source: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Price {
  currency: Currency,
  amount: Amount,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Currency {
  contract: String,
  name: String,
  symbol: String,
  decimals: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Amount {
  raw: String,
  decimal: f64,
  usd: f64,
  native: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventDetail {
  id: String,
  previous_price: Option<f64>,
  kind: String,
  tx_hash: Option<String>,
  tx_timestamp: Option<i64>,
  created_at: String,
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
      .json::<FloorAskResponse>()
      .await
      .unwrap()
      .events
      .iter()
      .map(|event| Floor {
        id: event.event.id.clone(),
        kind: event.event.kind.clone(),
        source: event.floor_ask.source.clone(),
        created_at: event.event.created_at.clone(),
        price: Some(
          event
            .floor_ask
            .price
            .as_ref()
            .map(|p| p.amount.decimal)
            .unwrap_or(0.0),
        ),
      })
      .filter(|floor| floor.price.unwrap_or(0.0) != 0.0)
      .collect();

    Some(floors)
  }
}
