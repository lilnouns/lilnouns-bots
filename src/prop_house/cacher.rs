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
    let cache_key = auction_cache_key(auction.id);
    cache.set(&cache_key, auction)
}

pub(crate) fn set_auctions_cache(auctions: &Vec<Auction>) -> Result<()> {
    let cache = &cache::CACHE;
    let mut items = Vec::new();
    for auction in auctions {
        items.push((auction_cache_key(auction.id), auction))
    }
    cache.set_batch(items)
}

pub(crate) fn get_auction_cache(id: isize) -> Result<Option<Auction>> {
    let cache = &cache::CACHE;
    let cache_key = auction_cache_key(id);
    cache.get::<String, Auction>(&cache_key)
}

pub(crate) fn set_proposal_cache(proposal: &Proposal) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = proposal_cache_key(proposal.id);
    cache.set(&cache_key, proposal)
}

pub(crate) fn set_proposals_cache(proposals: &Vec<Proposal>) -> Result<()> {
    let cache = &cache::CACHE;
    let mut items = Vec::new();
    for proposal in proposals {
        items.push((proposal_cache_key(proposal.id), proposal))
    }
    cache.set_batch(items)
}

pub(crate) fn get_proposal_cache(id: isize) -> Result<Option<Proposal>> {
    let cache = &cache::CACHE;
    let cache_key = proposal_cache_key(id);
    cache.get::<String, Proposal>(&cache_key)
}

pub(crate) fn set_vote_cache(vote: &Vote) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(vote.id);
    cache.set(&cache_key, vote)
}

pub(crate) fn set_votes_cache(votes: &Vec<Vote>) -> Result<()> {
    let cache = &cache::CACHE;
    let mut items = Vec::new();
    for vote in votes {
        items.push((vote_cache_key(vote.id), vote))
    }
    cache.set_batch(items)
}

pub(crate) fn get_vote_cache(id: isize) -> Result<Option<Vote>> {
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(id);
    cache.get::<String, Vote>(&cache_key)
}
