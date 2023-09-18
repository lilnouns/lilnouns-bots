use std::sync::Arc;

use futures::future::join_all;
use log::{error, info};

use fetcher::fetch_ideas;

use crate::prop_lot::cacher::{
    get_comment_cache, get_idea_cache, get_vote_cache, set_comments_cache, set_ideas_cache,
    set_votes_cache,
};
use crate::prop_lot::fetcher::{fetch_comments, fetch_votes};
use crate::prop_lot::handler::{handle_new_comment, handle_new_idea, handle_new_vote};

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    if let Some(ideas) = fetch_ideas().await {
        set_ideas_cache(&ideas).unwrap();
    }

    if let Some(votes) = fetch_votes().await {
        set_votes_cache(&votes).unwrap();
    }

    if let Some(comments) = fetch_comments().await {
        set_comments_cache(&comments).unwrap();
    }
}

pub async fn start() {
    if let Some(ideas) = fetch_ideas().await {
        let mut tasks = Vec::new();

        for idea in ideas {
            let arc_idea = Arc::new(idea);
            if let Ok(cached_idea) = get_idea_cache(arc_idea.id.try_into().unwrap()) {
                let task = tokio::spawn({
                    let arc_idea = Arc::clone(&arc_idea);
                    async move {
                        if cached_idea.is_none() {
                            info!("Handle a new idea... ({:?})", arc_idea.id);
                            let _ = handle_new_idea(&arc_idea)
                                .await
                                .map_err(|err| error!("Failed to handle new idea: {:?}", err));
                        }
                    }
                });

                tasks.push(task);
            }
        }

        join_all(tasks).await;
    }
    if let Some(votes) = fetch_votes().await {
        let mut tasks = Vec::new();

        for vote in votes {
            let arc_vote = Arc::new(vote);
            if let Ok(cached_vote) = get_vote_cache(arc_vote.id.try_into().unwrap()) {
                let task = tokio::spawn({
                    let arc_vote = Arc::clone(&arc_vote);
                    async move {
                        if cached_vote.is_none() {
                            info!("Handle a new vote... ({:?})", arc_vote.id);
                            let _ = handle_new_vote(&arc_vote)
                                .await
                                .map_err(|err| error!("Failed to handle new vote: {:?}", err));
                        }
                    }
                });

                tasks.push(task);
            }
        }

        join_all(tasks).await;
    }
    if let Some(comments) = fetch_comments().await {
        let mut tasks = Vec::new();

        for comment in comments {
            let arc_comment = Arc::new(comment);
            if let Ok(cached_comment) = get_comment_cache(arc_comment.id.try_into().unwrap()) {
                let task = tokio::spawn({
                    let arc_comment = Arc::clone(&arc_comment);
                    async move {
                        if cached_comment.is_none() {
                            info!("Handle a new comment... ({:?})", arc_comment.id);
                            let _ = handle_new_comment(&arc_comment)
                                .await
                                .map_err(|err| error!("Failed to handle new comment: {:?}", err));
                        }
                    }
                });

                tasks.push(task);
            }
        }

        join_all(tasks).await;
    }
}
