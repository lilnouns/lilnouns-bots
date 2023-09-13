use std::sync::Arc;

use futures::future::join_all;
use log::info;

pub use fetcher::fetch_ideas;

use crate::prop_lot::cacher::set_idea_cache;

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    let ideas = fetch_ideas().await;

    if let Some(idea_list) = ideas {
        let ideas_ids: Vec<String> = idea_list.iter().map(|i| i.id.to_string()).collect();
        info!("Fetched ideas ids({})", ideas_ids.join(","));

        let mut tasks = Vec::new();

        for idea in idea_list {
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
}
