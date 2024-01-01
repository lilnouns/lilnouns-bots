use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use worker::{Env, Result};

use crate::{
  cache::Cache,
  second_market::{
    fetcher::{Collection, RestFetcher},
    handler::{discord::DiscordHandler, farcaster::FarcasterHandler, Handler},
  },
};

pub(crate) mod fetcher;
mod handler;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Floor {
  pub id: String,
  pub kind: String,
  pub price: Option<f64>,
  pub source: Option<String>,
  pub created_at: String,
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

    if !self.cache.has("second_market:collections").await {
      if let Some(collections) = self.fetcher.fetch_collections().await {
        info!("Fetched {:?} collections.", collections.len());
        debug!("Putting fetched collections into cache.");
        self
          .cache
          .put("second_market:collections", &collections)
          .await;
      } else {
        warn!("Failed to fetch collections");
      }
    }

    debug!("Setup function finished.");
  }

  pub async fn start(&self) -> Result<()> {
    self.setup().await;

    debug!("Start function started.");

    let new_collections = match self.fetcher.fetch_collections().await {
      Some(collections) => collections,
      None => {
        debug!("Failed to fetch new collections.");
        return Ok(());
      }
    };
    debug!("Fetched {:?} collections.", new_collections.len());

    let old_collections = match self
      .cache
      .get::<Vec<Collection>>("second_market:collections")
      .await?
    {
      Some(collections) => collections,
      None => {
        debug!("No old collections found in the cache.");
        return Ok(());
      }
    };

    if let (Some(old_collection), Some(new_collection)) =
      (old_collections.get(0), new_collections.get(0))
    {
      if old_collection.floor_ask.price.amount.decimal
        != new_collection.floor_ask.price.amount.decimal
      {
        info!("Handle a new floor...");

        for handler in &self.handlers {
          if let Err(err) = handler.handle_new_floor(new_collection).await {
            error!("Failed to handle new floor: {:?}", err);
          } else {
            debug!("Successfully handled new floor.");
          }
        }

        self
          .cache
          .put::<String>(
            "second_market:old_price",
            &new_collection.floor_ask.price.amount.decimal.to_string(),
          )
          .await;

        self
          .cache
          .put("second_market:collections", &new_collections)
          .await;
        info!("Updated collections in cache");
      } else {
        debug!("Floor has not changed.");
      }
    } else {
      debug!("Unable to compare floors: One of the collections is empty.");
    }

    debug!("Start function finished.");

    Ok(())
  }
}
