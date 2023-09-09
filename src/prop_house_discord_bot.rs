use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::discord_bot::DiscordBot;
use crate::event::Event;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Community {
    id: i32,
    visible: bool,
    contract_address: String,
    name: String,
    profile_image_url: String,
    created_date: String,
    last_updated_date: Option<String>,
    description: String,
    num_auctions: String,
    eth_funded: String,
    total_funded: String,
    num_proposals: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Auction {
    id: i32,
    visible: bool,
    title: String,
    start_time: String,
    proposal_end_time: String,
    voting_end_time: String,
    funding_amount: String,
    created_date: String,
    last_updated_date: Option<String>,
    num_winners: i32,
    community_id: i32,
    currency_type: String,
    description: String,
    balance_block_tag: i32,
    prop_strategy: PropStrategy,
    vote_strategy: VoteStrategy,
    display_comments: bool,
    prop_strategy_description: Option<String>,
    vote_strategy_description: Option<String>,
    num_proposals: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PropStrategy {
    num: Option<i32>,
    chain_id: Option<i32>,
    strategy_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VoteStrategy {
    num: Option<i32>,
    chain_id: Option<i32>,
    strategy_name: Option<String>,
    block_tag: Option<i32>,
    contract: Option<String>,
    multiplier: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Proposal {
    address: String,
    signature_state: String,
    signed_data: SignedData,
    domain_separator: Option<DomainSeparator>,
    message_types: Option<MessageTypes>,
    id: i32,
    visible: bool,
    title: String,
    what: String,
    tldr: String,
    auction_id: i32,
    vote_count_for: i32,
    vote_count_against: i32,
    created_date: String,
    last_updated_date: Option<String>,
    deleted_at: Option<String>,
    req_amount: Option<String>,
    parent_type: String,
    votes: Vec<Vote>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignedData {
    signer: String,
    message: String,
    signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DomainSeparator {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageTypes {
    proposal: Option<Vec<MessageType>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageType {
    name: String,
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Vote {
    address: String,
    signature_state: String,
    signed_data: SignedData,
    domain_separator: Option<DomainSeparator>,
    message_types: Option<MessageTypes>,
    id: i32,
    direction: i32,
    created_date: String,
    proposal_id: i32,
    auction_id: i32,
    weight: i32,
    block_height: Option<i32>,
}


pub struct PropHouseDiscordBot {}

impl PropHouseDiscordBot {
    pub fn new() -> Self {
        PropHouseDiscordBot {}
    }
}

#[async_trait]
impl DiscordBot for PropHouseDiscordBot {
    async fn prepare(&self) -> Result<Value> {
        // Initialize the Sled database.
        let db = sled::open("/tmp/lil-nouns-prop-house")?;

        let community = fetch_community("lil nouns").await?;
        db.insert("community".as_bytes(), serde_json::to_vec(&community)?)?;

        let auctions = fetch_auctions(community.id).await?;
        for auction in auctions {
            db.insert(format!("auction_{}", auction.id).as_bytes(), serde_json::to_vec(&auction)?)?;

            let proposals = fetch_proposals(auction.id).await?;
            for proposal in proposals {
                db.insert(format!("proposal_{}", proposal.id).as_bytes(), serde_json::to_vec(&proposal)?)?;
            }
        }

        Ok(Value::default()) // Replace with actual logic
    }

    async fn process(&self, value: &Value) -> Result<Vec<Event>> {
        let event1 = Event::new(
            "".to_string(),
            Some("New Event".to_string()),
            Some(format!("Event data from: {}", value)),
        );

        let event2 = Event::new(
            "".to_string(),
            Some("Another Event".to_string()),
            Some(format!("Another event data from: {}", value)), );

        Ok(vec![event1, event2])
    }

    async fn dispatch(&self, event: &[Event]) -> Result<()> {
        // TODO: Add the logic to dispatch the event
        Ok(()) // Replace with actual logic
    }
}

async fn fetch_community(name: &str) -> Result<Community> {
    // Define the URL for the API endpoint
    let url = format!("https://prod.backend.prop.house/communities/name/{}", name);

    // Send a GET request to the API
    let response = reqwest::get(&url).await?;

    // Check if the response status is OK (200)
    if !response.status().is_success() {
        // Handle non-successful response (e.g., return an error)
        return Err(anyhow!("Failed to fetch Community: {:?}", response.status()));
    }

    // Deserialize the JSON response into a Community struct
    let community: Community = response.json().await?;

    // Return the Community
    Ok(community)
}

async fn fetch_auctions(community_id: i32) -> Result<Vec<Auction>> {
    // Define the URL for the API endpoint
    let url = format!("https://prod.backend.prop.house/auctions/forCommunity/{}", community_id);

    // Send a GET request to the API
    let response = reqwest::get(&url).await?;

    // Check if the response status is OK (200)
    if !response.status().is_success() {
        // Handle non-successful response (e.g., return an error)
        return Err(anyhow!("Failed to fetch auctions: {:?}", response.status()));
    }

    // Deserialize the JSON response into a vector of Auction structs
    let auctions: Vec<Auction> = response.json().await?;

    // Return the vector of auctions
    Ok(auctions)
}

async fn fetch_proposals(auction_id: i32) -> Result<Vec<Proposal>> {
    // Construct the URL with the auction_id
    let url = format!("https://prod.backend.prop.house/auctions/{}/proposals", auction_id);

    // Send a GET request to the API
    let response = reqwest::get(&url).await?;

    // Check if the response status is OK (200)
    if !response.status().is_success() {
        // Handle non-successful response (e.g., return an error)
        return Err(anyhow!("Failed to fetch proposals: {:?}", response.status()));
    }

    // Parse the JSON response into a Vec<Proposal>
    let proposals: Vec<Proposal> = response.json().await?;

    Ok(proposals)
}
