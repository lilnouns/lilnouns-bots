use async_trait::async_trait;
use worker::Result;

use crate::prop_house::{Auction, Proposal, Vote};

pub(crate) mod discord;
pub(crate) mod farcaster;

#[async_trait(? Send)]
pub trait Handler {
  async fn handle_new_auction(&self, auction: &Auction) -> Result<()>;
  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()>;
  async fn handle_new_vote(&self, vote: &Vote) -> Result<()>;
}
