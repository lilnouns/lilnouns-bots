use async_trait::async_trait;
use chrono::Local;
use log::{error, info};
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  prop_lot::{
    fetcher::{Comment, Idea, Vote},
    handler::Handler,
  },
  utils::{get_domain_name, get_explorer_address, get_short_address},
};

pub struct DiscordHandler {
  base_url: String,
  webhook_url: String,
  cache: Cache,
  client: Client,
}

impl DiscordHandler {
  pub fn new(base_url: String, webhook_url: String, cache: Cache, client: Client) -> Self {
    Self {
      base_url,
      webhook_url,
      cache,
      client,
    }
  }

  pub fn from(env: &Env) -> Result<DiscordHandler> {
    let base_url = env.var("PROP_LOT_BASE_URL")?.to_string();
    let webhook_url = env.secret("PROP_LOT_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::from(env);
    let client = Client::new();

    Ok(Self::new(base_url, webhook_url, cache, client))
  }

  async fn execute_webhook(&self, embed: Value) -> Result<()> {
    let msg_json = json!({ "embeds": [embed] });

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
  async fn handle_new_idea(&self, idea: &Idea) -> Result<()> {
    info!("Handling new idea: {}", idea.title);

    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let url = format!("{}/idea/{}", self.base_url, idea.id);
    let wallet = get_domain_name(&idea.creator_id)
      .await
      .unwrap_or(get_short_address(&idea.creator_id));
    let explorer = get_explorer_address(&idea.creator_id);
    let description = format!("A new Prop Lot proposal has been created: “{}”", idea.title);

    let embed = json!({
        "title": "New Prop Lot Proposal",
        "description": description,
        "url": url,
        "color": 0xFFB911,
        "footer": {"text": date},
        "author": {
            "name": wallet,
            "url": explorer,
        }
    });

    self.execute_webhook(embed).await?;

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.voter_id);

    let ideas = self
      .cache
      .get::<Vec<Idea>>("prop_lot:ideas")
      .await?
      .unwrap();

    let idea = ideas
      .iter()
      .find(|&a| a.id == vote.idea_id)
      .unwrap()
      .clone();

    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let wallet = get_domain_name(&vote.voter_id)
      .await
      .unwrap_or(get_short_address(&vote.voter_id));
    let explorer = get_explorer_address(&vote.voter_id);
    let url = format!("{}/idea/{}", self.base_url, idea.id);
    let description = format!(
      "{} has voted {} “{}” proposal.",
      wallet,
      match vote.direction {
        1 => "for",
        _ => "against",
      },
      idea.title
    );

    let embed = json!({
        "title": "New Prop Lot Proposal Vote",
        "description": description,
        "url": url,
        "color": 0xFFB911,
        "footer": {"text": date},
        "author": {
            "name": wallet,
            "url": explorer,
        }
    });

    self.execute_webhook(embed).await?;

    Ok(())
  }

  async fn handle_new_comment(&self, comment: &Comment) -> Result<()> {
    info!("Handling new comment from address: {}", comment.author_id);

    let ideas = self
      .cache
      .get::<Vec<Idea>>("prop_lot:ideas")
      .await?
      .unwrap();
    let idea = ideas
      .iter()
      .find(|&a| a.id == comment.idea_id)
      .unwrap()
      .clone();

    let url = format!("{}/idea/{}", self.base_url, idea.id);
    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let wallet = get_domain_name(&comment.author_id)
      .await
      .unwrap_or(get_short_address(&comment.author_id));
    let explorer = get_explorer_address(&comment.author_id);
    let description = format!("{} has commented on “{}” proposal.", wallet, idea.title);

    let embed = json!({
        "title": "New Prop Lot Proposal Comment",
        "description": description,
        "url": url,
        "color": 0xFFB911,
        "footer": {"text": date},
        "author": {
            "name": wallet,
            "url": explorer,
        }
    });

    self.execute_webhook(embed).await?;

    Ok(())
  }
}
