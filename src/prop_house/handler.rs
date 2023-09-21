use chrono::Local;
use reqwest::{header, Client};
use serde_json::{json, Value};
use worker::{Env, Result};

use crate::cache::Cache;
use crate::prop_house::fetcher::{Auction, Proposal, Vote};

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
        let base_url = env.var("PROP_HOUSE_BASE_URL")?.to_string();
        let webhook_url = env.var("PROP_HOUSE_DISCORD_WEBHOOK_URL")?.to_string();

        let cache = Cache::from(env);
        let client = Client::new();

        Ok(Self::new(base_url, webhook_url, cache, client))
    }

    async fn execute_webhook(&self, embed: Value) -> Result<()> {
        let msg_json = json!({"embeds": [embed]});

        self.client
            .post(&self.webhook_url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(msg_json.to_string())
            .send()
            .await
            .map_err(|e| worker::Error::from(format!("Failed to execute webhook: {}", e)))?;

        Ok(())
    }

    pub(crate) async fn handle_new_auction(&self, auction: &Auction) -> Result<()> {
        let date = Local::now().format("%m/%d/%Y %I:%M %p");

        let embed = json!({
            "title": "New Prop House Round",
            "description": format!(
                "A new Prop House round has been created: {}",
                auction.title
            ),
            "url": format!("{}/{}", self.base_url, auction.title.replace(' ', "-").to_lowercase()),
            "color": 0x8A2CE2,
            "footer": {
                "text": format!("{}", date)
            }
        });

        self.execute_webhook(embed).await?;

        Ok(())
    }

    pub(crate) async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
        let date = Local::now().format("%m/%d/%Y %I:%M %p");

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

        let embed = json!({
            "title": "New Prop House Proposal",
            "description": format!(
                "A new Prop House proposal has been created: {}",
                proposal.title
            ),
            "url": format!(
                "{}/{}/{}",
                self.base_url,
                auction.title.replace(' ', "-").to_lowercase(),
                proposal.id
            ),
            "color": 0x8A2CE2,
            "footer": {
                "text": format!("{}", date)
            },
            "author": {
                "name": format!(
                    "{}...{}",
                    &proposal.address[0..4],
                    &proposal.address[38..42]
                ),
                "url": format!(
                    "https://etherscan.io/address/{}",
                    proposal.address
                )
            }
        });

        self.execute_webhook(embed).await?;

        Ok(())
    }

    pub(crate) async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
        let date = Local::now().format("%m/%d/%Y %I:%M %p");

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

        let embed = json!({
            "title": "New Prop House Proposal Vote",
            "description": format!(
                "{} has voted {} Proposal",
                format!(
                    "{}...{}",
                    &vote.address[0..4],
                    &vote.address[38..42]
                ),
                match vote.direction {
                    1 => "for",
                    _ => "against"
                }
            ),
            "url": format!(
                "{}/{}/{}",
                self.base_url,
                proposal.title.replace(' ', "-").to_lowercase(),
                proposal.id
            ),
            "color": 0x8A2CE2,
            "footer": {
                "text": format!("{}", date)
            },
            "author": {
                "name": format!(
                    "{}...{}",
                    &vote.address[0..4],
                    &vote.address[38..42]
                ),
                "url": format!(
                    "https://etherscan.io/address/{}",
                    vote.address
                )
            }
        });

        self.execute_webhook(embed).await?;

        Ok(())
    }
}
