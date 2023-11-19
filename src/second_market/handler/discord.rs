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
  cache: Cache,
  client: Client,
}

impl DiscordHandler {
  pub fn new(webhook_url: String, cache: Cache, client: Client) -> Self {
    Self {
      webhook_url,
      cache,
      client,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<Self> {
    let webhook_url = env.secret("SECOND_MARKET_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(webhook_url, cache, client))
  }

  async fn execute_webhook(&self, embed: Value) -> Result<()> {
    let msg_json = json!({
      "username": "Raven",
      "avatar_url": "https://i.imgur.com/qP2QpJq.png",
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
    info!("Handling new floor: {:?}", floor.new_price);

    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let floor_source = if let Some(source) = floor.clone().source {
      source
    } else {
      String::new() // or any other default value
    };

    let url;
    if floor_source == "blur.io" {
      url = "https://blur.io/collection/lil-nouns";
    } else {
      url = "https://opensea.io/collection/lil-nouns";
    }

    let description = format!(
      "There has been a change in the floor price on the second market. The new floor price is \
       new floor price is now {} Ξ, while the previous was {} Ξ.",
      floor.new_price.unwrap().to_string(),
      floor.old_price.unwrap().to_string()
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
