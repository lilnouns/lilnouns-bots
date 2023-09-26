use cfg_if::cfg_if;
use log::{error, info, Level};
use worker::{event, Env, Result, ScheduleContext, ScheduledEvent};

use crate::cache::Cache;
use crate::prop_house::PropHouse;
use crate::prop_lot::handler::farcaster::FarcasterHandler;
use crate::prop_lot::{
    fetcher::GraphQLFetcher as PropLotFetcher, handler::discord::DiscordHandler, PropLot,
};

mod cache;
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
    let cache = Cache::from(env);

    match PropLot::<DiscordHandler>::new(
        cache.clone(),
        PropLotFetcher::from(env)?,
        DiscordHandler::from(env)?,
    )
    .start()
    .await
    {
        Ok(_) => info!("PropLot started successfully"),
        Err(error) => error!("Failed to start PropLot: {:?}", error),
    }

    match PropLot::<FarcasterHandler>::new(
        cache.clone(),
        PropLotFetcher::from(env)?,
        FarcasterHandler::from(env)?,
    )
    .start()
    .await
    {
        Ok(_) => info!("PropLot started successfully"),
        Err(error) => error!("Failed to start PropLot: {:?}", error),
    }

    match PropHouse::from(env) {
        Ok(result) => match result.start().await {
            Ok(_) => info!("PropHouse started successfully"),
            Err(error) => error!("Failed to start PropHouse: {:?}", error),
        },

        Err(error) => error!("Failed to create PropHouse: {:?}", error),
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
