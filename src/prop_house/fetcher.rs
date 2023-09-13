use std::env;

use anyhow::{anyhow, Context, Result};
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_house_schema.graphql",
    query_path = "graphql/queries/prop_house_query.graphql",
    request_derives = "Debug",
    response_derives = "Debug",
    variables_derives = "Debug",
    deprecated = "warn"
)]
struct Query;

type DateTime = String;

pub async fn fetch_auctions() -> Result<Vec<query::QueryCommunityAuctions>> {
    let url = match env::var("PROP_HOUSE_GRAPHQL_URL") {
        Ok(val) => val,
        Err(_) => return Err(anyhow!("PROP_HOUSE_GRAPHQL_URL is not set in env")),
    };

    let variables = query::Variables { id: 2 };

    let client = Client::new();

    let response = post_graphql::<Query, _>(&client, url.to_string(), variables)
        .await
        .context("Failed to execute GraphQL request")?;

    let community = match response.data {
        Some(data) => {
            let community = data.community; // directly use it without matching
            community
        }
        None => return Err(anyhow!("Response data is unavailable")),
    };

    Ok(community.auctions)
}
