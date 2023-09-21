use log::{error, info};
use worker::Result;

use fetcher::{Comment, GraphQLFetcher, Idea, Vote};
use handler::DiscordHandler;

use crate::cache::Cache;

pub mod fetcher;
pub mod handler;

pub async fn setup(cache: &Cache, fetcher: &GraphQLFetcher) -> Result<()> {
    if let Some(ideas) = fetcher.fetch_ideas().await {
        for idea in ideas {
            cache
                .put(&format!("{}{}", "PROP_LOT_IDEA_", idea.id), &idea)
                .await;
        }
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        for vote in votes {
            cache
                .put(&format!("{}{}", "PROP_LOT_VOTE_", vote.id), &vote)
                .await;
        }
    }

    if let Some(comments) = fetcher.fetch_comments().await {
        for comment in comments {
            cache
                .put(&format!("{}{}", "PROP_LOT_COMMENT_", comment.id), &comment)
                .await;
        }
    }

    Ok(())
}

pub async fn start(
    cache: &Cache,
    fetcher: &GraphQLFetcher,
    handler: &DiscordHandler,
) -> Result<()> {
    if let Some(ideas) = fetcher.fetch_ideas().await {
        for idea in &ideas {
            let cached_idea: Option<Idea> = cache
                .get(&format!("{}{}", "PROP_LOT_IDEA_", idea.id))
                .await?;

            if cached_idea.is_none() {
                info!("Handle a new idea... ({:?})", idea.id);
                if let Err(err) = handler.handle_new_idea(idea).await {
                    error!("Failed to handle new idea: {:?}", err);
                }
            }
        }
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        for vote in &votes {
            let cached_vote: Option<Vote> = cache
                .get(&format!("{}{}", "PROP_LOT_VOTE_", vote.id))
                .await?;

            if cached_vote.is_none() {
                info!("Handle a new vote... ({:?})", vote.id);
                if let Err(err) = handler.handle_new_vote(vote).await {
                    error!("Failed to handle new vote: {:?}", err);
                }
            }
        }
    }

    if let Some(comments) = fetcher.fetch_comments().await {
        for comment in &comments {
            let cached_comment: Option<Comment> = cache
                .get(&format!("{}{}", "PROP_LOT_COMMENT_", comment.id))
                .await?;

            if cached_comment.is_none() {
                info!("Handle a new comment... ({:?})", comment.id);
                if let Err(err) = handler.handle_new_comment(comment).await {
                    error!("Failed to handle new comment: {:?}", err);
                }
            }
        }
    }

    Ok(())
}
