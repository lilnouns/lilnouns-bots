use async_trait::async_trait;
use worker::Result;

use crate::second_market::Floor;

pub(crate) mod discord;
pub(crate) mod farcaster;

#[async_trait(? Send)]
pub trait Handler {
  async fn handle_new_floor(&self, floor: &Floor) -> Result<()>;
}
