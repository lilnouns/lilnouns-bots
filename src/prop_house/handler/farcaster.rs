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
  prop_house::{
    fetcher::{Auction, Proposal, Vote},
    handler::Handler,
  },
  utils::{get_domain_name, get_short_address},
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

  pub fn from(env: &Env) -> Result<FarcasterHandler> {
    let base_url = env.var("PROP_HOUSE_BASE_URL")?.to_string();
    let bearer_token = env.secret("PROP_HOUSE_WARP_CAST_TOKEN")?.to_string();

    let cache = Cache::from(env);
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
  async fn handle_new_auction(&self, auction: &Auction) -> Result<()> {
    info!("Handling new auction: {}", auction.title);

    let url = format!(
      "{}/{}",
      self.base_url,
      auction.title.replace(' ', "-").to_lowercase()
    );
    let description = format!(
      "A new Prop House round has been created: “{}”",
      auction.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        // "channelKey": "lil-nouns"
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }

  async fn handle_new_proposal(&self, proposal: &Proposal) -> Result<()> {
    info!("Handling new proposal: {}", proposal.title);

    let auctions = self
      .cache
      .get::<Vec<Auction>>("prop_house:auctions")
      .await?
      .unwrap();

    let auction = auctions
      .iter()
      .find(|&a| a.id == proposal.auction_id)
      .unwrap()
      .clone();

    let url = format!(
      "{}/{}/{}",
      self.base_url,
      auction.title.replace(' ', "-").to_lowercase(),
      proposal.id
    );
    let wallet = get_domain_name(&proposal.address)
      .await
      .unwrap_or(get_short_address(&proposal.address));
    let description = format!(
      "{} created a new proposal on Prop House: “{}”",
      wallet, proposal.title
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        // "channelKey": "lil-nouns"
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }

  async fn handle_new_vote(&self, vote: &Vote) -> Result<()> {
    info!("Handling new vote from address: {}", vote.address);

    let proposals = self
      .cache
      .get::<Vec<Proposal>>("prop_house:proposals")
      .await?
      .unwrap();

    let proposal = proposals
      .iter()
      .find(|&a| a.id == vote.proposal_id)
      .unwrap()
      .clone();

    let url = format!(
      "{}/{}/{}",
      self.base_url,
      proposal.title.replace(' ', "-").to_lowercase(),
      proposal.id
    );
    let wallet = get_domain_name(&vote.address)
      .await
      .unwrap_or(get_short_address(&vote.address));

    let description = format!(
      "{} has voted “{}” proposal.",
      wallet,
      match vote.direction {
        1 => "for",
        _ => "against",
      }
    );

    let request_data = json!({
        "text": description,
        "embeds": [url],
        // "channelKey": "lil-nouns"
    });

    self.make_http_request(request_data).await?;

    Ok(())
  }
}
