use log::{error, info};

use fetcher::fetch_auctions;

use crate::prop_house::cacher::{
    get_auction_cache, get_proposal_cache, get_vote_cache, set_auctions_cache, set_proposals_cache,
    set_votes_cache,
};
use crate::prop_house::fetcher::{fetch_proposals, fetch_votes};
use crate::prop_house::handler::DiscordHandler;

mod cacher;
mod fetcher;
mod handler;

pub async fn setup() {
    if let Some(auctions) = fetch_auctions().await {
        set_auctions_cache(&auctions).unwrap();
    }

    if let Some(proposals) = fetch_proposals().await {
        set_proposals_cache(&proposals).unwrap();
    }

    if let Some(votes) = fetch_votes().await {
        set_votes_cache(&votes).unwrap();
    }
}

pub async fn start() {
    let handler = DiscordHandler::new()
        .await
        .expect("Could not create a new DiscordHandler");

    if let Some(auctions) = fetch_auctions().await {
        for auction in auctions {
            if let Ok(cached_auction) = get_auction_cache(auction.id) {
                if cached_auction.is_none() {
                    info!("Handle a new auction... ({:?})", auction.id);
                    if let Err(err) = handler.handle_new_auction(&auction).await {
                        error!("Failed to handle new auction: {:?}", err);
                    }
                }
            }
        }
    }

    if let Some(proposals) = fetch_proposals().await {
        for proposal in proposals {
            if let Ok(cached_proposal) = get_proposal_cache(proposal.id) {
                if cached_proposal.is_none() {
                    info!("Handle a new proposal... ({:?})", proposal.id);
                    if let Err(err) = handler.handle_new_proposal(&proposal).await {
                        error!("Failed to handle new proposal: {:?}", err);
                    }
                }
            }
        }
    }

    if let Some(votes) = fetch_votes().await {
        for vote in votes {
            if let Ok(cached_vote) = get_vote_cache(vote.id) {
                if cached_vote.is_none() {
                    info!("Handle a new vote... ({:?})", vote.id);
                    if let Err(err) = handler.handle_new_vote(&vote).await {
                        error!("Failed to handle new vote: {:?}", err);
                    }
                }
            }
        }
    }
}
