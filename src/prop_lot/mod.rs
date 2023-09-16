use std::sync::Arc;

use futures::future::join_all;
use log::{error, info};

pub use fetcher::fetch_ideas;

use crate::prop_lot::cacher::{get_idea_cache, set_comment_cache, set_idea_cache, set_vote_cache};
use crate::prop_lot::fetcher::{fetch_comments, fetch_votes};
use crate::prop_lot::handler::handle_new_idea;

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    if let Some(ideas) = fetch_ideas().await {
        let mut tasks = Vec::new();

        for idea in ideas {
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

    if let Some(votes) = fetch_votes().await {
        let mut tasks = Vec::new();

        for vote in votes {
            let arc_vote = Arc::new(vote);
            let task = tokio::spawn({
                let arc_vote = Arc::clone(&arc_vote);
                async move {
                    info!("Cache a new vote... ({:?})", arc_vote.id);
                    let _ = set_vote_cache(&*arc_vote).await.map_err(|e| {
                        error!("Error while trying to set vote cache: {}", e);
                    });
                }
            });

            tasks.push(task);
        }

        join_all(tasks).await;
    }

    if let Some(comments) = fetch_comments().await {
        let mut tasks = Vec::new();

        for comment in comments {
            let arc_comment = Arc::new(comment);
            let task = tokio::spawn({
                let arc_comment = Arc::clone(&arc_comment);
                async move {
                    info!("Cache a new comment... ({:?})", arc_comment.id);
                    let _ = set_comment_cache(&*arc_comment).await.map_err(|e| {
                        error!("Error while trying to set comment cache: {}", e);
                    });
                }
            });

            tasks.push(task);
        }

        join_all(tasks).await;
    }
}

pub async fn start() {
    if let Some(ideas) = fetch_ideas().await {
        let mut tasks = Vec::new();

        for idea in ideas {
            let arc_idea = Arc::new(idea);
            let cached_idea = get_idea_cache(arc_idea.id.try_into().unwrap()).await;
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
