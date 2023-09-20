use log::{error, info};
use worker::Result;

use fetcher::{Auction, GraphQLFetcher, Proposal, Vote};
use handler::DiscordHandler;

use crate::cache::Cache;

pub mod fetcher;
pub mod handler;

pub async fn setup(cache: &Cache<'_>, fetcher: &GraphQLFetcher<'_>) -> Result<()> {
    if let Some(auctions) = fetcher.fetch_auctions().await {
        for auction in auctions {
            cache
                .put(
                    &format!("{}{}", "PROP_HOUSE_AUCTION_", auction.id),
                    &auction,
                )
                .await;
        }
    }

    if let Some(proposals) = fetcher.fetch_proposals().await {
        for proposal in proposals {
            cache
                .put(
                    &format!("{}{}", "PROP_HOUSE_PROPOSAL_", proposal.id),
                    &proposal,
                )
                .await;
        }
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        for vote in votes {
            cache
                .put(&format!("{}{}", "PROP_HOUSE_VOTE_", vote.id), &vote)
                .await;
        }
    }

    Ok(())
}

pub async fn start(
    cache: &Cache<'_>,
    fetcher: &GraphQLFetcher<'_>,
    handler: &DiscordHandler<'_>,
) -> Result<()> {
    if let Some(auctions) = fetcher.fetch_auctions().await {
        for auction in auctions {
            let cached_auction: Option<Auction> = cache
                .get(&format!("{}{}", "PROP_HOUSE_AUCTION_", auction.id))
                .await?;

            if cached_auction.is_none() {
                info!("Handle a new auction... ({:?})", auction.id);
                if let Err(err) = handler.handle_new_auction(&auction).await {
                    error!("Failed to handle new auction: {:?}", err);
                }
            }
        }
    }

    if let Some(proposals) = fetcher.fetch_proposals().await {
        for proposal in proposals {
            let cached_proposal: Option<Proposal> = cache
                .get(&format!("{}{}", "PROP_HOUSE_PROPOSAL_", proposal.id))
                .await?;

            if cached_proposal.is_none() {
                info!("Handle a new proposal... ({:?})", proposal.id);
                if let Err(err) = handler.handle_new_proposal(&proposal).await {
                    error!("Failed to handle new proposal: {:?}", err);
                }
            }
        }
    }

    if let Some(votes) = fetcher.fetch_votes().await {
        for vote in votes {
            let cached_vote: Option<Vote> = cache
                .get(&format!("{}{}", "PROP_HOUSE_VOTE_", vote.id))
                .await?;

            if cached_vote.is_none() {
                info!("Handle a new vote... ({:?})", vote.id);
                if let Err(err) = handler.handle_new_vote(&vote).await {
                    error!("Failed to handle new vote: {:?}", err);
                }
            }
        }
    }

    Ok(())
}
