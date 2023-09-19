use anyhow::Result;

use crate::cache;
use crate::prop_house::fetcher::{Auction, Proposal, Vote};

const AUCTION_CACHE_KEY_PREFIX: &str = "PROP_HOUSE_AUCTION_";
const PROPOSAL_CACHE_KEY_PREFIX: &str = "PROP_HOUSE_PROPOSAL_";
const VOTE_CACHE_KEY_PREFIX: &str = "PROP_HOUSE_VOTE_";

fn auction_cache_key(id: isize) -> String {
    format!("{}{}", AUCTION_CACHE_KEY_PREFIX, id)
}

fn proposal_cache_key(id: isize) -> String {
    format!("{}{}", PROPOSAL_CACHE_KEY_PREFIX, id)
}

fn vote_cache_key(id: isize) -> String {
    format!("{}{}", VOTE_CACHE_KEY_PREFIX, id)
}

pub(crate) fn set_auction_cache(auction: &Auction) -> Result<()> {
    let cache = &cache::CACHE;
    cache.set(auction_cache_key(auction.id), auction)
}

pub(crate) fn set_auctions_cache(auctions: &[Auction]) -> Result<()> {
    let cache = &cache::CACHE;
    let items: Vec<_> = auctions
        .iter()
        .map(|auction| (auction_cache_key(auction.id), auction))
        .collect();
    cache.set_batch(items)
}

pub(crate) fn get_auction_cache(id: isize) -> Result<Option<Auction>> {
    let cache = &cache::CACHE;
    cache.get::<String, Auction>(&auction_cache_key(id))
}

pub(crate) fn set_proposal_cache(proposal: &Proposal) -> Result<()> {
    let cache = &cache::CACHE;
    cache.set(proposal_cache_key(proposal.id), proposal)
}

pub(crate) fn set_proposals_cache(proposals: &[Proposal]) -> Result<()> {
    let cache = &cache::CACHE;
    let items: Vec<_> = proposals
        .iter()
        .map(|proposal| (proposal_cache_key(proposal.id), proposal))
        .collect();
    cache.set_batch(items)
}

pub(crate) fn get_proposal_cache(id: isize) -> Result<Option<Proposal>> {
    let cache = &cache::CACHE;
    cache.get::<String, Proposal>(&proposal_cache_key(id))
}

pub(crate) fn set_vote_cache(vote: &Vote) -> Result<()> {
    let cache = &cache::CACHE;
    cache.set(vote_cache_key(vote.id), vote)
}

pub(crate) fn set_votes_cache(votes: &[Vote]) -> Result<()> {
    let cache = &cache::CACHE;
    let items: Vec<_> = votes
        .iter()
        .map(|vote| (vote_cache_key(vote.id), vote))
        .collect();
    cache.set_batch(items)
}

pub(crate) fn get_vote_cache(id: isize) -> Result<Option<Vote>> {
    let cache = &cache::CACHE;
    cache.get::<String, Vote>(&vote_cache_key(id))
}
