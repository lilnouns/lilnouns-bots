use anyhow::Result;

use crate::cache;
use crate::prop_house::fetcher::{Auction, Proposal, Vote};

const AUCTION_CACHE_KEY_PREFIX: &str = "PROP_HOUSE_AUCTION_";
const PROPOSAL_CACHE_KEY_PREFIX: &str = "PROP_HOUSE_PROPOSAL_";
const VOTE_CACHE_KEY_PREFIX: &str = "PROP_HOUSE_VOTE_";

// Build the auction cache key
fn auction_cache_key(id: isize) -> String {
    format!("{}{}", AUCTION_CACHE_KEY_PREFIX, id)
}

// Build the proposal cache key
fn proposal_cache_key(id: isize) -> String {
    format!("{}{}", PROPOSAL_CACHE_KEY_PREFIX, id)
}

// Build the vote cache key
fn vote_cache_key(id: isize) -> String {
    format!("{}{}", VOTE_CACHE_KEY_PREFIX, id)
}

pub(crate) fn set_auction_cache(auction: &Auction) -> Result<()> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = auction_cache_key(auction.id);
    cache.set(&cache_key, auction)
}

pub(crate) fn get_auction_cache(id: isize) -> Result<Option<Auction>> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = auction_cache_key(id);
    cache.get::<Auction>(&cache_key)
}

// Store a proposal into the cache. Returns a Result to handle potential errors.
pub(crate) fn set_proposal_cache(proposal: &Proposal) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = proposal_cache_key(proposal.id);
    cache.set(&cache_key, proposal)
}

// Attempt to fetch a proposal from the cache
pub(crate) fn get_proposal_cache(id: isize) -> Result<Option<Proposal>> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = proposal_cache_key(id);
    cache.get::<Proposal>(&cache_key)
}

pub(crate) fn set_vote_cache(vote: &Vote) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(vote.id);
    cache.set(&cache_key, vote)
}

// Attempt to fetch a vote from the cache
pub(crate) fn get_vote_cache(id: isize) -> Result<Option<Vote>> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(id);
    cache.get::<Vote>(&cache_key)
}
