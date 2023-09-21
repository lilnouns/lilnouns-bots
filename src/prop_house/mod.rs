use log::{error, info};
use worker::Result;

use fetcher::{Auction, GraphQLFetcher, Proposal, Vote};
use handler::DiscordHandler;

use crate::cache::Cache;

pub mod fetcher;
pub mod handler;

pub async fn setup(cache: &Cache, fetcher: &GraphQLFetcher) -> Result<()> {
    if let Some(auctions) = fetcher.fetch_auctions().await {
        cache.put("prop_house:auctions", &auctions).await;
    }

    if let Some(proposals) = fetcher.fetch_proposals().await {
        cache.put("prop_house:proposals", &proposals).await;
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        cache.put("prop_house:votes", &votes).await;
    }

    Ok(())
}

pub async fn start(
    cache: &Cache,
    fetcher: &GraphQLFetcher,
    handler: &DiscordHandler,
) -> Result<()> {
    if let Some(auctions) = fetcher.fetch_auctions().await {
        if let Some(old_auctions) = cache.get::<Vec<Auction>>("prop_house:auctions").await? {
            let old_ids: Vec<_> = old_auctions.iter().map(|auction| &auction.id).collect();
            let new_auctions: Vec<_> = auctions
                .iter()
                .filter(|auction| !old_ids.contains(&&auction.id))
                .collect();

            for auction in new_auctions {
                info!("Handle a new auction... ({:?})", auction.id);
                if let Err(err) = handler.handle_new_auction(auction).await {
                    error!("Failed to handle new auction: {:?}", err);
                }
            }
        }
        cache.put("prop_house:auctions", &auctions).await;
    }

    if let Some(proposals) = fetcher.fetch_proposals().await {
        if let Some(old_proposals) = cache.get::<Vec<Proposal>>("prop_house:proposals").await? {
            let old_ids: Vec<_> = old_proposals.iter().map(|proposal| &proposal.id).collect();
            let new_proposals: Vec<_> = proposals
                .iter()
                .filter(|proposal| !old_ids.contains(&&proposal.id))
                .collect();

            for proposal in new_proposals {
                info!("Handle a new proposal... ({:?})", proposal.id);
                if let Err(err) = handler.handle_new_proposal(proposal).await {
                    error!("Failed to handle new proposal: {:?}", err);
                }
            }
        }
        cache.put("prop_lot:proposals", &proposals).await;
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
        cache.put("prop_house:votes", &votes).await;
    }

    Ok(())
}
