use async_trait::async_trait;
use worker::Result;

use crate::prop_lot::{Comment, Idea, Vote};

pub(crate) mod discord;
pub(crate) mod farcaster;

#[async_trait(? Send)]
pub trait Handler {
  async fn handle_new_idea(&self, idea: &Idea) -> Result<()>;
  async fn handle_new_vote(&self, vote: &Vote) -> Result<()>;
  async fn handle_new_comment(&self, comment: &Comment) -> Result<()>;
}
