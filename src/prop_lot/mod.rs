use log::{debug, error, info, warn};
use worker::Result;

use fetcher::{GraphQLFetcher, Idea, Vote};
use handler::DiscordHandler;

use crate::cache::Cache;
use crate::prop_lot::fetcher::Comment;

pub mod fetcher;
pub mod handler;

pub async fn setup(cache: &Cache, fetcher: &GraphQLFetcher) -> Result<()> {
    debug!("Setup function started.");
    if let Some(ideas) = fetcher.fetch_ideas().await {
        info!("Fetched {:?} idea.", ideas.len());
        debug!("Putting fetched ideas into cache.");
        cache.put("prop_lot:ideas", &ideas).await;
    } else {
        warn!("Failed to fetch ideas");
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        info!("Fetched {:?} votes.", votes.len());
        debug!("Putting fetched votes into cache.");
        cache.put("prop_lot:votes", &votes).await;
    } else {
        warn!("Failed to fetch votes");
    }

    if let Some(comments) = fetcher.fetch_comments().await {
        info!("Fetched {:?} comments.", comments.len());
        debug!("Putting fetched comments into cache.");
        cache.put("prop_lot:comments", &comments).await;
    } else {
        warn!("Failed to fetch comments");
    }

    debug!("Setup function finished.");
    Ok(())
}

pub async fn start(
    cache: &Cache,
    fetcher: &GraphQLFetcher,
    handler: &DiscordHandler,
) -> Result<()> {
    debug!("Start function started.");

    if let Some(ideas) = fetcher.fetch_ideas().await {
        debug!("Fetched {:?} ideas.", ideas.len());

        let mut new_ideas = Vec::new();

        if let Some(old_ideas) = cache.get::<Vec<Idea>>("prop_lot:ideas").await? {
            let old_ids: Vec<_> = old_ideas.iter().map(|idea| &idea.id).collect();
            new_ideas = ideas
                .iter()
                .filter(|idea| !old_ids.contains(&&idea.id))
                .cloned()
                .collect();

            debug!("Found {:?} new ideas.", new_ideas.len());

            for idea in &new_ideas {
                info!("Handle a new idea...");
                if let Err(err) = handler.handle_new_idea(idea).await {
                    error!("Failed to handle new idea: {:?}", err);
                } else {
                    debug!("Successfully handled new idea: {:?}", idea.id);
                }
            }
        }

        if !new_ideas.is_empty() {
            cache.put("prop_lot:ideas", &ideas).await;
            info!("Updated ideas in cache");
        }
    } else {
        warn!("Failed to fetch ideas");
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        debug!("Fetched {:?} votes.", votes.len());

        let mut new_votes = Vec::new();

        if let Some(old_votes) = cache.get::<Vec<Vote>>("prop_lot:votes").await? {
            let old_ids: Vec<_> = old_votes.iter().map(|vote| &vote.id).collect();
            new_votes = votes
                .iter()
                .filter(|vote| !old_ids.contains(&&vote.id))
                .cloned()
                .collect();

            debug!("Found {:?} new votes.", new_votes.len());

            for vote in &new_votes {
                info!("Handling a new vote...");
                if let Err(err) = handler.handle_new_vote(vote).await {
                    error!("Failed to handle new vote: {:?}", err);
                } else {
                    debug!("Successfully handled new vote: {:?}", vote.id);
                }
            }
        }

        if !new_votes.is_empty() {
            cache.put("prop_lot:votes", &votes).await;
            info!("Updated votes in cache");
        }
    } else {
        warn!("Failed to fetch votes");
    }

    if let Some(comments) = fetcher.fetch_comments().await {
        debug!("Fetched {:?} comments.", comments.len());

        let mut new_comments = Vec::new();

        if let Some(old_comments) = cache.get::<Vec<Comment>>("prop_lot:comments").await? {
            let old_ids: Vec<_> = old_comments.iter().map(|comment| &comment.id).collect();
            new_comments = comments
                .iter()
                .filter(|comment| !old_ids.contains(&&comment.id))
                .cloned()
                .collect();

            debug!("Found {:?} new comments.", new_comments.len());

            for comment in &new_comments {
                info!("Handling a new comment...");
                if let Err(err) = handler.handle_new_comment(comment).await {
                    error!("Failed to handle new comment: {:?}", err);
                } else {
                    debug!("Successfully handled new comment: {:?}", comment.id);
                }
            }
        }

        if !new_comments.is_empty() {
            cache.put("prop_lot:comments", &comments).await;
            info!("Updated comments in cache");
        }
    } else {
        warn!("Failed to fetch comments");
    }

    debug!("Start function finished.");

    Ok(())
}
