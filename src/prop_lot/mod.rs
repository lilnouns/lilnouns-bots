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
        for idea in &ideas {
            if let Ok(cached_idea) = get_idea_cache(idea.id.try_into().unwrap()) {
                if cached_idea.is_none() {
                    info!("Handle a new idea... ({:?})", idea.id);
                    if let Err(err) = handle_new_idea(&idea).await {
                        error!("Failed to handle new idea: {:?}", err);
                    }
                }
            }
        }
    }

    if let Some(votes) = fetch_votes().await {
        for vote in &votes {
            if let Ok(cached_vote) = get_vote_cache(vote.id.try_into().unwrap()) {
                if cached_vote.is_none() {
                    info!("Handle a new vote... ({:?})", vote.id);
                    if let Err(err) = handle_new_vote(&vote).await {
                        error!("Failed to handle new vote: {:?}", err);
                    }
                }
            }
        }
    }

    if let Some(comments) = fetch_comments().await {
        for comment in &comments {
            if let Ok(cached_comment) = get_comment_cache(comment.id.try_into().unwrap()) {
                if cached_comment.is_none() {
                    info!("Handle a new comment... ({:?})", comment.id);
                    if let Err(err) = handle_new_comment(&comment).await {
                        error!("Failed to handle new comment: {:?}", err);
                    }
                }
            }
        }
    }
}
