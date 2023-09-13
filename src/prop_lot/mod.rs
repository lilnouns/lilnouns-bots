use std::sync::Arc;

use futures::future::join_all;
use log::{error, info};

pub use fetcher::fetch_ideas;

use crate::prop_lot::cacher::set_idea_cache;

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    match fetch_ideas().await {
        Some(ideas) => {
            let ideas_ids: Vec<String> = ideas.iter().map(|i| i.id.to_string()).collect();
            info!("Fetched ideas ids({})", ideas_ids.join(","));

            let mut tasks = Vec::new();

            for idea in ideas {
                let arc_idea = Arc::new(idea);
                let task = tokio::spawn({
                    let arc_idea = Arc::clone(&arc_idea);
                    async move {
                        set_idea_cache(&*arc_idea).await.unwrap();
                    }
                });
                tasks.push(task);
            }

            join_all(tasks).await;
        }
        None => error!("Error: No ideas found"), // don't bail, just print an error
    };
}
