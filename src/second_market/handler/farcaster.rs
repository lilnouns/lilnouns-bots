use async_trait::async_trait;
use worker::{Env, Result};

use crate::second_market::{handler::Handler, Floor};

pub(crate) struct FarcasterHandler {}

impl FarcasterHandler {
  pub fn new() -> Self {
    Self {}
  }

  pub fn new_from_env(env: &Env) -> Result<Self> {
    Ok(Self::new())
  }
}

#[async_trait(? Send)]
impl Handler for FarcasterHandler {
  async fn handle_new_floor(&self, floor: &Floor) -> Result<()> {
    todo!()
  }
}
