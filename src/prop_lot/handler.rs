use chrono::Local;
use log::{error, info};
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Result};

use crate::cache::Cache;
use crate::prop_lot::fetcher::{Comment, Idea, Vote};
use crate::utils::get_ens;

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
        let webhook_url = env.var("PROP_LOT_DISCORD_WEBHOOK_URL")?.to_string();

        let cache = Cache::from(env);
        let client = Client::new();

        Ok(Self::new(base_url, webhook_url, cache, client))
    }

    async fn execute_webhook(&self, embed: Value) -> Result<()> {
        let msg_json = json!({ "embeds": [embed] });

        self.client
            .post(&self.webhook_url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(msg_json.to_string())
            .send()
            .await
            .map_err(|e| {
                error!("Failed to execute webhook: {}", e);
                worker::Error::from(format!("Failed to execute webhook: {}", e))
            })?;

        Ok(())
    }

    pub(crate) async fn handle_new_idea(&self, idea: &Idea) -> Result<()> {
        info!("Handling new idea: {}", idea.title);
        let date = Local::now().format("%m/%d/%Y %I:%M %p");
        let ens_or_address = get_ens(&idea.creator_id).await.unwrap_or(format!(
            "{}...{}",
            &idea.creator_id[0..4],
            &idea.creator_id[38..42]
        ));

        let embed = json!({
            "title": "New Prop Lot Proposal",
            "description": format!(
                "A new Prop Lot proposal has been created: {}",
                idea.title
            ),
            "url": format!(
                "{}/idea/{}",
                self.base_url,
                idea.id
            ),
            "color": 0xFFB911,
            "footer": {
                "text": format!("{}", date)
            },
            "author": {
                "name": ens_or_address,
                "url": format!(
                    "{}/idea/{}",
                    self.base_url,
                    idea.id
                )
            }
        });

        self.execute_webhook(embed).await?;

        Ok(())
    }

    pub(crate) async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
        info!("Handling new vote from address: {}", vote.voter_id);
        let date = Local::now().format("%m/%d/%Y %I:%M %p");
        let ens_or_address = get_ens(&vote.voter_id).await.unwrap_or(format!(
            "{}...{}",
            &vote.voter_id[0..4],
            &vote.voter_id[38..42]
        ));

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

        let embed = json!({
            "title": "New Prop Lot Proposal Vote",
            "description": format!(
                "{} has voted {} Proposal ({})",
                ens_or_address,
                match vote.direction {
                    1 => "for",
                    _ => "against",
                },
                idea.title
            ),
            "url": format!(
                "{}/idea/{}",
                self.base_url,
                idea.id
            ),
            "color": 0xFFB911,
            "footer": {
                "text": format!("{}", date)
            },
            "author": {
                "name": ens_or_address,
                "url": format!(
                    "https://etherscan.io/address/{}",
                    vote.voter_id
                )
            }
        });

        self.execute_webhook(embed).await?;

        Ok(())
    }

    pub(crate) async fn handle_new_comment(&self, comment: &Comment) -> Result<()> {
        info!("Handling new comment from address: {}", comment.author_id);
        let date = Local::now().format("%m/%d/%Y %I:%M %p");
        let ens_or_address = get_ens(&comment.author_id).await.unwrap_or(format!(
            "{}...{}",
            &comment.author_id[0..4],
            &comment.author_id[38..42]
        ));

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

        let embed = json!({
            "title": "New Prop Lot Proposal Comment",
            "description": format!(
                "{} has commented on Proposal ({})",
                ens_or_address,
                idea.title
            ),
            "url": format!(
                "{}/idea/{}",
                self.base_url,
                idea.id
            ),
            "color": 0xFFB911,
            "footer": {
                "text": format!("{}", date)
            },
            "author": {
                "name": ens_or_address,
                "url": format!(
                    "https://etherscan.io/address/{}",
                    comment.author_id
                )
            }
        });

        self.execute_webhook(embed).await?;

        Ok(())
    }
}
