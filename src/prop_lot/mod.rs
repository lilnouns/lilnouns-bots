use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use worker::{Env, Result};

use crate::{
  cache::Cache,
  prop_lot::{
    fetcher::GraphQLFetcher,
    handler::{discord::DiscordHandler, farcaster::FarcasterHandler, Handler},
  },
};

pub(crate) mod fetcher;
pub(crate) mod handler;

#[derive(Serialize, Deserialize, Clone)]
pub struct Idea {
  pub id: isize,
  pub title: String,
  pub tldr: String,
  pub creator_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
  pub id: isize,
  pub voter_id: String,
  pub idea_id: isize,
  pub direction: isize,
  pub voter_weight: isize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
  pub id: isize,
  pub idea_id: isize,
  pub author_id: String,
  pub body: String,
}

pub struct PropLot {
  cache: Cache,
  fetcher: GraphQLFetcher,
  handlers: Vec<Box<dyn Handler>>,
}

impl PropLot {
  pub fn new(cache: Cache, fetcher: GraphQLFetcher, handlers: Vec<Box<dyn Handler>>) -> Self {
    Self {
      cache,
      fetcher,
      handlers,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<Self> {
    let cache = Cache::new_from_env(env);
    let fetcher = GraphQLFetcher::new_from_env(env)?;
    let mut handlers = vec![];

    if env.var("PROP_LOT_DISCORD_ENABLED")?.to_string() == "true" {
      let discord_handler: Box<dyn Handler> = Box::new(DiscordHandler::new_from_env(env)?);
      handlers.push(discord_handler);
    }

    if env.var("PROP_LOT_FARCASTER_ENABLED")?.to_string() == "true" {
      let farcaster_handler: Box<dyn Handler> = Box::new(FarcasterHandler::new_from_env(env)?);
      handlers.push(farcaster_handler);
    }

    Ok(Self::new(cache, fetcher, handlers))
  }

  pub async fn setup(&self) {
    debug!("Setup function started.");

    if !self.cache.has("prop_lot:ideas").await {
      if let Some(ideas) = self.fetcher.fetch_ideas().await {
        info!("Fetched {:?} idea.", ideas.len());
        debug!("Putting fetched ideas into cache.");
        self.cache.put("prop_lot:ideas", &ideas).await;
      } else {
        warn!("Failed to fetch ideas");
      }
    }

    if !self.cache.has("prop_lot:votes").await {
      if let Some(votes) = self.fetcher.fetch_votes().await {
        info!("Fetched {:?} votes.", votes.len());
        debug!("Putting fetched votes into cache.");
        self.cache.put("prop_lot:votes", &votes).await;
      } else {
        warn!("Failed to fetch votes");
      }
    }

    if !self.cache.has("prop_lot:comments").await {
      if let Some(comments) = self.fetcher.fetch_comments().await {
        info!("Fetched {:?} comments.", comments.len());
        debug!("Putting fetched comments into cache.");
        self.cache.put("prop_lot:comments", &comments).await;
      } else {
        warn!("Failed to fetch comments");
      }
    }

    debug!("Setup function finished.");
  }

  pub async fn start(&self) -> Result<()> {
    self.setup().await;

    debug!("Start function started.");

    if let Some(ideas) = self.fetcher.fetch_ideas().await {
      debug!("Fetched {:?} ideas.", ideas.len());

      let mut new_ideas = Vec::new();

      if let Some(old_ideas) = self.cache.get::<Vec<Idea>>("prop_lot:ideas").await? {
        let old_ids: Vec<_> = old_ideas.iter().map(|idea| &idea.id).collect();
        new_ideas = ideas
          .iter()
          .filter(|idea| !old_ids.contains(&&idea.id))
          .cloned()
          .collect();

        debug!("Found {:?} new ideas.", new_ideas.len());

        for idea in &new_ideas {
          info!("Handle a new idea...");
          for handler in &self.handlers {
            if let Err(err) = handler.handle_new_idea(idea).await {
              error!("Failed to handle new idea: {:?}", err);
            } else {
              debug!("Successfully handled new idea: {:?}", idea.id);
            }
          }
        }
      }

      if !new_ideas.is_empty() {
        self.cache.put("prop_lot:ideas", &ideas).await;
        info!("Updated ideas in cache");
      }
    } else {
      warn!("Failed to fetch ideas");
    }

    if let Some(votes) = self.fetcher.fetch_votes().await {
      debug!("Fetched {:?} votes.", votes.len());

      let mut new_votes = Vec::new();

      if let Some(old_votes) = self.cache.get::<Vec<Vote>>("prop_lot:votes").await? {
        let old_ids: Vec<_> = old_votes.iter().map(|vote| &vote.id).collect();
        new_votes = votes
          .iter()
          .filter(|vote| !old_ids.contains(&&vote.id))
          .cloned()
          .collect();

        debug!("Found {:?} new votes.", new_votes.len());

        for vote in &new_votes {
          info!("Handling a new vote...");
          for handler in &self.handlers {
            if let Err(err) = handler.handle_new_vote(vote).await {
              error!("Failed to handle new vote: {:?}", err);
            } else {
              debug!("Successfully handled new vote: {:?}", vote.id);
            }
          }
        }
      }

      if !new_votes.is_empty() {
        self.cache.put("prop_lot:votes", &votes).await;
        info!("Updated votes in cache");
      }
    } else {
      warn!("Failed to fetch votes");
    }

    if let Some(comments) = self.fetcher.fetch_comments().await {
      debug!("Fetched {:?} comments.", comments.len());

      let mut new_comments = Vec::new();

      if let Some(old_comments) = self.cache.get::<Vec<Comment>>("prop_lot:comments").await? {
        let old_ids: Vec<_> = old_comments.iter().map(|comment| &comment.id).collect();
        new_comments = comments
          .iter()
          .filter(|comment| !old_ids.contains(&&comment.id))
          .cloned()
          .collect();

        debug!("Found {:?} new comments.", new_comments.len());

        for comment in &new_comments {
          info!("Handling a new comment...");
          for handler in &self.handlers {
            if let Err(err) = handler.handle_new_comment(comment).await {
              error!("Failed to handle new comment: {:?}", err);
            } else {
              debug!("Successfully handled new comment: {:?}", comment.id);
            }
          }
        }
      }

      if !new_comments.is_empty() {
        self.cache.put("prop_lot:comments", &comments).await;
        info!("Updated comments in cache");
      }
    } else {
      warn!("Failed to fetch comments");
    }

    debug!("Start function finished.");

    Ok(())
  }
}
