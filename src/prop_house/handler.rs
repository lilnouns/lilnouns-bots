use std::env;

use anyhow::{Context, Result};
use chrono::Local;
use serenity::http::Http;
use serenity::json::Value;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;

use crate::prop_house::cacher::{
    get_auction_cache, get_proposal_cache, set_auction_cache, set_proposal_cache, set_vote_cache,
};
use crate::prop_house::fetcher::{Auction, Proposal, Vote};

pub struct DiscordHandler {
    base_url: String,
    http: Http,
    webhook: Webhook,
}

impl DiscordHandler {
    pub async fn new() -> Result<Self> {
        let base_url =
            env::var("PROP_HOUSE_BASE_URL").context("PROP_HOUSE_BASE_URL is not set in env")?;

        let webhook_url = env::var("PROP_HOUSE_DISCORD_WEBHOOK_URL")
            .context("PROP_HOUSE_DISCORD_WEBHOOK_URL is not set in env")?;

        let http = Http::new("");
        let webhook = Webhook::from_url(&http, webhook_url.as_str())
            .await
            .context("Failed to create webhook from URL")?;

        Ok(Self {
            base_url,
            http,
            webhook,
        })
    }

    async fn execute_webhook(&self, message: Value) -> Result<()> {
        self.webhook
            .execute(&self.http, false, |w| w.embeds(vec![message]))
            .await
            .context("Failed to execute webhook")?;

        Ok(())
    }

    pub(crate) async fn handle_new_auction(&self, auction: &Auction) -> Result<()> {
        let message = Embed::fake(|e| {
            e.title("New Prop House Round")
                .url(format!(
                    "{}/{}",
                    self.base_url,
                    auction.title.replace(' ', "-").to_lowercase()
                ))
                .description(format!(
                    "A new Prop House round has been created: {}",
                    auction.title
                ))
                .footer(|f| f.text(format!("{}", Local::now().format("%m/%d/%Y %I:%M %p"))))
                .colour(0x8A2CE2)
        });

        self.execute_webhook(message).await?;

        set_auction_cache(auction)?;

        Ok(())
    }

    pub(crate) async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
        let auction = get_auction_cache(proposal.auction_id)?
            .ok_or_else(|| anyhow::anyhow!("No auction found for id {}", proposal.auction_id))?;

        let message = Embed::fake(|e| {
            e.author(|a| {
                a.name(format!(
                    "{}...{}",
                    &proposal.address[0..4],
                    &proposal.address[38..42]
                ))
                .url(format!("https://etherscan.io/address/{}", proposal.address))
            })
            .title("New Prop House Proposal")
            .url(format!(
                "{}/{}/{}",
                self.base_url,
                auction.title.replace(' ', "-").to_lowercase(),
                proposal.id
            ))
            .description(format!(
                "A new Prop House proposal has been created: {}",
                proposal.title
            ))
            .footer(|f| f.text(format!("{}", Local::now().format("%m/%d/%Y %I:%M %p"))))
            .colour(0x8A2CE2)
        });

        self.execute_webhook(message).await?;

        set_proposal_cache(proposal)?;

        Ok(())
    }

    pub(crate) async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
        let proposal = get_proposal_cache(vote.proposal_id)?
            .ok_or_else(|| anyhow::anyhow!("No proposal found for id {}", vote.proposal_id))?;

        let message = Embed::fake(|e| {
            e.author(|a| {
                a.name(format!(
                    "{}...{}",
                    &vote.address[0..4],
                    &vote.address[38..42]
                ))
                .url(format!("https://etherscan.io/address/{}", vote.address))
            })
            .title("New Prop House Proposal Vote")
            .url(format!(
                "{}/{}/{}",
                self.base_url,
                proposal.title.replace(' ', "-").to_lowercase(),
                proposal.id
            ))
            .description(format!(
                "{} has voted {} Proposal ({})",
                format!("{}...{}", &vote.address[0..4], &vote.address[38..42]),
                match vote.direction {
                    1 => "for",
                    _ => "against",
                },
                proposal.title
            ))
            .footer(|f| f.text(format!("{}", Local::now().format("%m/%d/%Y %I:%M %p"))))
            .colour(0x8A2CE2)
        });

        self.execute_webhook(message).await?;

        set_vote_cache(vote)?;

        Ok(())
    }
}
