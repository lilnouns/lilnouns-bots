use log::{error, info, Level};
use tracing_subscriber::{
  fmt::{format::Pretty, time::UtcTime},
  prelude::*,
};
use tracing_web::{performance_layer, MakeWebConsoleWriter};
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
fn start() {
  utils::set_panic_hook();

  let fmt_layer = tracing_subscriber::fmt::layer()
    .with_ansi(false) // Only partially supported across browsers
    .without_time() // std::time is not available in browsers
    .with_writer(MakeWebConsoleWriter::new()); // write events to the console
  let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

  tracing_subscriber::registry()
    .with(fmt_layer)
    .with(perf_layer)
    .init();
}

#[event(scheduled)]
async fn cron(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
  match start(&event, &env).await {
    Ok(_) => info!("Operation was a success."),
    Err(e) => error!("An error occurred: {:?}", e),
  }
}
