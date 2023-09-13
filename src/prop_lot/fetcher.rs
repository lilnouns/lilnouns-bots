use std::env;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/prop_lot_schema.graphql",
    query_path = "graphql/queries/prop_lot_query.graphql",
    request_derives = "Debug",
    response_derives = "Debug",
    variables_derives = "Debug",
    deprecated = "warn"
)]
struct Query;

type Date = String;
type Idea = query::QueryIdeas;

pub async fn fetch_ideas() -> Result<Vec<Idea>> {
    let url = match env::var("PROP_LOT_GRAPHQL_URL") {
        Ok(val) => val,
        Err(_) => return Err(anyhow!("PROP_LOT_GRAPHQL_URL is not set in env")),
    };

    let variables = query::Variables {
        options: query::IdeaInputOptions {
            idea_id: None,
            sort: Some(query::SORT_TYPE::LATEST),
        },
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let response = post_graphql::<Query, _>(&client, url.to_string(), variables)
        .await
        .map_err(|e| {
            if e.is_timeout() {
                anyhow!("Request timeout - Please check your network connection and try again")
            } else {
                anyhow!("Failed to execute GraphQL request: {}", e)
            }
        })?;

    let ideas = match response.data {
        Some(data) => data
            .ideas
            .ok_or(anyhow!("Ideas not found in the response data"))?,
        None => return Err(anyhow!("Response data is unavailable")),
    };

    Ok(ideas)
}
