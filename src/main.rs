use std::error::Error;

use discord_bot::DiscordBot;
use prop_house_discord_bot::PropHouseDiscordBot;

mod discord_bot;
mod event;
mod prop_house_discord_bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = PropHouseDiscordBot::new();
    let source = bot.prepare().await?;
    let events = bot.process(source).await?;
    bot.dispatch(events).await?;

    Ok(())
}
