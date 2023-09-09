use serde_json::Value;
use std::error::Error;
use crate::event::Event;

pub trait DiscordBot {
    fn prepare(&self) -> Result<Value, Box<dyn Error>>;
    fn process(&self, value: &Value) -> Result<Vec<Event>, Box<dyn Error>>;
    fn dispatch(&self, event: &[Event]) -> Result<(), Box<dyn Error>>;
}
