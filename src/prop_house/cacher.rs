use anyhow::Result;

use crate::cache;
use crate::prop_house::fetcher::{Auction, Proposal, Vote};

const AUCTION_CACHE_KEY_PREFIX: &[u8] = b"PROP_HOUSE_AUCTION_";

// Build the auction cache key
fn auction_cache_key(id: isize) -> Vec<u8> {
    [AUCTION_CACHE_KEY_PREFIX, &id.to_be_bytes()].concat()
}

// Store an auction into the cache. Returns a Result to handle potential errors.
pub(crate) async fn set_auction_cache(auction: &Auction) -> Result<()> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = auction_cache_key(auction.id);

    // Ensure serialization is successful
    let auction_json = serde_json::to_string(auction)?;

    // Attempt to set the value in cache and cater for potential error
    cache.set(&cache_key, auction_json.as_bytes())?;

    Ok(())
}

// Attempt to fetch an auction from the cache
pub(crate) async fn get_auction_cache(id: isize) -> Option<Auction> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = auction_cache_key(id);
    match cache.get(cache_key).unwrap() {
        Some(bytes) => serde_json::from_slice(&bytes).ok(),
        None => None,
    }
}

const PROPOSAL_CACHE_KEY_PREFIX: &[u8] = b"PROP_HOUSE_PROPOSAL_";

// Build the proposal cache key
fn proposal_cache_key(id: isize) -> Vec<u8> {
    [PROPOSAL_CACHE_KEY_PREFIX, &id.to_be_bytes()].concat()
}

// Store a proposal into the cache. Returns a Result to handle potential errors.
pub(crate) async fn set_proposal_cache(
    proposal: &Proposal,
) -> Result<(), Box<dyn std::error::Error>> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = proposal_cache_key(proposal.id);

    // Ensure serialization is successful
    let proposal_json = serde_json::to_string(proposal)?;

    // Attempt to set the value in cache and cater for potential error
    cache.set(&cache_key, proposal_json.as_bytes())?;

    Ok(())
}

// Attempt to fetch a proposal from the cache
pub(crate) async fn get_proposal_cache(id: isize) -> Option<Proposal> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = proposal_cache_key(id);
    match cache.get(cache_key).unwrap() {
        Some(bytes) => serde_json::from_slice(&bytes).ok(),
        None => None,
    }
}

const VOTE_CACHE_KEY_PREFIX: &[u8] = b"PROP_HOUSE_VOTE_";

// Build the vote cache key
fn vote_cache_key(id: isize) -> Vec<u8> {
    [VOTE_CACHE_KEY_PREFIX, &id.to_be_bytes()].concat()
}

// Store a vote into the cache. Returns a Result to handle potential errors.
pub(crate) async fn set_vote_cache(vote: &Vote) -> Result<(), Box<dyn std::error::Error>> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(vote.id);

    // Ensure serialization is successful
    let vote_json = serde_json::to_string(vote)?;

    // Attempt to set the value in cache and cater for potential error
    cache.set(&cache_key, vote_json.as_bytes())?;

    Ok(())
}

// Attempt to fetch a vote from the cache
pub(crate) async fn get_vote_cache(id: isize) -> Option<Vote> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(id);
    match cache.get(cache_key).unwrap() {
        Some(bytes) => serde_json::from_slice(&bytes).ok(),
        None => None,
    }
}
