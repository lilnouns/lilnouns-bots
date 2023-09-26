use async_trait::async_trait;

use crate::prop_house::fetcher::{Auction, Proposal, Vote};

pub(crate) mod discord;
mod farcaster;

#[async_trait(? Send)]
pub(crate) trait Handler {
    async fn handle_new_auction(&self, auction: &Auction) -> worker::Result<()>;
    async fn handle_new_proposal(&self, proposal: &Proposal) -> worker::Result<()>;
    async fn handle_new_vote(&self, vote: &Vote) -> worker::Result<()>;
}
