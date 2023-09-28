use cfg_if::cfg_if;
use log::{error, info, Level};
use worker::{event, Env, Result, ScheduleContext, ScheduledEvent};

use crate::{meta_gov::MetaGov, prop_house::PropHouse, prop_lot::PropLot};

mod cache;
mod meta_gov;
mod prop_house;
mod prop_lot;
mod utils;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

async fn start(env: &Env) -> Result<()> {
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

#[event(scheduled)]
async fn cron(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
  worker_logger::init_with_level(&Level::Debug);
  set_panic_hook();

  match start(&env).await {
    Ok(_) => info!("Operation was a success."),
    Err(e) => error!("An error occurred: {:?}", e),
  }
}
