use std::collections::HashMap;

use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
  Response,
};
use serde_json::{json, to_string, Value};
use utils::link::Link;
use worker::{Env, Error, Result};

use crate::{
  cache::Cache,
  prop_lot::{handler::Handler, Comment, Idea, Vote},
  utils,
  utils::fname::get_username_by_address,
};

pub(crate) struct FarcasterHandler {
  base_url: String,
  warpcast_url: String,
  warpcast_bearer_token: String,
  warpcast_channel_key: String,
  farquest_api_key: String,
  cache: Cache,
  client: Client,
  link: Link,
}

impl FarcasterHandler {
  pub fn new(
    base_url: String,
    warpcast_url: String,
    warpcast_bearer_token: String,
    warpcast_channel_key: String,
    farquest_api_key: String,
    cache: Cache,
    client: Client,
    link: Link,
  ) -> Self {
    Self {
      base_url,
      warpcast_url,
      warpcast_bearer_token,
      warpcast_channel_key,
      farquest_api_key,
      cache,
      client,
      link,
    }
  }

  pub fn new_from_env(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("PROP_LOT_BASE_URL")?.to_string();
    let warpcast_url = env.var("WARPCAST_API_BASE_URL")?.to_string();
    let warpcast_bearer_token = env.secret("PROP_LOT_WARPCAST_TOKEN")?.to_string();
    let warpcast_channel_key = env.var("PROP_LOT_WARPCAST_CHANNEL")?.to_string();
    let farquest_api_key = env.secret("FARQUEST_API_KEY")?.to_string();

    let cache = Cache::new_from_env(env);
    let client = Client::new();
    let link = Link::new_from_env(&env);

    Ok(Self::new(
      base_url,
      warpcast_url,
      warpcast_bearer_token,
      warpcast_channel_key,
      farquest_api_key,
      cache,
      client,
      link,
    ))
  }

  async fn make_http_request(&self, request_data: Value) -> Result<Response> {
    let url = format!("{}/casts", self.warpcast_url);
    let token = format!("Bearer {}", self.warpcast_bearer_token);
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

    let wallet = get_username_by_address(self.farquest_api_key.as_str(), &idea.creator_id).await;

    let description = format!(
      "{} created a new proposal on Prop Lot: “{}”",
      wallet, idea.title
    );

    let request_data = json!({
      "text": description,
      "embeds": [url],
      "channelKey": self.warpcast_channel_key
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
      .ok_or("Failed to get cast hash")?;
    debug!("Cast hash: {}", cast_hash);

    let mut ideas_casts = self
      .cache
      .get::<HashMap<String, String>>("prop_lot:ideas:casts")
      .await?
      .ok_or("Failed to retrieve ideas casts")?;
    debug!("Ideas casts before insertion: {:?}", ideas_casts);

    ideas_casts.insert(idea.id.to_string(), cast_hash.to_string());
    debug!("Ideas casts after insertion: {:?}", ideas_casts);

    let ideas_casts_as_string = to_string(&ideas_casts)?;
    debug!("Ideas casts as string: {}", ideas_casts_as_string);

    self
      .cache
      .put("prop_lot:ideas:casts", &ideas_casts_as_string)
      .await;
    debug!("Finished putting ideas casts in cache");

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
      .clone()
      .ok_or("Idea not found in the funding list.")?;

    let ideas_casts = self
      .cache
      .get::<HashMap<String, String>>("prop_lot:ideas:casts")
      .await?
      .unwrap_or_default();

    let idea_id = idea.id.to_string();
    let cast_hash = ideas_casts.get(&idea_id).ok_or("Cast hash not found")?;

    let wallet = get_username_by_address(self.farquest_api_key.as_str(), &vote.voter_id).await;

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
          "channelKey": self.warpcast_channel_key
        })
      } else {
        json!({
          "text": description,
          "channelKey": self.warpcast_channel_key,
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
      .clone()
      .ok_or("Idea not found in the funding list.")?;

    let ideas_casts = self
      .cache
      .get::<HashMap<String, String>>("prop_lot:ideas:casts")
      .await?
      .unwrap_or_default();

    let cast_hash = ideas_casts
      .get(&idea.id.to_string())
      .ok_or("Cast hash not found")?;

    let wallet = get_username_by_address(self.farquest_api_key.as_str(), &comment.author_id).await;

    let mut description = format!("{} has commented on “{}” proposal.", wallet, idea.title);
    let chars_limit = 1024 - 10 - description.len();
    let mut comment_body = comment.clone().body.trim().to_string();
    if comment_body.len() > chars_limit {
      comment_body.truncate(chars_limit);
      comment_body.push_str("...");
    }
    description = format!("{}\n\n“{}”", description, comment_body);

    let request_data = json!({
      "text": description,
      "channelKey": self.warpcast_channel_key,
      "parent": {"hash": cast_hash},
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }
}
