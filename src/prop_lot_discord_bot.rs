use anyhow::Result;
use async_trait::async_trait;

use crate::discord_bot::DiscordBot;
use crate::event::Event;

pub struct PropLotDiscordBot {}

impl PropLotDiscordBot {
    pub(crate) fn new() -> Self {
        PropLotDiscordBot {}
    }
}

#[async_trait]
impl DiscordBot for PropLotDiscordBot {
    type RawData = ();

    async fn prepare(&self) -> Result<Self::RawData> {
        todo!()
    }

    async fn process(&self, source: Self::RawData) -> Result<Vec<Event>> {
        todo!()
    }

    async fn dispatch(&self, events: Vec<Event>) -> Result<()> {
        todo!()
    }
}
