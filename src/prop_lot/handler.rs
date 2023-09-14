use std::env;

use anyhow::Context;
use anyhow::Result;
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;

use crate::prop_lot::fetcher::Idea;

pub async fn handle_new_idea(idea: &Idea) -> Result<()> {
    let base_url = env::var("PROP_LOT_BASE_URL").context("PROP_LOT_BASE_URL is not set in env")?;

    let webhook_url = env::var("PROP_LOT_DISCORD_WEBHOOK_URL")
        .context("PROP_LOT_DISCORD_WEBHOOK_URL is not set in env")?;

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, webhook_url.as_str())
        .await
        .context("Failed to create webhook from URL")?;

    let message = Embed::fake(|e| {
        e.title(format!("New Idea: {}", idea.title))
            .url(format!("{}/idea/{}", base_url, idea.id))
            .description(&idea.tldr)
            .colour(0xFFB911)
    });

    webhook
        .execute(&http, false, |w| w.embeds(vec![message]))
        .await
        .context("Failed to execute webhook")?;

    Ok(())
}
