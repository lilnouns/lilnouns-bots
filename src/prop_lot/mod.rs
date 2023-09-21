use log::{error, info};
use worker::Result;

use fetcher::{Comment, GraphQLFetcher, Idea, Vote};
use handler::DiscordHandler;

use crate::cache::Cache;

pub mod fetcher;
pub mod handler;

pub async fn setup(cache: &Cache, fetcher: &GraphQLFetcher) -> Result<()> {
    if let Some(ideas) = fetcher.fetch_ideas().await {
        cache.put("prop_lot:ideas", &ideas).await;
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        cache.put("prop_lot:votes", &votes).await;
    }

    if let Some(comments) = fetcher.fetch_comments().await {
        cache.put("prop_lot:comments", &comments).await;
    }

    Ok(())
}

pub async fn start(
    cache: &Cache,
    fetcher: &GraphQLFetcher,
    handler: &DiscordHandler,
) -> Result<()> {
    if let Some(ideas) = fetcher.fetch_ideas().await {
        if let Some(old_ideas) = cache.get::<Vec<Idea>>("prop_lot:ideas").await? {
            let old_ids: Vec<_> = old_ideas.iter().map(|idea| &idea.id).collect();
            let new_ideas: Vec<_> = ideas
                .iter()
                .filter(|idea| !old_ids.contains(&&idea.id))
                .collect();

            for idea in new_ideas {
                info!("Handle a new idea... ({:?})", idea.id);
                if let Err(err) = handler.handle_new_idea(idea).await {
                    error!("Failed to handle new idea: {:?}", err);
                }
            }
        }
        cache.put("prop_lot:ideas", &ideas).await;
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        if let Some(old_votes) = cache.get::<Vec<Vote>>("prop_lot:votes").await? {
            let old_ids: Vec<_> = old_votes.iter().map(|vote| &vote.id).collect();
            let new_votes: Vec<_> = votes
                .iter()
                .filter(|vote| !old_ids.contains(&&vote.id))
                .collect();

            for vote in new_votes {
                info!("Handle a new vote... ({:?})", vote.id);
                if let Err(err) = handler.handle_new_vote(vote).await {
                    error!("Failed to handle new vote: {:?}", err);
                }
            }
        }
        cache.put("prop_lot:votes", &votes).await;
    }

    if let Some(comments) = fetcher.fetch_comments().await {
        if let Some(old_comments) = cache.get::<Vec<Comment>>("prop_lot:comments").await? {
            let old_ids: Vec<_> = old_comments.iter().map(|comment| &comment.id).collect();
            let new_comments: Vec<_> = comments
                .iter()
                .filter(|comment| !old_ids.contains(&&comment.id))
                .collect();

            for comment in new_comments {
                info!("Handle a new comment... ({:?})", comment.id);
                if let Err(err) = handler.handle_new_comment(comment).await {
                    error!("Failed to handle new comment: {:?}", err);
                }
            }
        }
        cache.put("prop_lot:comments", &comments).await;
    }

    Ok(())
}
