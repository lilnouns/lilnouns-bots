use anyhow::{Context, Result};

use discord_bot::DiscordBot;

use crate::prop_lot_discord_bot::PropLotDiscordBot;

mod cache;
mod discord_bot;
mod event;
mod prop_house_discord_bot;
mod prop_lot_discord_bot;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = PropLotDiscordBot::new();
    let source = bot.prepare().await?;
    let events = bot.process(source).await?;
    bot.dispatch(events).await?;

    Ok(())
}
