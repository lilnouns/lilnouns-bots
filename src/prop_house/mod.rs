use log::{debug, error, info, warn};
use worker::{Env, Result};

use fetcher::{Auction, GraphQLFetcher, Proposal, Vote};
use handler::DiscordHandler;

use crate::cache::Cache;

pub mod fetcher;
pub mod handler;

pub struct PropHouse {
    cache: Cache,
    fetcher: GraphQLFetcher,
    handler: DiscordHandler,
}

impl PropHouse {
    pub fn new(cache: Cache, fetcher: GraphQLFetcher, handler: DiscordHandler) -> Self {
        Self {
            cache,
            fetcher,
            handler,
        }
    }

    pub fn from(env: &Env) -> Result<Self> {
        let cache = Cache::from(env);
        let fetcher = GraphQLFetcher::from(env)?;
        let handler = DiscordHandler::from(env)?;

        Ok(Self::new(cache, fetcher, handler))
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
                    if let Err(err) = self.handler.handle_new_auction(auction).await {
                        error!("Failed to handle new auction: {:?}", err);
                    } else {
                        debug!("Successfully handled new auction: {:?}", auction.id);
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
                    if let Err(err) = self.handler.handle_new_proposal(proposal).await {
                        error!("Failed to handle new proposal: {:?}", err);
                    } else {
                        debug!("Successfully handled new proposal: {:?}", proposal.id);
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
                    if let Err(err) = self.handler.handle_new_vote(vote).await {
                        error!("Failed to handle new vote: {:?}", err);
                    } else {
                        debug!("Successfully handled new vote: {:?}", vote.id);
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
