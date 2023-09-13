use log::{error, info};

pub use fetcher::fetch_auctions;

mod fetcher;

pub async fn setup() {
    match fetch_auctions().await {
        Some(auctions) => {
            let auction_ids: Vec<String> = auctions.iter().map(|a| a.id.to_string()).collect();
            info!("Fetched auctions ids({})", auction_ids.join(","));
        }
        None => error!("Error: No auctions found"), // don't bail, just print an error
    };
}
