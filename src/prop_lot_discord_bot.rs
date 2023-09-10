use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::cache;
use crate::discord_bot::DiscordBot;
use crate::event::Event;

// Key prefix for caching idea records
static IDEA_CACHE_KEY_PREFIX: &str = "NOUNS_IDEA_";

// Key prefix for caching idea popularity alert sent records
static IDEA_POPULARITY_ALERT_SENT_CACHE_KEY_PREFIX: &str = "NOUNS_IDEA_POPULARITY_ALERT_SENT_";

fn idea_cache_key(id: i32) -> String {
    format!("{}{}", IDEA_CACHE_KEY_PREFIX, id)
}

fn idea_popularity_alert_sent_cache_key(id: i32) -> String {
    format!("{}{}", IDEA_POPULARITY_ALERT_SENT_CACHE_KEY_PREFIX, id)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdeaResponse {
    pub status: bool,
    pub message: String,
    pub data: Vec<Idea>,
}

// Define the Idea struct for serialization/deserialization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Idea {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub tldr: String,
    pub description: String,
    pub creator_id: Option<String>,
    pub votecount: i32,
    pub votes: Vec<IdeaVote>,
    pub count: Option<IdeaCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdeaCount {
    pub comments: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdeaVote {
    pub id: i32,
    pub direction: i32,
    pub idea_id: Option<i32>,
    pub voter_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub voter: IdeaVoter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdeaVoter {
    pub id: i32,
    pub wallet: String,
    pub ens: Option<String>,
    pub lilnoun_count: i32,
}

pub trait IIdeaLifecycleHandler {
    fn handle_new_idea(&self, idea: &Idea) -> Result<()>;
    fn handle_popular_idea(&self, idea: &Idea) -> Result<()>;
}

pub struct DiscordIdeaLifecycleHandler {
    discord_clients: Vec<serenity::http::Http>,
}

// impl DiscordIdeaLifecycleHandler {
//     pub fn new(discord_clients: Vec<serenity::http::Http>) -> Self {
//         Self { discord_clients }
//     }
//
//     async fn send_discord_message(
//         &self,
//         idea: &Idea,
//         title: &str,
//         description: &str,
//     ) -> Result<()> {
//         // let embed = Embed {
//         //     title: Some(title.to_string()),
//         //     url: Some(format!("https://lilnouns.wtf/ideas/{}", idea.id)),
//         //     description: Some(description.to_string()),
//         //     timestamp: Some(idea.created_at.to_string()),
//         //     ..Default::default()
//         // };
//
//         // for client in &self.discord_clients {
//         //     let _ = client
//         //         .send_message(idea.discord_channel_id, |m| {
//         //             m.embed(|e| e.0 = embed.clone());
//         //             m
//         //         })
//         //         .await?;
//         // }
//
//         Ok(())
//     }
//
//     pub async fn handle_new_idea(&self, idea: &Idea) -> Result<()> {
//         let title = "New Prop Lot Idea";
//         let description = format!(
//             "A new Prop Lot idea (#{}) has been created: {}",
//             idea.id, idea.title
//         );
//         self.send_discord_message(idea, title, &description).await?;
//         println!("processed discord new idea {}", idea.id);
//         Ok(())
//     }
//
//     pub async fn handle_popular_idea(&self, idea: &Idea) -> Result<()> {
//         let title = "New Popular Idea";
//         let description = format!(
//             "It seems idea #{} ({}) got a lot of attention. Please take a look!",
//             idea.id, idea.title
//         );
//         self.send_discord_message(idea, title, &description).await?;
//         println!("processed discord idea popularity alert {}", idea.id);
//         Ok(())
//     }
// }

// Function to store an idea into the cache
async fn update_idea_cache(idea: &Idea) -> Result<()> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = idea_cache_key(idea.id);
    let idea_json = serde_json::to_string(idea)?;

    println!("{:?}", cache_key);

    // Insert the idea into the sled database
    let _ = cache.put(cache_key.as_bytes(), idea_json.as_bytes());

    Ok(())
}

// Function to fetch an idea from the cache
async fn get_idea_cache(id: i32) -> Option<Idea> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = idea_cache_key(id);

    // Fetch the idea from the sled database
    if let Some(idea_json) = cache.get(cache_key.as_bytes()) {
        let idea: Option<Idea> = serde_json::from_slice(&idea_json).ok();
        idea
    } else {
        None
    }
}

// Function to store an idea popularity notification receipt in the cache
async fn set_idea_popularity_alerted(id: i32) -> sled::Result<()> {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = idea_popularity_alert_sent_cache_key(id);

    // Insert a value into the sled database to indicate popularity alert
    let _ = cache.put(cache_key.as_bytes(), &[1].as_slice());

    Ok(())
}

// Function to determine if a popularity alert has been sent for a specific idea id
async fn has_alerted_of_popularity(id: i32) -> bool {
    // Access the global CACHE instance and use it
    let cache = &cache::CACHE;
    let cache_key = idea_popularity_alert_sent_cache_key(id);

    // Check if the cache key exists in the sled database
    cache.has(cache_key.as_bytes()).unwrap_or(false)
}

// Implement a function to fetch all prop lot ideas
pub async fn get_all_ideas() -> Result<Vec<Idea>> {
    // Define the URL for the API
    let url = "https://lil-noun-api.fly.dev/ideas?sort=OLDEST";

    // Send a GET request to the API
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        // Deserialize the JSON response into an IdeaResponse struct
        let idea_response: IdeaResponse = response.json().await?;

        // Extract the data from the response
        let ideas = idea_response.data;

        // Return the list of ideas
        Ok(ideas)
    } else {
        // Handle the case where the request was not successful
        Err(anyhow::anyhow!(
            "Request failed with status code: {}",
            response.status()
        ))
    }
}

