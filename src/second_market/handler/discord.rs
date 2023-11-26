use async_trait::async_trait;
use chrono::Local;
use log::{error, info};
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  second_market::{handler::Handler, Floor},
};

pub(crate) struct DiscordHandler {
  webhook_url: String,
  collection: String,
  cache: Cache,
  client: Client,
}

impl DiscordHandler {
  pub fn new(webhook_url: String, collection: String, cache: Cache, client: Client) -> Self {
    Self {
      webhook_url,
      collection,
      cache,
      client,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<Self> {
    let webhook_url = env.secret("SECOND_MARKET_DISCORD_WEBHOOK_URL")?.to_string();
    let collection = env.var("SECOND_MARKET_COLLECTION_ADDRESS")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(webhook_url, collection, cache, client))
  }

  async fn execute_webhook(&self, embed: Value) -> Result<()> {
    let msg_json = json!({
      "username": "Raven",
      "avatar_url": "https://res.cloudinary.com/nekofar/image/upload/b_rgb:039BE5/ln_raven.jpg",
      "embeds": [embed]
    });

    self
      .client
      .post(&self.webhook_url)
      .header(header::CONTENT_TYPE, "application/json")
      .body(msg_json.to_string())
      .send()
      .await
      .map_err(|e| {
        error!("Failed to execute webhook: {}", e);
        Error::from(format!("Failed to execute webhook: {}", e))
      })?;

    Ok(())
  }
}

#[async_trait(? Send)]
impl Handler for DiscordHandler {
  async fn handle_new_floor(&self, floor: &Floor) -> Result<()> {
    info!("Handling new floor: {:?}", floor.price);

    let old_price = self
      .cache
      .get::<f64>("second_market:old_price")
      .await?
      .unwrap_or_default();
    let new_price = floor.price.unwrap_or_default();

    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let url = match floor.clone().source.unwrap_or_else(String::new).as_str() {
      "blur.io" => format!("https://blur.io/collection/{}", self.collection),
      _ => format!("https://opensea.io/assets/ethereum/{}", self.collection),
    };

    let description = format!(
      "There has been a change in the floor price on the second market. The new floor price is \
       now **{}** Ξ, while the previous was **{}** Ξ.",
      new_price, old_price
    );

    let embed = json!({
      "title": "New Second Market Floor",
      "description": description,
      "url": url,
      "color": 0x039BE5,
      "footer": {"text": date}
    });

    self.execute_webhook(embed).await?;

    Ok(())
  }
}
