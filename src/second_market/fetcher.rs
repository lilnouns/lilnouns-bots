use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use worker::{Env, Result};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
  pub collections: Vec<Collection>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
  pub chain_id: u64,
  pub id: String,
  pub slug: String,
  pub created_at: String,
  pub updated_at: String,
  pub name: String,
  pub symbol: String,
  pub contract_deployed_at: Option<String>,
  pub image: String,
  pub banner: Option<String>,
  pub twitter_url: Option<String>,
  pub discord_url: Option<String>,
  pub external_url: String,
  pub twitter_username: Option<String>,
  pub opensea_verification_status: String,
  pub description: String,
  pub metadata_disabled: bool,
  pub is_spam: bool,
  pub sample_images: Vec<String>,
  pub token_count: String,
  pub on_sale_count: String,
  pub primary_contract: String,
  pub token_set_id: String,
  pub creator: String,
  pub royalties: Royalties,
  pub all_royalties: AllRoyalties,
  pub floor_ask: FloorAsk,
  pub top_bid: TopBid,
  pub rank: HashMap<String, Option<u64>>,
  pub volume: HashMap<String, f64>,
  pub volume_change: HashMap<String, f64>,
  pub floor_sale: HashMap<String, f64>,
  pub floor_sale_change: HashMap<String, f64>,
  pub collection_bid_supported: bool,
  pub owner_count: u64,
  pub contract_kind: String,
  pub minted_timestamp: u64,
  pub mint_stages: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Royalties {
  pub recipient: Option<String>,
  pub breakdown: Vec<String>,
  pub bps: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllRoyalties {
  pub opensea: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FloorAsk {
  pub id: String,
  pub source_domain: String,
  pub price: Price,
  pub maker: String,
  pub valid_from: u64,
  pub valid_until: u64,
  pub token: Token,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopBid {
  pub id: String,
  pub source_domain: String,
  pub price: Price,
  pub maker: String,
  pub valid_from: u64,
  pub valid_until: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
  pub currency: Currency,
  pub amount: Amount,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
  pub contract: String,
  pub name: String,
  pub symbol: String,
  pub decimals: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
  pub raw: String,
  pub decimal: f64,
  pub usd: f64,
  pub native: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
  pub contract: String,
  pub token_id: String,
  pub name: String,
  pub image: String,
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

  pub async fn fetch_collections(&self) -> Option<Vec<Collection>> {
    let endpoint = format!("{}/collections/v7?id={}", self.base_url, self.collection);

    let client = Client::new();

    let response = client
      .get(endpoint)
      .header("X-Api-Key", &self.api_key)
      .send()
      .await
      .unwrap();

    let collections = response
      .json::<Root>()
      .await
      .unwrap_or_default()
      .collections;

    Some(collections)
  }
}
