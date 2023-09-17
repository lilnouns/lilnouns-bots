use anyhow::Result;

use crate::cache;
use crate::prop_lot::fetcher::{Comment, Idea, Vote};

const IDEA_CACHE_KEY_PREFIX: &[u8] = b"PROP_LOT_IDEA_";

const VOTE_CACHE_KEY_PREFIX: &[u8] = b"PROP_LOT_VOTE_";

const COMMENT_CACHE_KEY_PREFIX: &[u8] = b"PROP_LOT_COMMENT_";

// Build the idea cache key
fn idea_cache_key(id: isize) -> Vec<u8> {
    [IDEA_CACHE_KEY_PREFIX, &id.to_be_bytes()].concat()
}

// Build the vote cache key
fn vote_cache_key(id: isize) -> Vec<u8> {
    [VOTE_CACHE_KEY_PREFIX, &id.to_be_bytes()].concat()
}

// Build the comment cache key
fn comment_cache_key(id: isize) -> Vec<u8> {
    [COMMENT_CACHE_KEY_PREFIX, &id.to_be_bytes()].concat()
}

// Store an idea into the cache. Returns a Result to handle potential errors.
pub async fn set_idea_cache(idea: &Idea) -> Result<()> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = idea_cache_key(idea.id.try_into().unwrap());

    // Ensure serialization is successful
    let idea_json = serde_json::to_string(idea)?;

    // Attempt to set the value in cache and cater for potential error
    cache.set(&cache_key, idea_json.as_bytes())?;

    Ok(())
}

// Attempt to fetch an idea from the cache
pub async fn get_idea_cache(id: isize) -> Option<Idea> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = idea_cache_key(id);
    match cache.get(cache_key).unwrap() {
        Some(bytes) => serde_json::from_slice(&bytes).ok(),
        None => None,
    }
}

// Store a vote into the cache. Returns a Result to handle potential errors.
pub async fn set_vote_cache(vote: &Vote) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(vote.id.try_into().unwrap());

    let vote_json = serde_json::to_string(vote)?;

    cache.set(&cache_key, vote_json.as_bytes())?;

    Ok(())
}

// Attempt to fetch a vote from the cache
pub async fn get_vote_cache(id: isize) -> Option<Vote> {
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(id);
    match cache.get(cache_key).unwrap() {
        Some(bytes) => serde_json::from_slice(&bytes).ok(),
        None => None,
    }
}

// Store a comment into the cache. Returns a Result to handle potential errors.
pub async fn set_comment_cache(comment: &Comment) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = comment_cache_key(comment.id.try_into().unwrap());

    let comment_json = serde_json::to_string(comment)?;

    cache.set(&cache_key, comment_json.as_bytes())?;

    Ok(())
}

// Attempt to fetch a comment from the cache
pub async fn get_comment_cache(id: isize) -> Option<Comment> {
    let cache = &cache::CACHE;
    let cache_key = comment_cache_key(id);
    match cache.get(cache_key).unwrap() {
        Some(bytes) => serde_json::from_slice(&bytes).ok(),
        None => None,
    }
}
