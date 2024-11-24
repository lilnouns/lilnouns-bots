use log::{error, info, Level};
use worker::{event, Env, Result, ScheduleContext, ScheduledEvent};

use crate::{
  lil_nouns::LilNouns,
  meta_gov::MetaGov,
  prop_house::PropHouse,
  prop_lot::PropLot,
  second_market::SecondMarket,
};

mod cache;
mod lil_nouns;
mod meta_gov;
mod prop_house;
mod prop_lot;
mod second_market;
mod utils;

async fn start(event: &ScheduledEvent, env: &Env) -> Result<()> {
  match event.cron().as_str() {
    "*/5 * * * *" => {
      if env.var("LIL_NOUNS_ENABLED")?.to_string() == "true" {
        match LilNouns::new_from_env(env) {
          Ok(result) => match result.start().await {
            Ok(_) => info!("LilNouns started successfully"),
            Err(error) => error!("Failed to start LilNouns: {:?}", error),
          },

          Err(error) => error!("Failed to create LilNouns: {:?}", error),
        }
      };

      if env.var("META_GOV_ENABLED")?.to_string() == "true" {
        match MetaGov::new_from_env(env) {
          Ok(result) => match result.start().await {
            Ok(_) => info!("MetaGov started successfully"),
            Err(error) => error!("Failed to start MetaGov: {:?}", error),
          },

          Err(error) => error!("Failed to create MetaGov: {:?}", error),
        }
      };

      if env.var("PROP_HOUSE_ENABLED")?.to_string() == "true" {
        match PropHouse::new_from_env(env) {
          Ok(result) => match result.start().await {
            Ok(_) => info!("PropHouse started successfully"),
            Err(error) => error!("Failed to start PropHouse: {:?}", error),
          },

          Err(error) => error!("Failed to create PropHouse: {:?}", error),
        }
      }

      if env.var("PROP_LOT_ENABLED")?.to_string() == "true" {
        match PropLot::new_from_env(env) {
          Ok(result) => match result.start().await {
            Ok(_) => info!("PropLot started successfully"),
            Err(error) => error!("Failed to start PropLot: {:?}", error),
          },

          Err(error) => error!("Failed to create PropLot: {:?}", error),
        }
      }
    }
    "0 0 * * *" => {
      if env.var("SECOND_MARKET_ENABLED")?.to_string() == "true" {
        match SecondMarket::new_from_env(env) {
          Ok(result) => match result.start().await {
            Ok(_) => info!("SecondMarket started successfully"),
            Err(error) => error!("Failed to start SecondMarket: {:?}", error),
          },

          Err(error) => error!("Failed to create SecondMarket: {:?}", error),
        }
      }
    }
    _ => {}
  }

  Ok(())
}

#[event(start)]
pub fn start() {
  worker_logger::init_with_level(&Level::Trace);
  utils::set_panic_hook();
}

#[event(scheduled)]
async fn cron(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
  match start(&event, &env).await {
    Ok(_) => info!("Operation was a success."),
    Err(e) => error!("An error occurred: {:?}", e),
  }
}
