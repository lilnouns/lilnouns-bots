use std::error::Error;

use discord_bot::DiscordBot;
use prop_house_discord_bot::PropHouseDiscordBot;

mod event;
mod discord_bot;
mod prop_house_discord_bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = PropHouseDiscordBot::new();
    bot.prepare().await?;

    Ok(())
}
