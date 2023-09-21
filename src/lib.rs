use cfg_if::cfg_if;
use log::{error, info, Level};
use worker::{event, Date, Env, Request, Response, Result, ScheduleContext, ScheduledEvent};

use prop_house::fetcher::GraphQLFetcher as PropHouseGraphQLFetcher;
use prop_house::handler::DiscordHandler as PropHouseDiscordHandler;
use prop_lot::fetcher::GraphQLFetcher as PropLotGraphQLFetcher;
use prop_lot::handler::DiscordHandler as PropLotDiscordHandler;

use crate::cache::Cache;

mod cache;
mod prop_house;
mod prop_lot;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

#[event(scheduled)]
async fn cron(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    worker_logger::init_with_level(&Level::Debug);
    set_panic_hook();

    let cache = Cache::from(&env);

    let prop_lot_key = "PROP_LOT_SETUP";
    let prop_lot_fetcher = PropLotGraphQLFetcher::from(&env).unwrap();
    let prop_lot_handler = PropLotDiscordHandler::from(&env).unwrap();

    let prop_house_key = "PROP_HOUSE_SETUP";
    let prop_house_fetcher = PropHouseGraphQLFetcher::from(&env).unwrap();
    let prop_house_handler = PropHouseDiscordHandler::from(&env).unwrap();

    if cache.get::<String>(prop_lot_key).await.ok().is_none() {
        if let Err(e) = prop_lot::setup(&cache, &prop_lot_fetcher).await {
            error!("Failed to setup prop_lot: {}", e);
        } else {
            // On successful setup, set the key in the cache.
            let now: String = chrono::Utc::now().to_string();
            cache.put(prop_lot_key, &now).await;
        }
    }

    if let Err(e) = prop_lot::start(&cache, &prop_lot_fetcher, &prop_lot_handler).await {
        error!("Failed to start prop_lot: {}", e);
    }

    if cache.get::<String>(prop_house_key).await.ok().is_none() {
        if let Err(e) = prop_house::setup(&cache, &prop_house_fetcher).await {
            error!("Failed to setup prop_house: {}", e);
        } else {
            // On successful setup, set the key in the cache.
            let now = chrono::Utc::now().to_string();
            cache.put(prop_house_key, &now).await;
        }
    }

    if let Err(e) = prop_house::start(&cache, &prop_house_fetcher, &prop_house_handler).await {
        error!("Failed to start prop_house: {}", e);
    }
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    worker_logger::init_with_level(&Level::Debug);
    set_panic_hook();

    info!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );

    Response::error("Bad Request", 400)
}
