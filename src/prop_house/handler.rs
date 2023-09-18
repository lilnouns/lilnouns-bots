use std::env;

use anyhow::{Context, Result};
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;

use crate::prop_house::cacher::{
    get_auction_cache, get_proposal_cache, set_auction_cache, set_proposal_cache, set_vote_cache,
};
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

    set_auction_cache(auction).unwrap();

    Ok(())
}

pub(crate) async fn handle_new_proposal(proposal: &Proposal) -> Result<()> {
    let auction = get_auction_cache(proposal.auction_id)?
        .ok_or_else(|| anyhow::anyhow!("No auction found for id {}", proposal.auction_id))?;

    let base_url =
        env::var("PROP_HOUSE_BASE_URL").context("PROP_HOUSE_BASE_URL is not set in env")?;

    let url = env::var("PROP_HOUSE_DISCORD_WEBHOOK_URL")
        .context("PROP_HOUSE_DISCORD_WEBHOOK_URL is not set in env")?;

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, url.as_str())
        .await
        .context("Failed to create webhook from URL")?;

    let message = Embed::fake(|e| {
        e.author(|a| {
            a.name(format!(
                "{}...{}",
                &proposal.address[0..4],
                &proposal.address[38..42]
            ))
        })
        .title(format!("New Proposal: {}", proposal.title))
        .url(format!(
            "{}/{}/{}",
            base_url,
            auction.title.replace(' ', "-").to_lowercase(),
            proposal.id
        ))
        .description(&proposal.tldr)
        .colour(0x8A2CE2)
    });

    webhook
        .execute(&http, false, |w| w.embeds(vec![message]))
        .await
        .context("Failed to execute webhook")?;

    set_proposal_cache(proposal)?;

    Ok(())
}

pub(crate) async fn handle_new_vote(vote: &Vote) -> Result<()> {
    let proposal = get_proposal_cache(vote.proposal_id)?
        .ok_or_else(|| anyhow::anyhow!("No proposal found for id {}", vote.proposal_id))?;

    let base_url =
        env::var("PROP_HOUSE_BASE_URL").context("PROP_HOUSE_BASE_URL is not set in env")?;

    let url = env::var("PROP_HOUSE_DISCORD_WEBHOOK_URL")
        .context("PROP_HOUSE_DISCORD_WEBHOOK_URL is not set in env")?;

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, url.as_str())
        .await
        .context("Failed to create webhook from URL")?;

    let message = Embed::fake(|e| {
        e.author(|a| {
            a.name(format!(
                "{}...{}",
                &vote.address[0..4],
                &vote.address[38..42]
            ))
        })
        .title(format!(
            "New Vote {}: {}",
            match vote.direction {
                1 => "For",
                _ => "Against",
            },
            proposal.title
        ))
        .url(format!(
            "{}/{}/{}",
            base_url,
            proposal.title.replace(' ', "-").to_lowercase(),
            proposal.id
        ))
        .description(&proposal.tldr)
        .colour(0x8A2CE2)
    });

    webhook
        .execute(&http, false, |w| w.embeds(vec![message]))
        .await
        .context("Failed to execute webhook")?;

    set_vote_cache(vote).unwrap();

    Ok(())
}
