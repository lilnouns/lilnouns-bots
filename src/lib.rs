use log::{error, info, Level};
use worker::{event, Env, Result, ScheduleContext, ScheduledEvent};

use crate::{lil_nouns::LilNouns, meta_gov::MetaGov, prop_house::PropHouse, prop_lot::PropLot};

mod cache;
mod lil_nouns;
mod meta_gov;
mod prop_house;
mod prop_lot;
mod utils;

async fn start(env: &Env) -> Result<()> {
  if env.var("LIL_NOUNS_ENABLED").unwrap().to_string() == "true" {
    match LilNouns::new_from_env(env) {
      Ok(result) => match result.start().await {
        Ok(_) => info!("LilNouns started successfully"),
        Err(error) => error!("Failed to start LilNouns: {:?}", error),
      },

      Err(error) => error!("Failed to create LilNouns: {:?}", error),
    }
  };

  if env.var("META_GOV_ENABLED").unwrap().to_string() == "true" {
    match MetaGov::new_from_env(env) {
      Ok(result) => match result.start().await {
        Ok(_) => info!("MetaGov started successfully"),
        Err(error) => error!("Failed to start MetaGov: {:?}", error),
      },

      Err(error) => error!("Failed to create MetaGov: {:?}", error),
    }
  };

  if env.var("PROP_HOUSE_ENABLED").unwrap().to_string() == "true" {
    match PropHouse::new_from_env(env) {
      Ok(result) => match result.start().await {
        Ok(_) => info!("PropHouse started successfully"),
        Err(error) => error!("Failed to start PropHouse: {:?}", error),
      },

      Err(error) => error!("Failed to create PropHouse: {:?}", error),
    }
  }

  if env.var("PROP_LOT_ENABLED").unwrap().to_string() == "true" {
    match PropLot::new_from_env(env) {
      Ok(result) => match result.start().await {
        Ok(_) => info!("PropLot started successfully"),
        Err(error) => error!("Failed to start PropLot: {:?}", error),
      },

      Err(error) => error!("Failed to create PropLot: {:?}", error),
    }
  }

  Ok(())
}

#[event(start)]
pub fn start() {
  worker_logger::init_with_level(&Level::Debug);
  utils::set_panic_hook();
}

#[event(scheduled)]
async fn cron(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
  match start(&env).await {
    Ok(_) => info!("Operation was a success."),
    Err(e) => error!("An error occurred: {:?}", e),
  }
}
