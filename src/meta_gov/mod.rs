use log::{debug, error, info, warn};
use worker::Env;
use worker::Result;

use handler::discord::DiscordHandler;
use handler::farcaster::FarcasterHandler;

use crate::cache::Cache;
use crate::meta_gov::fetcher::{GraphQLFetcher, Proposal, Vote};
use crate::meta_gov::handler::Handler;

mod fetcher;
mod handler;

struct MetaGov {
  cache: Cache,
  fetcher: GraphQLFetcher,
  handlers: Vec<Box<dyn Handler>>,
}

impl MetaGov {
  pub fn new(cache: Cache, fetcher: GraphQLFetcher, handlers: Vec<Box<dyn Handler>>) -> Self {
    Self {
      cache,
      fetcher,
      handlers,
    }
  }
  pub fn from(env: &Env) -> Result<Self> {
    let cache = Cache::from(env);
    let fetcher = GraphQLFetcher::from(env)?;
    let mut handlers = vec![];

    let discord_handler: Box<dyn Handler> = Box::new(DiscordHandler::from(env)?);
    let farcaster_handler: Box<dyn Handler> = Box::new(FarcasterHandler::from(env)?);

    handlers.push(discord_handler);
    handlers.push(farcaster_handler);

    Ok(Self::new(cache, fetcher, handlers))
  }

  pub async fn setup(&self) {
    debug!("Setup function started.");

    if !self.cache.has("meta_gov:proposals").await {
      if let Some(proposals) = self.fetcher.fetch_proposals().await {
        info!("Fetched {:?} proposals.", proposals.len());
        debug!("Putting fetched proposals into cache.");
        self.cache.put("meta_gov:proposals", &proposals).await;
      } else {
        warn!("Failed to fetch proposals");
      }
    };

    if !self.cache.has("meta_gov:votes").await {
      if let Some(votes) = self.fetcher.fetch_votes().await {
        info!("Fetched {:?} votes.", votes.len());
        debug!("Putting fetched votes into cache.");
        self.cache.put("meta_gov:votes", &votes).await;
      } else {
        warn!("Failed to fetch votes");
      }
    };

    debug!("Setup function finished.");
  }

  pub async fn start(&self) -> Result<()> {
    self.setup().await;

    debug!("Start function started.");

    if let Some(proposals) = self.fetcher.fetch_proposals().await {
      debug!("Fetched {:?} proposals.", proposals.len());

      let mut new_proposals = Vec::new();

      if let Some(old_proposals) = self
        .cache
        .get::<Vec<Proposal>>("meta_gov:proposals")
        .await?
      {
        let old_ids: Vec<_> = old_proposals.iter().map(|proposal| &proposal.id).collect();
        new_proposals = proposals
          .iter()
          .filter(|proposal| !old_ids.contains(&&proposal.id))
          .cloned()
          .collect();

        debug!("Found {:?} new proposals.", new_proposals.len());

        for proposal in &new_proposals {
          info!("Handling a new proposal... ({:?})", proposal.id);
          for handler in &self.handlers {
            if let Err(err) = handler.handle_new_proposal(proposal).await {
              error!("Failed to handle new proposal: {:?}", err);
            } else {
              debug!("Successfully handled new proposal: {:?}", proposal.id);
            }
          }
        }
      }

      if !new_proposals.is_empty() {
        self.cache.put("meta_gov:proposals", &proposals).await;
        info!("Updated proposals in cache");
      }
    } else {
      warn!("Failed to fetch proposals");
    }

    if let Some(votes) = self.fetcher.fetch_votes().await {
      debug!("Fetched {:?} votes.", votes.len());

      let mut new_votes = Vec::new();

      if let Some(old_votes) = self.cache.get::<Vec<Vote>>("meta_gov:votes").await? {
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
        self.cache.put("meta_gov:votes", &votes).await;
        info!("Updated votes in cache");
      }
    } else {
      warn!("Failed to fetch votes");
    }

    debug!("Start function finished.");

    Ok(())
  }
}
