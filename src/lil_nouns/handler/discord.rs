use async_trait::async_trait;
use chrono::Utc;
use header::CONTENT_TYPE;
use log::{error, info};
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  lil_nouns::{handler::Handler, Proposal, Vote},
  utils::{ens::get_domain_name, get_explorer_address, get_short_address},
};

pub(crate) struct DiscordHandler {
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
    let base_url = env.var("LIL_NOUNS_BASE_URL")?.to_string();
    let webhook_url = env.secret("LIL_NOUNS_DISCORD_WEBHOOK_URL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(base_url, webhook_url, cache, client))
  }

  async fn execute_webhook(&self, embed: Value) -> Result<()> {
    let msg_json = json!({
      "username": "Raven",
      "avatar_url": "https://res.cloudinary.com/nekofar/image/upload/b_rgb:7BC4F2/ln_raven.jpg",
      "embeds": [embed]
    });

    self
      .client
      .post(&self.webhook_url)
      .header(CONTENT_TYPE, "application/json")
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
    let date = format!("<t:{}:R>", Utc::now().timestamp());
    let wallet = get_domain_name(&proposal.proposer)
      .await
      .unwrap_or(get_short_address(&proposal.proposer));
    let description = format!(
      "A new Lil Nouns proposal has been created: “{}”",
      proposal.title
    );
    let explorer = get_explorer_address(&proposal.proposer);

    let embed = json!({
        "title": "New Lil Nouns Proposal",
        "description": description,
        "url": url,
        "color": 0x7BC4F2,
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
    info!("Handling new vote from address: {}", vote.voter);

    let proposals = self
      .cache
      .get::<Vec<Proposal>>("lil_nouns:proposals")
      .await?
      .unwrap_or_default();

    let proposal = proposals
      .iter()
      .find(|&a| a.id == vote.proposal_id)
      .unwrap()
      .clone();

    let url = format!("{}/{}", self.base_url, proposal.id);
    let date = format!("<t:{}:R>", Utc::now().timestamp());
    let wallet = get_domain_name(&vote.voter)
      .await
      .unwrap_or(get_short_address(&vote.voter));

    let mut description = format!(
      "{} has voted {} “{}” proposal.",
      wallet,
      match vote.direction {
        0 => "against",
        1 => "for",
        2 => "abstain on",
        _ => "unknown",
      },
      proposal.title
    );

    if !vote.reason.is_none() {
      let chars_limit = 320 - 10 - description.len();
      let mut vote_reason = vote.clone().reason.unwrap_or_default();
      if vote_reason.len() > chars_limit {
        vote_reason.truncate(chars_limit);
        vote_reason.push_str("...");
      }
      description = format!("{}\n\n“{}”", description, vote_reason);
    }

    let explorer = get_explorer_address(&vote.voter);

    let embed = json!({
        "title": "New Lil Nouns Proposal Vote",
        "description": description,
        "url": url,
        "color": 0x7BC4F2,
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
