use async_trait::async_trait;

use crate::prop_lot::fetcher::{Comment, Idea, Vote};

pub(crate) mod discord;
pub(crate) mod farcaster;

#[async_trait(? Send)]
pub trait Handler {
    async fn handle_new_idea(&self, idea: &Idea) -> worker::Result<()>;
    async fn handle_new_vote(&self, vote: &Vote) -> worker::Result<()>;
    async fn handle_new_comment(&self, comment: &Comment) -> worker::Result<()>;
}
