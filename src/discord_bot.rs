use anyhow::Result;
use async_trait::async_trait;

use crate::event::Event;

#[async_trait]
pub trait DiscordBot {
    type RawData;

    async fn prepare(&self) -> Result<Self::RawData>;
    async fn process(&self, source: Self::RawData) -> Result<Vec<Event>>;
    async fn dispatch(&self, event: &[Event]) -> Result<()>;
}
