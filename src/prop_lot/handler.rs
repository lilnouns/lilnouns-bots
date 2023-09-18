use std::env;

use anyhow::Context;
use anyhow::Result;
use log::{debug, error};
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;

use crate::prop_lot::cacher::{get_idea_cache, set_comment_cache, set_idea_cache, set_vote_cache};
use crate::prop_lot::fetcher::{Comment, Idea, Vote};

pub(crate) async fn handle_new_idea(idea: &Idea) -> Result<()> {
    let base_url = env::var("PROP_LOT_BASE_URL").context("PROP_LOT_BASE_URL is not set in env")?;

    let webhook_url = env::var("PROP_LOT_DISCORD_WEBHOOK_URL")
        .context("PROP_LOT_DISCORD_WEBHOOK_URL is not set in env")?;

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, webhook_url.as_str())
        .await
        .context("Failed to create webhook from URL")?;

    let message = Embed::fake(|e| {
        e.author(|a| {
            a.name(format!(
                "{}...{}",
                &idea.creator_id[0..4],
                &idea.creator_id[38..42]
            ))
        })
        .title(format!("New Idea: {}", idea.title))
        .url(format!("{}/idea/{}", base_url, idea.id))
        .description(&idea.tldr)
        .colour(0xFFB911)
    });

    webhook
        .execute(&http, false, |w| w.embeds(vec![message]))
        .await
        .context("Failed to execute webhook")?;

    set_idea_cache(idea).unwrap();

    Ok(())
}

pub(crate) async fn handle_new_vote(vote: &Vote) -> Result<()> {
    if let Ok(Some(idea)) = get_idea_cache(vote.idea_id) {
        debug!("New Vote on Proposal: {}", idea.title)
    } else {
        error!("No idea found for given id: {}", vote.idea_id);
    }
    set_vote_cache(vote).unwrap();
    Ok(())
}

pub(crate) async fn handle_new_comment(comment: &Comment) -> Result<()> {
    if let Ok(Some(idea)) = get_idea_cache(comment.idea_id) {
        debug!("New Comment on Idea: {}", idea.title)
    } else {
        error!("No idea found for given id: {}", comment.idea_id);
    }
    set_comment_cache(comment).unwrap();
    Ok(())
}
