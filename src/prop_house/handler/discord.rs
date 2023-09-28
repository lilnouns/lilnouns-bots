use async_trait::async_trait;
use chrono::Local;
use log::{error, info};
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  prop_house::{
    fetcher::{Auction, Proposal, Vote},
    handler::Handler,
  },
  utils::{ens::get_domain_name, get_explorer_address, get_short_address},
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

  pub fn new_from_env(env: &Env) -> Result<DiscordHandler> {
    let base_url = env.var("PROP_HOUSE_BASE_URL")?.to_string();
    let webhook_url = env.secret("PROP_HOUSE_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::new_from_env(env);
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
  async fn handle_new_auction(&self, auction: &Auction) -> Result<()> {
    info!("Handling new auction: {}", auction.title);

    let url = format!(
      "{}/{}",
      self.base_url,
      auction.title.replace(' ', "-").to_lowercase()
    );
    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let description = format!(
      "A new Prop House round has been created: “{}”",
      auction.title
    );

    let embed = json!({
        "title": "New Prop House Round",
        "description": description,
        "url": url,
        "color": 0x8A2CE2,
        "footer": {"text": date}
    });

    self.execute_webhook(embed).await?;

    Ok(())
  }

  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    info!("Handling new proposal: {}", proposal.title);

    let auctions = self
      .cache
      .get::<Vec<Auction>>("prop_house:auctions")
      .await?
      .unwrap();

    let auction = auctions
      .iter()
      .find(|&a| a.id == proposal.auction_id)
      .unwrap()
      .clone();

    let url = format!(
      "{}/{}/{}",
      self.base_url,
      auction.title.replace(' ', "-").to_lowercase(),
      proposal.id
    );
    let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
    let wallet = get_domain_name(&proposal.address)
      .await
      .unwrap_or(get_short_address(&proposal.address));
    let description = format!(
      "A new Prop House proposal has been created: “{}”",
      proposal.title
    );
    let explorer = get_explorer_address(&proposal.address);

    let embed = json!({
        "title": "New Prop House Proposal",
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

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.address);

    let proposals = self
      .cache
      .get::<Vec<Proposal>>("prop_house:proposals")
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
    let wallet = get_domain_name(&vote.address)
      .await
      .unwrap_or(get_short_address(&vote.address));

    let description = format!(
      "{} has voted “{}” proposal.",
      wallet,
      match vote.direction {
        1 => "for",
        _ => "against",
      }
    );
    let explorer = get_explorer_address(&vote.address);

    let embed = json!({
        "title": "New Prop House Proposal Vote",
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
