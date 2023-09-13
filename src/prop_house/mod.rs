use std::sync::Arc;

use futures::future::join_all;
use log::{error, info};

pub use fetcher::fetch_auctions;

use crate::prop_house::cacher::set_auction_cache;

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    match fetch_auctions().await {
        Some(auctions) => {
            let auctions_ids: Vec<String> = auctions.iter().map(|i| i.id.to_string()).collect();
            info!("Fetched auctions ids({})", auctions_ids.join(","));

            let mut tasks = Vec::new();

            for auction in auctions {
                let arc_auction = Arc::new(auction);
                let task = tokio::spawn({
                    let arc_auction = Arc::clone(&arc_auction);
                    async move {
                        set_auction_cache(&*arc_auction).await.unwrap();
                    }
                });
                tasks.push(task);
            }

            join_all(tasks).await;
        }
        None => error!("Error: No auctions found"), // don't bail, just print an error
    };
}
