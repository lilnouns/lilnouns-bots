use fetcher::GraphQLFetcher;
use handler::{discord::DiscordHandler, farcaster::FarcasterHandler, Handler};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use worker::{Env, Result};

use crate::cache::Cache;

pub mod fetcher;
pub mod handler;

#[derive(Serialize, Deserialize, Clone)]
pub struct Auction {
  pub id: isize,
  pub title: String,
  pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Proposal {
  pub id: isize,
  pub title: String,
  pub tldr: String,
  pub address: String,
  pub auction_id: isize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
  pub id: isize,
  pub address: String,
  pub auction_id: isize,
  pub proposal_id: isize,
  pub direction: isize,
}

pub struct PropHouse {
  cache: Cache,
  fetcher: GraphQLFetcher,
  handlers: Vec<Box<dyn Handler>>,
}

impl PropHouse {
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

    if env.var("PROP_HOUSE_DISCORD_ENABLED")?.to_string() == "true" {
      let discord_handler: Box<dyn Handler> = Box::new(DiscordHandler::new_from_env(env)?);
      handlers.push(discord_handler);
    }

    if env.var("PROP_HOUSE_FARCASTER_ENABLED")?.to_string() == "true" {
      let farcaster_handler: Box<dyn Handler> = Box::new(FarcasterHandler::new_from_env(env)?);
      handlers.push(farcaster_handler);
    }

    Ok(Self::new(cache, fetcher, handlers))
  }

  pub async fn setup(&self) {
    debug!("Setup function started.");

    if !self.cache.has("prop_house:auctions").await {
      if let Some(auctions) = self.fetcher.fetch_auctions().await {
        info!("Fetched {:?} auctions.", auctions.len());
        debug!("Putting fetched auctions into cache.");
        self.cache.put("prop_house:auctions", &auctions).await;
      } else {
        warn!("Failed to fetch auctions");
      }
    };

    if !self.cache.has("prop_house:proposals").await {
      if let Some(proposals) = self.fetcher.fetch_proposals().await {
        info!("Fetched {:?} proposals.", proposals.len());
        debug!("Putting fetched proposals into cache.");
        self.cache.put("prop_house:proposals", &proposals).await;
      } else {
        warn!("Failed to fetch proposals");
      }
    };

    if !self.cache.has("prop_house:votes").await {
      if let Some(votes) = self.fetcher.fetch_votes().await {
        info!("Fetched {:?} votes.", votes.len());
        debug!("Putting fetched votes into cache.");
        self.cache.put("prop_house:votes", &votes).await;
      } else {
        warn!("Failed to fetch votes");
      }
    };

    debug!("Setup function finished.");
  }

  pub async fn start(&self) -> Result<()> {
    self.setup().await;

    debug!("Start function started.");

    if let Some(auctions) = self.fetcher.fetch_auctions().await {
      debug!("Fetched {:?} auctions.", auctions.len());

      let mut new_auctions = Vec::new();

      if let Some(old_auctions) = self
        .cache
        .get::<Vec<Auction>>("prop_house:auctions")
        .await?
      {
        let old_ids: Vec<_> = old_auctions.iter().map(|auction| &auction.id).collect();
        new_auctions = auctions
          .iter()
          .filter(|auction| !old_ids.contains(&&auction.id))
          .cloned()
          .collect();

        debug!("Found {:?} new auctions.", new_auctions.len());

        for auction in &new_auctions {
          info!("Handling a new auction...");
          for handler in &self.handlers {
            if let Err(err) = handler.handle_new_auction(auction).await {
              error!("Failed to handle new auction: {:?}", err);
            } else {
              debug!("Successfully handled new auction: {:?}", auction.id);
            }
          }
        }
      }

      if !new_auctions.is_empty() {
        self.cache.put("prop_house:auctions", &auctions).await;
        info!("Updated auctions in cache");
      }
    } else {
      warn!("Failed to fetch auctions");
    }

    if let Some(proposals) = self.fetcher.fetch_proposals().await {
      debug!("Fetched {:?} proposals.", proposals.len());

      let mut new_proposals = Vec::new();

      if let Some(old_proposals) = self
        .cache
        .get::<Vec<Proposal>>("prop_house:proposals")
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
        self.cache.put("prop_house:proposals", &proposals).await;
        info!("Updated proposals in cache");
      }
    } else {
      warn!("Failed to fetch proposals");
    }

    if let Some(votes) = self.fetcher.fetch_votes().await {
      debug!("Fetched {:?} votes.", votes.len());

      let mut new_votes = Vec::new();

      if let Some(old_votes) = self.cache.get::<Vec<Vote>>("prop_house:votes").await? {
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
        self.cache.put("prop_house:votes", &votes).await;
        info!("Updated votes in cache");
      }
    } else {
      warn!("Failed to fetch votes");
    }

    debug!("Start function finished.");

    Ok(())
  }
}
