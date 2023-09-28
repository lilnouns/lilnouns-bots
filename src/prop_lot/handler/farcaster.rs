use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
};
use serde_json::{json, Value};
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  prop_lot::{
    fetcher::{Comment, Idea, Vote},
    handler::Handler,
  },
  utils::ens::get_wallet_handle,
};

pub struct FarcasterHandler {
  base_url: String,
  bearer_token: String,
  cache: Cache,
  client: Client,
}

impl FarcasterHandler {
  pub fn new(base_url: String, bearer_token: String, cache: Cache, client: Client) -> Self {
    Self {
      base_url,
      bearer_token,
      cache,
      client,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("PROP_LOT_BASE_URL")?.to_string();
    let bearer_token = env.secret("PROP_LOT_WARP_CAST_TOKEN")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();

    Ok(Self::new(base_url, bearer_token, cache, client))
  }

  async fn make_http_request(&self, request_data: Value) -> Result<()> {
    let url = "https://api.warpcast.com/v2/casts";
    let token = format!("Bearer {}", self.bearer_token);
    let mut headers = HeaderMap::new();

    let parsed_token =
      HeaderValue::from_str(&token).map_err(|_| Error::from("Error while parsing token"))?;

    headers.insert(AUTHORIZATION, parsed_token);
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Send the HTTP POST request
    let response = self
      .client
      .post(url)
      .headers(headers)
      .json(&request_data)
      .send()
      .await
      .map_err(|e| {
        error!("Failed to execute request: {}", e);
        Error::from(format!("Failed to execute request: {}", e))
      })?;

    debug!("Response status: {:?}", response.status());

    Ok(())
  }
}

#[async_trait(? Send)]
impl Handler for FarcasterHandler {
  async fn handle_new_idea(&self, idea: &Idea) -> Result<()> {
    info!("Handling new idea: {}", idea.title);
    let url = format!("{}/idea/{}", self.base_url, idea.id);

    let wallet = get_wallet_handle(&idea.creator_id, "xyz.farcaster").await;

    let description = format!(
      "{} created a new proposal on Prop Lot: “{}”",
      wallet, idea.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": "lil-nouns"
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.voter_id);

    let ideas = self
      .cache
      .get::<Vec<Idea>>("prop_lot:ideas")
      .await?
      .unwrap();

    let idea = ideas
      .iter()
      .find(|&a| a.id == vote.idea_id)
      .unwrap()
      .clone();

    let wallet = get_wallet_handle(&vote.voter_id, "xyz.farcaster").await;

    let url = format!("{}/idea/{}", self.base_url, idea.id);
    let description = format!(
      "“{}” voted {} by {}.",
      idea.title.to_uppercase(),
      match vote.direction {
        1 => "for",
        _ => "against",
      },
      wallet
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": "lil-nouns"
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }

  async fn handle_new_comment(&self, comment: &Comment) -> Result<()> {
    info!("Handling new comment from address: {}", comment.author_id);

    let ideas = self
      .cache
      .get::<Vec<Idea>>("prop_lot:ideas")
      .await?
      .unwrap();

    let idea = ideas
      .iter()
      .find(|&a| a.id == comment.idea_id)
      .unwrap()
      .clone();

    let url = format!("{}/idea/{}", self.base_url, idea.id);

    let wallet = get_wallet_handle(&comment.author_id, "xyz.farcaster").await;

    let mut description = format!("“{}” commented by {}.", idea.title.to_uppercase(), wallet);
    let chars_limit = 320 - 10 - (description.len() + url.len());
    let mut comment_body = comment.clone().body;
    if comment_body.len() > chars_limit {
      comment_body.truncate(chars_limit);
      comment_body.push_str("...");
    }
    description = format!("{}\n\n“{}”", description, comment_body);

    let request_data = json!({
        "text": description,
        "embeds": [url],
        "channelKey": "lil-nouns"
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }
}
