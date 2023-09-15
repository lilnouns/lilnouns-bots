use anyhow::Result;

use crate::cache;
use crate::prop_house::fetcher::Auction;

const AUCTION_CACHE_KEY_PREFIX: &[u8] = b"PROP_HOUSE_AUCTION_";

// Build the auction cache key
fn auction_cache_key(id: i32) -> Vec<u8> {
    [AUCTION_CACHE_KEY_PREFIX, &id.to_be_bytes()].concat()
}

// Store an auction into the cache. Returns a Result to handle potential errors.
pub async fn set_auction_cache(auction: &Auction) -> Result<()> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = auction_cache_key(auction.id as i32);

    // Ensure serialization is successful
    let auction_json = serde_json::to_string(auction)?;

    // Attempt to set the value in cache and cater for potential error
    cache.set(&*cache_key, auction_json.as_bytes())?;

    Ok(())
}

// Attempt to fetch an auction from the cache
pub async fn get_auction_cache(id: i32) -> Option<Auction> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = auction_cache_key(id);
    match cache.get(cache_key).unwrap() {
        Some(bytes) => serde_json::from_slice(&*bytes).ok(),
        None => None,
    }
}
