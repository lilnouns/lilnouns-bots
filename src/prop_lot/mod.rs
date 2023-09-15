use std::sync::Arc;

use futures::future::join_all;
use log::{error, info};

pub use fetcher::fetch_ideas;

use crate::prop_lot::cacher::{get_idea_cache, set_idea_cache};
use crate::prop_lot::handler::handle_new_idea;

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    let ideas = fetch_ideas().await;

    if let Some(idea_list) = ideas {
        let mut tasks = Vec::new();

        for idea in idea_list {
            let arc_idea = Arc::new(idea);
            let task = tokio::spawn({
                let arc_idea = Arc::clone(&arc_idea);
                async move {
                    info!("Cache a new idea... ({:?})", arc_idea.id);
                    let _ = set_idea_cache(&*arc_idea).await.map_err(|e| {
                        error!("Error while trying to set idea cache: {}", e);
                    });
                }
            });

            tasks.push(task);
        }

        join_all(tasks).await;
    }
}

pub async fn start() {
    let ideas = fetch_ideas().await;

    if let Some(idea_list) = ideas {
        let mut tasks = Vec::new();

        for idea in idea_list {
            let arc_idea = Arc::new(idea);
            let cached_idea = get_idea_cache(arc_idea.id as i32).await;
            let task = tokio::spawn({
                let arc_idea = Arc::clone(&arc_idea);
                async move {
                    if cached_idea.is_none() {
                        info!("Handle a new idea... ({:?})", arc_idea.id);
                        let _ = handle_new_idea(&*arc_idea)
                            .await
                            .map_err(|err| error!("Failed to handle new idea: {:?}", err));
                    }
                }
            });

            tasks.push(task);
        }

        join_all(tasks).await;
    }
}