pub async fn setup_prop_lot() -> Result<()> {
    let ideas = get_all_ideas().await?;

    let tasks: Vec<_> = ideas
        .into_iter()
        .map(|i| {
            let i_clone = Arc::new(i);
            tokio::spawn(async move {
                match Arc::try_unwrap(i_clone) {
                    Ok(idea) => update_idea_cache(&idea).await,
                    Err(_) => panic!("More than one reference to the Arc"),
                }
            })
        })
        .collect();

    for task in tasks {
        task.await??;
    }

    Ok(())
}

async fn process_prop_lot_tick() -> Result<()> {
    // let mut _idea_lifecycle_handlers: Vec<Box<dyn IIdeaLifecycleHandler + Send + Sync>> = vec![];

    let ideas = get_all_ideas().await?;

    let idea_ids: HashSet<_> = ideas.iter().map(|idea| idea.id).collect();
    println!("propLotHandler: all ideas ids({:?})", idea_ids);

    // let mut tasks = Vec::new();

    // for idea in &ideas {
    //     // let idea_clone = idea.clone();
    //     // let _idea_ids_clone = idea_ids.clone();
    //
    //     // let _idea_lifecycle_handlers = _idea_lifecycle_handlers.clone();
    //
    //     // tasks.push(tokio::spawn(async move {
    //     //     let cached_idea = get_idea_cache(idea_clone.id).await;
    //     //
    //     //     match cached_idea {
    //     //         Some(_) => {
    //     //             if idea_clone.votecount >= 200
    //     //                 && !has_alerted_of_popularity(idea_clone.id).await
    //     //             {
    //     //                 for handler in &_idea_lifecycle_handlers {
    //     //                     if let Err(err) = handler.handle_popular_idea(&idea_clone) {
    //     //                         eprintln!("Error handling popular idea: {:?}", err);
    //     //                         continue;
    //     //                     }
    //     //                     if let Err(err) = set_idea_popularity_alerted(idea_clone.id).await {
    //     //                         eprintln!("Error setting idea popularity alerted: {:?}", err);
    //     //                         continue;
    //     //                     }
    //     //                 }
    //     //             }
    //     //         }
    //     //         None => {
    //     //             for handler in &_idea_lifecycle_handlers {
    //     //                 if let Err(err) = handler.handle_new_idea(&idea_clone) {
    //     //                     eprintln!("Error handling new idea: {:?}", err);
    //     //                 }
    //     //             }
    //     //         }
    //     //     }
    //     //
    //     //     if let Err(err) = update_idea_cache(&idea_clone).await {
    //     //         eprintln!("Error updating cache for idea {}: {:?}", idea_clone.id, err);
    //     //     }
    //     // }));
    // }

    // for task in tasks {
    //     if let Err(err) = task.await {
    //         eprintln!("Error executing task: {:?}", err);
    //     }
    // }

    Ok(())
}

pub struct PropLotDiscordBot {}

impl PropLotDiscordBot {
    pub(crate) fn new() -> Self {
        PropLotDiscordBot {}
    }
}

#[async_trait]
impl DiscordBot for PropLotDiscordBot {
    type RawData = ();

    async fn prepare(&self) -> Result<Self::RawData> {
        setup_prop_lot().await?;

        Ok(())
    }

    async fn process(&self, source: Self::RawData) -> Result<Vec<Event>> {
        print!("{:?}", source);
        todo!()
    }

    async fn dispatch(&self, _events: Vec<Event>) -> Result<()> {
        // // Get the Discord webhook ID and token from environment variables
        // let webhook_id = env::var("DISCORD_WEBHOOK_ID")?;
        // let webhook_token = env::var("DISCORD_WEBHOOK_TOKEN")?;
        //
        // // Create an HTTP client for Discord
        // let http = Http::new("");
        //
        // // Create a Discord webhook client with the provided webhook ID and token
        // let webhook =
        //     Webhook::from_id_with_token(&http, webhook_id.parse()?, &webhook_token).await?;
        //
        // webhook
        //     .execute(&http, false, |w| {
        //         w.content("hello there").username("Webhook test")
        //     })
        //     .await
        //     .expect("Could not execute webhook.");
        //
        Ok(())
    }
}
