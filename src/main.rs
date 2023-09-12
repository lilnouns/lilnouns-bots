use std::env;

use anyhow::{anyhow, Context, Result};
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_lot_schema.graphql",
    query_path = "graphql/queries/prop_lot_query.graphql",
    request_derives = "Debug",
    response_derives = "Debug",
    variables_derives = "Debug",
    deprecated = "warn"
)]
struct PropLot;

type Date = String;
type Idea = prop_lot::PropLotIdeas;

#[tokio::main]
async fn main() -> Result<()> {
    let ideas = fetch_prop_lot_ideas().await?;

    let ideas_ids: Vec<String> = ideas.iter().map(|i| i.id.to_string()).collect();
    let joined_ids = ideas_ids.join(",");

    println!("All ideas ids({})", joined_ids);

    Ok(())
}

async fn fetch_prop_lot_ideas() -> Result<Vec<Idea>> {
    let url = match env::var("PROP_LOT_GRAPHQL_URL") {
        Ok(val) => val,
        Err(_) => return Err(anyhow!("PROP_LOT_GRAPHQL_URL is not set in env")),
    };

    let variables = prop_lot::Variables {
        options: prop_lot::IdeaInputOptions {
            idea_id: None,
            sort: Some(prop_lot::SORT_TYPE::LATEST),
        },
    };

    let client = Client::new();

    let response = post_graphql::<PropLot, _>(&client, url.to_string(), variables)
        .await
        .context("Failed to execute GraphQL request")?;

    let ideas = match response.data {
        Some(data) => data
            .ideas
            .ok_or(anyhow!("Ideas not found in the response data"))?,
        None => return Err(anyhow!("Response data is unavailable")),
    };

    Ok(ideas)
}
