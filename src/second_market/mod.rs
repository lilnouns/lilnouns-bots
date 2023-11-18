use log::{debug, error, info, warn};
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
  pub kind: String,
  pub source: Option<String>,
  pub created_at: String,
  pub new_price: Option<f64>,
  pub old_price: Option<f64>,
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

  pub async fn setup(&self) {
    debug!("Setup function started.");

    if !self.cache.has("second_market:floors").await {
      if let Some(floors) = self.fetcher.fetch_floors().await {
        info!("Fetched {:?} floor.", floors.len());
        debug!("Putting fetched floors into cache.");
        self.cache.put("second_market:floors", &floors).await;
      } else {
        warn!("Failed to fetch floors");
      }
    }

    debug!("Setup function finished.");
  }

  pub async fn start(&self) -> Result<()> {
    self.setup().await;

    debug!("Start function started.");

    if let Some(floors) = self.fetcher.fetch_floors().await {
      debug!("Fetched {:?} floors.", floors.len());

      let mut new_floors = Vec::new();

      if let Some(old_floors) = self.cache.get::<Vec<Floor>>("second_market:floors").await? {
        let old_ids: Vec<_> = old_floors.iter().map(|floor| &floor.id).collect();
        new_floors = floors
          .iter()
          .filter(|floor| !old_ids.contains(&&floor.id))
          .cloned()
          .collect();

        debug!("Found {:?} new floors.", new_floors.len());

        if let Some(floor) = new_floors.get(0) {
          if floor.kind == "new-order" && floor.new_price != floor.old_price {
            info!("Handle a new floor...");
            for handler in &self.handlers {
              if let Err(err) = handler.handle_new_floor(floor).await {
                error!("Failed to handle new floor: {:?}", err);
              } else {
                debug!("Successfully handled new floor: {:?}", floor.id);
              }
            }
          }
        }
      }

      if !new_floors.is_empty() {
        self.cache.put("second_market:floors", &floors).await;
        info!("Updated floors in cache");
      }
    } else {
      warn!("Failed to fetch floors");
    }

    debug!("Start function finished.");

    Ok(())
  }
}
