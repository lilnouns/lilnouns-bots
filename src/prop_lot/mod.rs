use log::{error, info};

pub use fetcher::fetch_ideas;

mod fetcher;

pub async fn setup() {
    match fetch_ideas().await {
        Some(ideas) => {
            let ideas_ids: Vec<String> = ideas.iter().map(|i| i.id.to_string()).collect();
            info!("Fetched ideas ids({})", ideas_ids.join(","));
        }
        None => error!("Error: No ideas found"), // don't bail, just print an error
    };
}
