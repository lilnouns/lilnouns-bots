use std::sync::Arc;

use futures::future::join_all;
use log::info;

pub use fetcher::fetch_auctions;

use crate::prop_house::cacher::set_auction_cache;

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    let auctions = fetch_auctions().await;

    if let Some(auction_list) = auctions {
        let auctions_ids: Vec<String> = auction_list.iter().map(|i| i.id.to_string()).collect();
        info!("Fetched auctions ids({})", auctions_ids.join(","));

        let mut tasks = Vec::new();

        for auction in auction_list {
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
}
