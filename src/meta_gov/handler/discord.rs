use async_trait::async_trait;
use chrono::Local;
use log::{error, info};
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::cache::Cache;
use crate::meta_gov::fetcher::{Proposal, Vote};
use crate::meta_gov::handler::Handler;
use crate::utils::{get_domain_name, get_explorer_address, get_short_address};

pub struct DiscordHandler {
  pub base_url: String,
  pub webhook_url: String,
  pub cache: Cache,
  pub client: Client,
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
    let base_url = env.var("META_GOV_BASE_URL")?.to_string();
    let webhook_url = env.secret("META_GOV_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::from(env);
    let client = Client::new();

    Ok(Self::new(base_url, webhook_url, cache, client))
  }

  async fn execute_webhook(&self, embed: Value) -> Result<()> {
    let msg_json = json!({"embeds": [embed]});

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
  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    info!("Handling new proposal: {}", proposal.title);

    let url = format!("{}/{}", self.base_url, proposal.id);
    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let description = format!("A new Meta Gov proposal has been created: “{}”", "");

    let embed = json!({
        "title": "New Meta Gov Proposal",
        "description": description,
        "url": url,
        "color": 0x8A2CE2,
        "footer": {"text": date}
    });

    self.execute_webhook(embed).await?;

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

    let url = format!(
      "{}/{}/{}",
      self.base_url,
      proposal.title.replace(' ', "-").to_lowercase(),
      proposal.id
    );
    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let wallet = get_domain_name(&vote.voter)
      .await
      .unwrap_or(get_short_address(&vote.voter));

    let description = format!(
      "{} has voted “{}” proposal.",
      wallet,
      match vote.choice {
        0 => "for",
        1 => "against",
        _ => "abstain on",
      }
    );
    let explorer = get_explorer_address(&vote.voter);

    let embed = json!({
        "title": "New Meta Gov Proposal Vote",
        "description": description,
        "url": url,
        "color": 0x8A2CE2,
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
