use std::path::Path;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::discord_bot::DiscordBot;
use crate::event::Event;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
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
    auctions: Option<Vec<Auction>>,
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
    proposals: Option<Vec<Proposal>>,
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

#[derive(PartialEq, Debug, Deserialize, Serialize)]
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

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignedData {
    signer: String,
    message: String,
    signature: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DomainSeparator {
    name: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageTypes {
    proposal: Option<Vec<MessageType>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageType {
    name: String,
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
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

impl PartialEq for Auction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.proposals == other.proposals
    }
}

pub struct PropHouseDiscordBot {}

impl PropHouseDiscordBot {
    pub fn new() -> Self {
        PropHouseDiscordBot {}
    }
}

#[async_trait]
impl DiscordBot for PropHouseDiscordBot {
    type RawData = Community;

    async fn prepare(&self) -> Result<Self::RawData> {
        let mut community = fetch_community("lil nouns").await?;

        let mut auctions = fetch_auctions(community.id).await?;
        for auction in auctions.iter_mut() {
            auction.proposals = Some(fetch_proposals(auction.id).await?);
        }

        community.auctions = Some(auctions);

        Ok(community)
    }

    async fn process(&self, source: Self::RawData) -> Result<Vec<Event>> {
        // Define the Sled database path as a constant.
        const DATABASE_PATH: &str = "/tmp/lil-nouns-prop-house";
        // Define the key in use as a constant as well.
        const COMMUNITY_DB_KEY: &str = "community";

        // Assert that the path exists
        assert!(
            Path::new(DATABASE_PATH).exists(),
            "Database path does not exist"
        );

        // Initialize a Sled database
        let db = sled::open(DATABASE_PATH).context("Failed to open database")?;

        // Initialize deserialized_data as None
        let mut deserialized_data: Option<Community> = None;

        match db
            .get(COMMUNITY_DB_KEY)
            .context("Failed to get data from Sled")?
        {
            Some(data) => {
                deserialized_data =
                    serde_json::from_slice(&data).context("Failed to deserialize data")?;
            }
            None => println!("Key not found in Sled."),
        }

        if let Some(deserialized) = deserialized_data {
            if let Some(ref deserialized_auctions) = deserialized.auctions {
                if let Some(source_auctions) = source.auctions {
                    // Find new auctions
                    let new_auctions: Vec<_> = source_auctions
                        .iter()
                        .filter(|auction| !deserialized_auctions.contains(auction))
                        .collect();

                    print!("New Auctions: {:?}", new_auctions);

                    // Find changed proposals
                    let changed_proposals: Vec<_> = deserialized_auctions
                        .iter()
                        .enumerate()
                        .filter_map(|(index, deserialized_auction)| {
                            if let Some(source_auction) = source_auctions.get(index) {
                                // Assuming proposals is Vec<Proposal>, if it's other type replace below code accordingly
                                let des_proposals = &deserialized_auction.proposals;
                                let src_proposals = &source_auction.proposals;
                                Some(
                                    des_proposals
                                        .iter()
                                        .zip(src_proposals.iter())
                                        .filter_map(|(des_prop, src_prop)| {
                                            if des_prop != src_prop {
                                                Some((index, des_prop.clone())) // Return index and changed proposal
                                            } else {
                                                None
                                            }
                                        })
                                        .collect::<Vec<_>>(),
                                )
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .collect();

                    println!("Changed Proposals: {:?}", changed_proposals);
                }
            }

            let serialized_data =
                serde_json::to_string(&deserialized).context("Failed to serialize data")?;

            db.insert(COMMUNITY_DB_KEY, serialized_data.as_bytes())
                .context("Failed to insert data into Sled")?;
        }

        let event1 = Event::new(
            "".to_string(),
            Some("New Event".to_string()),
            Some(format!("Event data from: {:?}", "")),
        );

        Ok(vec![event1])
    }

    async fn dispatch(&self, events: Vec<Event>) -> Result<()> {
        for event in events {
            print!("Description: {:?}", event);
        }

        Ok(())
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
        return Err(anyhow!(
            "Failed to fetch Community: {:?}",
            response.status()
        ));
    }

    // Deserialize the JSON response into a Community struct
    let community: Community = response.json().await?;

    // Return the Community
    Ok(community)
}

async fn fetch_auctions(community_id: i32) -> Result<Vec<Auction>> {
    // Define the URL for the API endpoint
    let url = format!(
        "https://prod.backend.prop.house/auctions/forCommunity/{}",
        community_id
    );

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
    let url = format!(
        "https://prod.backend.prop.house/auctions/{}/proposals",
        auction_id
    );

    // Send a GET request to the API
    let response = reqwest::get(&url).await?;

    // Check if the response status is OK (200)
    if !response.status().is_success() {
        // Handle non-successful response (e.g., return an error)
        return Err(anyhow!(
            "Failed to fetch proposals: {:?}",
            response.status()
        ));
    }

    // Parse the JSON response into a Vec<Proposal>
    let proposals: Vec<Proposal> = response.json().await?;

    Ok(proposals)
}
