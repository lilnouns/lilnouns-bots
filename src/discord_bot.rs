use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::event::Event;

#[async_trait]
pub trait DiscordBot {
    async fn prepare(&self) -> Result<Value>;
    async fn process(&self, value: &Value) -> Result<Vec<Event>>;
    async fn dispatch(&self, event: &[Event]) -> Result<()>;
}
