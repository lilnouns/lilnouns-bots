use std::env;

use anyhow::{Context, Result};
use log::{debug, error};
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;

use crate::prop_house::cacher::get_proposal_cache;
use crate::prop_house::fetcher::{Auction, Proposal, Vote};

pub(crate) async fn handle_new_auction(auction: &Auction) -> Result<()> {
    let base_url =
        env::var("PROP_HOUSE_BASE_URL").context("PROP_HOUSE_BASE_URL is not set in env")?;

    let url = env::var("PROP_HOUSE_DISCORD_WEBHOOK_URL")
        .context("PROP_HOUSE_DISCORD_WEBHOOK_URL is not set in env")?;

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, url.as_str())
        .await
        .context("Failed to create webhook from URL")?;

    let message = Embed::fake(|e| {
        e.title(format!("New Round: {}", auction.title))
            .url(format!(
                "{}/{}",
                base_url,
                auction.title.replace(' ', "-").to_lowercase()
            ))
            .description(&auction.description)
            .colour(0x8A2CE2)
    });

    webhook
        .execute(&http, false, |w| w.embeds(vec![message]))
        .await
        .context("Failed to execute webhook")?;

    Ok(())
}

pub(crate) async fn handle_new_proposal(proposal: &Proposal) -> Result<()> {
    debug!("New Proposal: {}", proposal.title);
    Ok(())
}

pub(crate) async fn handle_new_vote(vote: &Vote) -> Result<()> {
    if let Ok(Some(proposal)) = get_proposal_cache(vote.proposal_id) {
        debug!("New Vote on Proposal: {}", proposal.title)
    } else {
        error!("No proposal found for given id: {}", vote.proposal_id);
    }
    Ok(())
}
