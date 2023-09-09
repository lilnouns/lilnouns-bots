use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use crate::discord_bot::DiscordBot;
use crate::event::Event;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PropStrategy {
    num: Option<i32>,
    chain_id: Option<i32>,
    strategy_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoteStrategy {
    num: Option<i32>,
    chain_id: Option<i32>,
    strategy_name: Option<String>,
    block_tag: Option<i32>,
    contract: Option<String>,
    multiplier: Option<i32>,
}

pub struct PropHouseDiscordBot {}

impl PropHouseDiscordBot {
    pub fn new() -> Self {
        PropHouseDiscordBot {}
    }
}

#[async_trait]
impl DiscordBot for PropHouseDiscordBot {
        // Make an HTTP GET request to the URL
        let url = "https://prod.backend.prop.house/communities/name/lil%20nouns";
        let response = get(url).expect("Failed to send request");

        // Check if the response status is OK (200)
        if response.status().is_success() {
            // Deserialize the JSON response into a Community struct
            let community: Community = serde_json::from_str(&response.text().expect("Failed to read response")).expect("Failed to deserialize JSON");
    async fn prepare(&self) -> Result<Value> {

            // Now you can access the fields of the Community struct
            println!("Community Name: {}", community.name);
            println!("Contract Address: {}", community.contract_address);

            // Construct the URL for the second request
            let url = format!("https://prod.backend.prop.house/auctions/forCommunity/{}", community.id);

            // Make an HTTP GET request to the URL
            let response = get(&url).expect("Failed to send request");

            // Check if the response status is OK (200)
            if response.status().is_success() {
                // Deserialize the JSON response into a vector of Auction structs
                let auctions: Vec<Auction> = serde_json::from_str(&response.text().expect("Failed to read response")).expect("Failed to deserialize JSON");

                // Now you can access the list of auctions and process them as needed
                for auction in auctions {
                    println!("Auction Title: {}", auction.title);
                    println!("Start Time: {}", auction.start_time);
                    // Access other fields as needed
                }
            } else {
                println!("HTTP request failed with status code: {}", response.status());
            }
        } else {
            println!("HTTP request failed with status code: {}", response.status());
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
