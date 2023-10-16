use std::collections::HashMap;

use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
  Response,
};
use serde_json::{json, Value};
use utils::link::Link;
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  prop_lot::{handler::Handler, Comment, Idea, Vote},
  utils,
  utils::ens::get_wallet_handle,
};

pub struct FarcasterHandler {
  base_url: String,
  bearer_token: String,
  channel_key: String,
  cache: Cache,
  client: Client,
  pub link: Link,
}

impl FarcasterHandler {
  pub fn new(
    base_url: String,
    bearer_token: String,
    channel_key: String,
    cache: Cache,
    client: Client,
    link: Link,
  ) -> Self {
    Self {
      base_url,
      bearer_token,
      channel_key,
      cache,
      client,
      link,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("PROP_LOT_BASE_URL")?.to_string();
    let bearer_token = env.secret("PROP_LOT_WARP_CAST_TOKEN")?.to_string();
    let channel_key = env.var("PROP_LOT_WARP_CAST_CHANNEL")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();
    let link = Link::new_from_env(&env);

    Ok(Self::new(
      base_url,
      bearer_token,
      channel_key,
      cache,
      client,
      link,
    ))
  }

  async fn make_http_request(&self, request_data: Value) -> Result<Response> {
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

    Ok(response)
  }
}

#[async_trait(? Send)]
impl Handler for FarcasterHandler {
  async fn handle_new_idea(&self, idea: &Idea) -> Result<()> {
    info!("Handling new idea: {}", idea.title);

    let url = &self
      .link
      .generate(format!("{}/idea/{}", self.base_url, idea.id))
      .await
      .unwrap_or_else(|_| format!("{}/idea/{}", self.base_url, idea.id));

    let wallet = get_wallet_handle(&idea.creator_id, "xyz.farcaster").await;

    let description = format!(
      "{} created a new proposal on Prop Lot: “{}”",
      wallet, idea.title
    );

    let request_data = json!({
      "text": description,
      "embeds": [url],
      "channelKey": self.channel_key
    });

    let response = self.make_http_request(request_data).await.map_err(|e| {
      error!("Failed to make HTTP request: {}", e);
      return e;
    })?;

    let response_body = response.text().await.map_err(|e| {
      error!("Failed to get text from response: {}", e);
      Error::from(format!("Failed to get text from response: {}", e))
    })?;

    let parsed_body: serde_json::Result<Value> = serde_json::from_str(&response_body);

    let response_body: Value = match parsed_body {
      Ok(body) => body,
      Err(e) => {
        error!("Failed to parse JSON: {}", e);
        return Err(e.into());
      }
    };

    let cast_hash = response_body["result"]["cast"]["hash"]
      .as_str()
      .unwrap_or_default();

    let idea_id = idea.id;
    let mut ideas_casts = self
      .cache
      .get::<HashMap<isize, String>>("prop_lot:ideas:casts")
      .await?
      .unwrap_or_default();

    ideas_casts.insert(idea_id, cast_hash.to_string());

    self.cache.put("prop_lot:ideas:casts", &ideas_casts).await;

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.voter_id);

    let ideas = self
      .cache
      .get::<Vec<Idea>>("prop_lot:ideas")
      .await?
      .unwrap_or_default();

    let idea = ideas
      .iter()
      .find(|&a| a.id == vote.idea_id)
      .unwrap()
      .clone();

    let ideas_casts = self
      .cache
      .get::<HashMap<isize, String>>("prop_lot:ideas:casts")
      .await?
      .unwrap_or_default();

    let empty_string = String::new();
    let cast_hash = ideas_casts.get(&idea.id).unwrap_or(&empty_string);

    let wallet = get_wallet_handle(&vote.voter_id, "xyz.farcaster").await;

    let description = format!(
      "{} has voted {} “{}” proposal.",
      wallet,
      match vote.direction {
        1 => "for",
        _ => "against",
      },
      idea.title
    );

    let request_data = {
      if cast_hash.is_empty() {
        json!({
          "text": description,
          "channelKey": self.channel_key
        })
      } else {
        json!({
          "text": description,
          "channelKey": self.channel_key,
          "parent": {
              "hash": cast_hash,
          }
        })
      }
    };

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

    let ideas_casts = self
      .cache
      .get::<HashMap<isize, String>>("prop_lot:ideas:casts")
      .await?
      .unwrap_or_default();

    let cast_hash = ideas_casts.get(&idea.id).unwrap();

    let wallet = get_wallet_handle(&comment.author_id, "xyz.farcaster").await;

    let mut description = format!("{} has commented on “{}” proposal.", wallet, idea.title);
    let chars_limit = 320 - 10 - description.len();
    let mut comment_body = comment.clone().body;
    if comment_body.len() > chars_limit {
      comment_body.truncate(chars_limit);
      comment_body.push_str("...");
    }
    description = format!("{}\n\n“{}”", description, comment_body);

    if !cast_hash.is_empty() {
      let request_data = json!({
        "text": description,
        "channelKey": self.channel_key,
        "parent": {
          "hash": cast_hash,
        },
      });

      self.make_http_request(request_data).await?;
    }

    Ok(())
  }
}
