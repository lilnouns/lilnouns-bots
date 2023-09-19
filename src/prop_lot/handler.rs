use std::env;

use anyhow::{Context, Result};
use chrono::Local;
use serenity::json::Value;
use serenity::{
    http::Http,
    model::{channel::Embed, webhook::Webhook},
};

use crate::prop_lot::cacher::{get_idea_cache, set_comment_cache, set_idea_cache, set_vote_cache};
use crate::prop_lot::fetcher::{Comment, Idea, Vote};

pub struct DiscordHandler {
    base_url: String,
    http: Http,
    webhook: Webhook,
}

impl DiscordHandler {
    pub async fn new() -> Result<Self> {
        let base_url =
            env::var("PROP_LOT_BASE_URL").context("PROP_LOT_BASE_URL is not set in env")?;

        let webhook_url = env::var("PROP_LOT_DISCORD_WEBHOOK_URL")
            .context("PROP_LOT_DISCORD_WEBHOOK_URL is not set in env")?;

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

    pub(crate) async fn handle_new_idea(&self, idea: &Idea) -> Result<()> {
        let message = Embed::fake(|e| {
            e.author(|a| {
                a.name(format!(
                    "{}...{}",
                    &idea.creator_id[0..4],
                    &idea.creator_id[38..42]
                ))
                .url(format!("https://etherscan.io/address/{}", idea.creator_id))
            })
            .title("New Prop Lot Proposal")
            .url(format!("{}/idea/{}", self.base_url, idea.id))
            .description(format!(
                "A new Prop Lot proposal has been created: {}",
                idea.title
            ))
            .footer(|f| f.text(format!("{}", Local::now().format("%m/%d/%Y %I:%M %p"))))
            .colour(0xFFB911)
        });

        self.execute_webhook(message).await?;

        set_idea_cache(idea)?;

        Ok(())
    }

    pub(crate) async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
        let idea = get_idea_cache(vote.idea_id)?
            .ok_or_else(|| anyhow::anyhow!("No idea found for id {}", vote.idea_id))?;

        let message = Embed::fake(|e| {
            e.author(|a| {
                a.name(format!(
                    "{}...{}",
                    &vote.voter_id[0..4],
                    &vote.voter_id[38..42]
                ))
                .url(format!("https://etherscan.io/address/{}", vote.voter_id))
            })
            .title("New Prop Lot Proposal Vote")
            .url(format!("{}/idea/{}", self.base_url, idea.id))
            .description(format!(
                "{} has voted {} Proposal ({})",
                format!("{}...{}", &vote.voter_id[0..4], &vote.voter_id[38..42]),
                match vote.direction {
                    1 => "for",
                    _ => "against",
                },
                idea.title
            ))
            .footer(|f| f.text(format!("{}", Local::now().format("%m/%d/%Y %I:%M %p"))))
            .colour(0x8A2CE2)
        });

        self.execute_webhook(message).await?;

        set_vote_cache(vote)?;

        Ok(())
    }

    pub(crate) async fn handle_new_comment(&self, comment: &Comment) -> Result<()> {
        let idea = get_idea_cache(comment.idea_id)?
            .ok_or_else(|| anyhow::anyhow!("No idea found for id {}", comment.idea_id))?;

        let message = Embed::fake(|e| {
            e.author(|a| {
                a.name(format!(
                    "{}...{}",
                    &comment.author_id[0..4],
                    &comment.author_id[38..42]
                ))
                .url(format!(
                    "https://etherscan.io/address/{}",
                    comment.author_id
                ))
            })
            .title("New Prop Lot Proposal Comment")
            .url(format!("{}/idea/{}", self.base_url, idea.id))
            .description(format!(
                "{} has commented on Proposal ({})",
                format!(
                    "{}...{}",
                    &comment.author_id[0..4],
                    &comment.author_id[38..42]
                ),
                idea.title
            ))
            .footer(|f| f.text(format!("{}", Local::now().format("%m/%d/%Y %I:%M %p"))))
            .colour(0x8A2CE2)
        });

        self.execute_webhook(message).await?;

        set_comment_cache(comment)?;

        Ok(())
    }
}
