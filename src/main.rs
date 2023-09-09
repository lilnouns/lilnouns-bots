mod event;
mod discord_bot;
mod prop_house_discord_bot;

use std::error::Error;
use discord_bot::DiscordBot;
use prop_house_discord_bot::PropHouseDiscordBot;

fn main() -> Result<(), Box<dyn Error>>{
    let bot = PropHouseDiscordBot::new();
    bot.prepare().expect("TODO: panic message");

    Ok(())
}
