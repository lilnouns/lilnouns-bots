use anyhow::Result;

use crate::cache;
use crate::prop_lot::fetcher::{Comment, Idea, Vote};

const IDEA_CACHE_KEY_PREFIX: &str = "PROP_LOT_IDEA_";

const VOTE_CACHE_KEY_PREFIX: &str = "PROP_LOT_VOTE_";

const COMMENT_CACHE_KEY_PREFIX: &str = "PROP_LOT_COMMENT_";

fn idea_cache_key(id: isize) -> String {
    format!("{}{}", IDEA_CACHE_KEY_PREFIX, id)
}

fn vote_cache_key(id: isize) -> String {
    format!("{}{}", VOTE_CACHE_KEY_PREFIX, id)
}

fn comment_cache_key(id: isize) -> String {
    format!("{}{}", COMMENT_CACHE_KEY_PREFIX, id)
}

pub(crate) fn set_idea_cache(idea: &Idea) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = idea_cache_key(idea.id.try_into().unwrap());
    cache.set(&cache_key, idea)
}

pub(crate) fn get_idea_cache(id: isize) -> Result<Option<Idea>> {
    let cache = &cache::CACHE;
    let cache_key = idea_cache_key(id);
    cache.get::<String, Idea>(&cache_key)
}

pub(crate) fn set_vote_cache(vote: &Vote) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(vote.id.try_into().unwrap());
    cache.set(&cache_key, vote)
}

pub(crate) fn get_vote_cache(id: isize) -> Result<Option<Vote>> {
    let cache = &cache::CACHE;
    let cache_key = vote_cache_key(id);
    cache.get::<String, Vote>(&cache_key)
}

pub(crate) fn set_comment_cache(comment: &Comment) -> Result<()> {
    let cache = &cache::CACHE;
    let cache_key = comment_cache_key(comment.id.try_into().unwrap());
    cache.set(&cache_key, comment)
}

pub(crate) fn get_comment_cache(id: isize) -> Result<Option<Comment>> {
    let cache = &cache::CACHE;
    let cache_key = comment_cache_key(id);
    cache.get::<String, Comment>(&cache_key)
}
