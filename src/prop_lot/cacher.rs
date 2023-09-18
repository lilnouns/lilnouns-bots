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
    let cache_key = idea_cache_key(idea.id);
    cache.set(cache_key, idea)
}

pub(crate) fn set_ideas_cache(ideas: &Vec<Idea>) -> Result<()> {
    let cache = &cache::CACHE;
    let mut items = Vec::new();
    for idea in ideas {
        items.push((idea_cache_key(idea.id), idea))
    }
    cache.set_batch(items)
}

pub(crate) fn get_idea_cache(id: isize) -> Result<Option<Idea>> {
    let cache = &cache::CACHE;
    cache.get::<String, Idea>(&idea_cache_key(id))
}

pub(crate) fn set_vote_cache(vote: &Vote) -> Result<()> {
    let cache = &cache::CACHE;
    cache.set(vote_cache_key(vote.id), vote)
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
    cache.get::<String, Vote>(&vote_cache_key(id))
}

pub(crate) fn set_comment_cache(comment: &Comment) -> Result<()> {
    let cache = &cache::CACHE;
    cache.set(comment_cache_key(comment.id), comment)
}

pub(crate) fn set_comments_cache(comments: &Vec<Comment>) -> Result<()> {
    let cache = &cache::CACHE;
    let mut items = Vec::new();
    for comment in comments {
        items.push((comment_cache_key(comment.id), comment))
    }
    cache.set_batch(items)
}

pub(crate) fn get_comment_cache(id: isize) -> Result<Option<Comment>> {
    let cache = &cache::CACHE;
    let cache_key = comment_cache_key(id);
    cache.get::<String, Comment>(&cache_key)
}
