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
    cache.set(idea_cache_key(idea.id), idea)
}

pub(crate) fn set_ideas_cache(ideas: &[Idea]) -> Result<()> {
    let cache = &cache::CACHE;
    let items: Vec<_> = ideas
        .iter()
        .map(|idea| (idea_cache_key(idea.id), idea))
        .collect();
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

pub(crate) fn set_comment_cache(comment: &Comment) -> Result<()> {
    let cache = &cache::CACHE;
    cache.set(comment_cache_key(comment.id), comment)
}

pub(crate) fn set_comments_cache(comments: &[Comment]) -> Result<()> {
    let cache = &cache::CACHE;
    let items: Vec<_> = comments
        .iter()
        .map(|comment| (comment_cache_key(comment.id), comment))
        .collect();
    cache.set_batch(items)
}

pub(crate) fn get_comment_cache(id: isize) -> Result<Option<Comment>> {
    let cache = &cache::CACHE;
    cache.get::<String, Comment>(&comment_cache_key(id))
}
