use async_trait::async_trait;
use chrono::Local;
use log::{error, info};
use regex::Regex;
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  meta_gov::{handler::Handler, Proposal, Vote},
  utils::{ens::get_domain_name, get_explorer_address, get_short_address},
};

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

  pub fn new_from_env(env: &Env) -> Result<DiscordHandler> {
    let base_url = env.var("META_GOV_BASE_URL")?.to_string();
    let webhook_url = env.secret("META_GOV_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(base_url, webhook_url, cache, client))
  }

  async fn execute_webhook(&self, embed: Value) -> Result<()> {
    let msg_json = json!({
      "username": "Meta Gov",
      "avatar_url": "https://i.imgur.com/zdMjAeD.png",
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

  async fn extract_proposal_info(&self, proposal: Proposal) -> Result<(u32, String)> {
    let captures = Regex::new(r"(\d+): (.+)")
      .unwrap()
      .captures(&*proposal.title)
      .ok_or(Error::from("Capture Failed"))?;
    let proposal_id = captures
      .get(1)
      .ok_or(Error::from("Failed to get proposal ID"))?;
    let proposal_title = captures
      .get(2)
      .ok_or(Error::from("Failed to get proposal Title"))?;
    let proposal_id = proposal_id
      .as_str()
      .parse::<u32>()
      .map_err(|_| Error::from("Failed to parse proposal ID"))?;
    let proposal_title = proposal_title.as_str().to_string();

    Ok((proposal_id, proposal_title))
  }
}

#[async_trait(? Send)]
impl Handler for DiscordHandler {
  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    match self.extract_proposal_info(proposal.clone()).await {
      Ok((proposal_id, proposal_title)) => {
        info!("Handling new proposal: {}", proposal_title);

        let url = format!("{}/{}", self.base_url, proposal_id);
        let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
        let description = format!(
          "A new Meta Gov proposal has been created: “{}”",
          proposal_title
        );

        let embed = json!({
            "title": "New Meta Gov Proposal",
            "description": description,
            "url": url,
            "color": 0xE40536,
            "footer": {"text": date}
        });

        self.execute_webhook(embed).await?;
      }
      Err(e) => {
        error!("Failed to extract proposal info: {}", e);
      }
    }

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

    match self.extract_proposal_info(proposal.clone()).await {
      Ok((proposal_id, proposal_title)) => {
        info!("Handling new proposal: {}", proposal_title);

        let url = format!("{}/{}", self.base_url, proposal_id);
        let date = Local::now().format("%m/%d/%Y %I:%M %p").to_string();
        let wallet = get_domain_name(&vote.voter)
          .await
          .unwrap_or(get_short_address(&vote.voter));

        let description = format!(
          "{} has voted {} “{}” proposal.",
          wallet,
          match vote.choice {
            1 => "for",
            2 => "against",
            3 => "abstain on",
            _ => "unknown",
          },
          proposal_title
        );
        let explorer = get_explorer_address(&vote.voter);

        let embed = json!({
            "title": "New Meta Gov Proposal Vote",
            "description": description,
            "url": url,
            "color": 0xE40536,
            "footer": {"text": date},
            "author": {
                "name": wallet,
                "url": explorer,
            }
        });

        self.execute_webhook(embed).await?;
      }
      Err(e) => {
        error!("Failed to extract proposal info: {}", e);
      }
    }
    Ok(())
  }
}
