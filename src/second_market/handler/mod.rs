use async_trait::async_trait;
use worker::Result;

use crate::second_market::fetcher::Collection;

pub(crate) mod discord;
pub(crate) mod farcaster;

#[async_trait(? Send)]
pub trait Handler {
  async fn handle_new_floor(&self, collection: &Collection) -> Result<()>;
}
