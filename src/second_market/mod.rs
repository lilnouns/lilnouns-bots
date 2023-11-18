use serde::{Deserialize, Serialize};
use worker::{Env, Result};

use crate::{
  cache::Cache,
  second_market::{
    fetcher::RestFetcher,
    handler::{discord::DiscordHandler, farcaster::FarcasterHandler, Handler},
  },
};

pub(crate) mod fetcher;
mod handler;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Floor {
  pub id: String,
  pub price: f64,
  pub source: String,
  pub created_at: String,
  pub previous_price: f64,
}

pub struct SecondMarket {
  cache: Cache,
  fetcher: RestFetcher,
  handlers: Vec<Box<dyn Handler>>,
}

impl SecondMarket {
  pub fn new(cache: Cache, fetcher: RestFetcher, handlers: Vec<Box<dyn Handler>>) -> Self {
    Self {
      cache,
      fetcher,
      handlers,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<Self> {
    let cache = Cache::new_from_env(env);
    let fetcher = RestFetcher::new_from_env(env)?;
    let mut handlers = vec![];

    if env
      .var("SECOND_MARKET_DISCORD_ENABLED")
      .unwrap()
      .to_string()
      == "true"
    {
      let discord_handler: Box<dyn Handler> = Box::new(DiscordHandler::new_from_env(env)?);
      handlers.push(discord_handler);
    }

    if env
      .var("SECOND_MARKET_FARCASTER_ENABLED")
      .unwrap()
      .to_string()
      == "true"
    {
      let farcaster_handler: Box<dyn Handler> = Box::new(FarcasterHandler::new_from_env(env)?);
      handlers.push(farcaster_handler);
    }

    Ok(Self::new(cache, fetcher, handlers))
  }

  pub async fn setup(&self) {}

  pub async fn start(&self) -> Result<()> {
    Ok(())
  }
}
